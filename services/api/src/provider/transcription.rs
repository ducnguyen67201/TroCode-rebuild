use super::{bounded_body, grants};
use crate::{error::ApiError, hosted::HostedState};
use axum::{
    Extension, Json,
    extract::{Multipart, State},
    http::HeaderMap,
};
use reqwest::multipart::{Form, Part};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

const MAX_WAV_BYTES: usize = 1024 * 1024;
const MAX_PROMPT_BYTES: usize = 2_000;
const MAX_PROVIDER_RESPONSE: usize = 64 * 1024;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscriptResponse {
    sequence: i32,
    text: String,
    languages: Vec<String>,
}

pub async fn transcribe(
    State(state): State<HostedState>,
    Extension(correlation): Extension<Uuid>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Result<Json<TranscriptResponse>, ApiError> {
    let mut file = None;
    let mut sequence = None;
    let mut duration_ms = None;
    let mut final_chunk = None;
    let mut prompt = None;
    let mut languages = Vec::new();
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::provider_invalid(correlation))?
    {
        match field.name().unwrap_or_default() {
            "file" if file.is_none() => {
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|_| ApiError::provider_invalid(correlation))?;
                if bytes.len() > MAX_WAV_BYTES {
                    return Err(ApiError::provider_invalid(correlation));
                }
                file = Some(bytes.to_vec());
            }
            "sequence" if sequence.is_none() => {
                sequence = field
                    .text()
                    .await
                    .ok()
                    .and_then(|value| value.parse::<i32>().ok())
            }
            "durationMs" if duration_ms.is_none() => {
                duration_ms = field
                    .text()
                    .await
                    .ok()
                    .and_then(|value| value.parse::<i64>().ok())
            }
            "final" if final_chunk.is_none() => {
                final_chunk = field
                    .text()
                    .await
                    .ok()
                    .and_then(|value| value.parse::<bool>().ok())
            }
            "prompt" if prompt.is_none() => prompt = field.text().await.ok(),
            "languages[]" if languages.len() < 2 => {
                if let Ok(value) = field.text().await {
                    languages.push(value);
                }
            }
            _ => return Err(ApiError::provider_invalid(correlation)),
        }
    }
    let file = file.ok_or_else(|| ApiError::provider_invalid(correlation))?;
    let sequence = sequence.ok_or_else(|| ApiError::provider_invalid(correlation))?;
    let supplied_duration = duration_ms.ok_or_else(|| ApiError::provider_invalid(correlation))?;
    let _final_chunk = final_chunk.ok_or_else(|| ApiError::provider_invalid(correlation))?;
    if prompt
        .as_ref()
        .is_some_and(|value| value.len() > MAX_PROMPT_BYTES)
        || languages.is_empty()
        || languages
            .iter()
            .any(|value| !matches!(value.as_str(), "en" | "vi"))
    {
        return Err(ApiError::provider_invalid(correlation));
    }
    let actual_duration =
        wav_duration_ms(&file).ok_or_else(|| ApiError::provider_invalid(correlation))?;
    if actual_duration > 3_000 || actual_duration.abs_diff(supplied_duration) > 2 {
        return Err(ApiError::provider_invalid(correlation));
    }
    let token = grants::grant_bearer(&headers, correlation)?;
    grants::consume(
        &state.providers,
        token,
        "voice",
        Some(actual_duration),
        Some(sequence),
        correlation,
    )
    .await?;
    let mut form = Form::new()
        .text("model", state.providers.transcription_model.to_string())
        .text("response_format", "json")
        .part(
            "file",
            Part::bytes(file)
                .file_name(format!("chunk-{sequence}.wav"))
                .mime_str("audio/wav")
                .map_err(|_| ApiError::internal(correlation))?,
        );
    if let Some(prompt) = prompt.filter(|value| !value.is_empty()) {
        form = form.text("prompt", prompt);
    }
    for language in &languages {
        form = form.text("languages[]", language.clone());
    }
    let started = std::time::Instant::now();
    let response = state
        .providers
        .client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .bearer_auth(state.providers.api_key.as_ref())
        .multipart(form)
        .send()
        .await
        .map_err(|_| ApiError::provider_unavailable(correlation))?;
    let status = response.status();
    if response
        .content_length()
        .is_some_and(|length| length > MAX_PROVIDER_RESPONSE as u64)
    {
        return Err(ApiError::provider_unavailable(correlation));
    }
    let bytes = bounded_body(response, MAX_PROVIDER_RESPONSE)
        .await
        .map_err(|_| ApiError::provider_unavailable(correlation))?;
    tracing::info!(event="provider.transcription.completed", correlation_id=%correlation, sequence, audio_ms=actual_duration, response_bytes=bytes.len(), provider_elapsed_ms=started.elapsed().as_millis() as u64, status=status.as_u16());
    if !status.is_success() {
        return Err(ApiError::provider_unavailable(correlation));
    }
    let value: Value =
        serde_json::from_slice(&bytes).map_err(|_| ApiError::provider_unavailable(correlation))?;
    let text = value
        .get("text")
        .and_then(Value::as_str)
        .filter(|text| text.len() <= 2_000)
        .ok_or_else(|| ApiError::provider_unavailable(correlation))?
        .to_owned();
    Ok(Json(TranscriptResponse {
        sequence,
        text,
        languages,
    }))
}

fn wav_duration_ms(bytes: &[u8]) -> Option<i64> {
    if bytes.len() < 44
        || &bytes[0..4] != b"RIFF"
        || &bytes[8..12] != b"WAVE"
        || &bytes[12..16] != b"fmt "
    {
        return None;
    }
    let audio_format = u16::from_le_bytes(bytes[20..22].try_into().ok()?);
    let channels = u16::from_le_bytes(bytes[22..24].try_into().ok()?);
    let sample_rate = u32::from_le_bytes(bytes[24..28].try_into().ok()?);
    let bits = u16::from_le_bytes(bytes[34..36].try_into().ok()?);
    if audio_format != 1 || channels != 1 || bits != 16 || !(8_000..=192_000).contains(&sample_rate)
    {
        return None;
    }
    let mut offset = 12_usize;
    while offset.checked_add(8)? <= bytes.len() {
        let length = u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().ok()?) as usize;
        if &bytes[offset..offset + 4] == b"data" {
            if offset + 8 + length > bytes.len() || !length.is_multiple_of(2) {
                return None;
            }
            return Some((length as i64 / 2) * 1_000 / i64::from(sample_rate));
        }
        offset = offset.checked_add(8 + length + (length % 2))?;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reads_pcm16_mono_duration() {
        let mut wav = Vec::new();
        let samples = vec![0_i16; 16_000];
        {
            let mut cursor = std::io::Cursor::new(&mut wav);
            let mut writer = hound::WavWriter::new(
                &mut cursor,
                hound::WavSpec {
                    channels: 1,
                    sample_rate: 16_000,
                    bits_per_sample: 16,
                    sample_format: hound::SampleFormat::Int,
                },
            )
            .unwrap();
            for sample in samples {
                writer.write_sample(sample).unwrap();
            }
            writer.finalize().unwrap();
        }
        assert_eq!(wav_duration_ms(&wav), Some(1_000));
    }
}
