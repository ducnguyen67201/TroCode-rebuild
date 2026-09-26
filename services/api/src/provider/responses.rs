use super::{bounded_body, cursor_tools::valid_cursor_tools, grants};
use crate::{error::ApiError, hosted::HostedState};
use axum::{Extension, Json, extract::State, http::HeaderMap};
use serde_json::Value;
use uuid::Uuid;

const MAX_PROVIDER_RESPONSE: usize = 2 * 1024 * 1024;
const MAX_INPUT_BYTES: usize = 1_500_000;
const MAX_TEXT_CHARS: usize = 100_000;
const MAX_IMAGE_URL_CHARS: usize = 750_000;
const MAX_IMAGES: usize = 8;

pub async fn responses(
    State(state): State<HostedState>,
    Extension(correlation): Extension<Uuid>,
    headers: HeaderMap,
    Json(request): Json<Value>,
) -> Result<Json<Value>, ApiError> {
    validate_request(&request, state.providers.guidance_model.as_ref()).map_err(|reason| {
        tracing::warn!(event="provider.responses.rejected", correlation_id=%correlation, reason);
        ApiError::provider_invalid(correlation)
    })?;
    let token = grants::grant_bearer(&headers, correlation)?;
    grants::consume(&state.providers, token, "agent", None, None, correlation).await?;
    let started = std::time::Instant::now();
    let response = state
        .providers
        .client
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(state.providers.api_key.as_ref())
        .json(&request)
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
    tracing::info!(event="provider.responses.completed", correlation_id=%correlation, response_bytes=bytes.len(), provider_elapsed_ms=started.elapsed().as_millis() as u64, status=status.as_u16());
    if !status.is_success() {
        return Err(ApiError::provider_unavailable(correlation));
    }
    serde_json::from_slice(&bytes)
        .map(Json)
        .map_err(|_| ApiError::provider_unavailable(correlation))
}

fn validate_request(value: &Value, model: &str) -> Result<(), &'static str> {
    let object = value.as_object().ok_or("request_not_object")?;
    let allowed = [
        "model",
        "input",
        "tools",
        "store",
        "stream",
        "background",
        "parallel_tool_calls",
        "max_output_tokens",
        "include",
        "previous_response_id",
        "instructions",
        "reasoning",
        "text",
        "tool_choice",
    ];
    if object.keys().any(|key| !allowed.contains(&key.as_str()))
        || object.get("model") != Some(&Value::String(model.to_owned()))
        || object.get("store") != Some(&Value::Bool(false))
        || object
            .get("stream")
            .is_some_and(|value| value != &Value::Bool(false))
        || object
            .get("background")
            .is_some_and(|value| value != &Value::Bool(false))
        || object
            .get("previous_response_id")
            .is_some_and(|value| !value.is_null())
        || object
            .get("max_output_tokens")
            .and_then(Value::as_u64)
            .is_none_or(|value| value == 0 || value > 1_024)
        || object
            .get("parallel_tool_calls")
            .is_some_and(|value| value != &Value::Bool(false))
        || object
            .get("tool_choice")
            .is_some_and(|value| !matches!(value.as_str(), Some("auto" | "none")))
    {
        return Err("request_contract");
    }
    if !valid_cursor_tools(object.get("tools")) {
        return Err("cursor_tools");
    }
    let input = object.get("input").ok_or("input_missing")?;
    let serialized = serde_json::to_vec(input).map_err(|_| "input_serialization")?;
    if serialized.len() > MAX_INPUT_BYTES {
        return Err("input_too_large");
    }
    let mut images = 0_usize;
    inspect_input(input, &mut images)?;
    if images > MAX_IMAGES {
        return Err("too_many_images");
    }
    Ok(())
}

fn inspect_input(value: &Value, images: &mut usize) -> Result<(), &'static str> {
    match value {
        Value::String(text) => {
            if text.starts_with("data:image/") {
                *images += 1;
                if text.len() > MAX_IMAGE_URL_CHARS {
                    return Err("image_too_large");
                }
            } else if text.len() > MAX_TEXT_CHARS {
                return Err("text_too_large");
            } else if text.starts_with("http://") || text.starts_with("https://") {
                return Err("remote_input_url");
            }
        }
        Value::Array(values) => {
            for value in values {
                inspect_input(value, images)?;
            }
        }
        Value::Object(values) => {
            for value in values.values() {
                inspect_input(value, images)?;
            }
        }
        _ => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn rejects_computer_shell_and_unknown_function_tools() {
        let base = json!({"model":"guidance","input":"show me","store":false,"max_output_tokens":1024,"parallel_tool_calls":false,"tools":crate::provider::cursor_tools::test_cursor_tools(),"tool_choice":"auto"});
        assert!(validate_request(&base, "guidance").is_ok());
        for tools in [
            json!([{"type":"computer"}]),
            json!([{"type":"computer_use_preview"}]),
            json!([{"type":"shell"}]),
            json!([{"type":"function","name":"delete_file"}]),
        ] {
            let mut invalid = base.clone();
            invalid["tools"] = tools;
            assert!(validate_request(&invalid, "guidance").is_err());
        }
        for (key, value) in [
            ("model", json!("other")),
            ("store", json!(true)),
            ("stream", json!(true)),
            ("background", json!(true)),
            ("max_output_tokens", json!(1025)),
            ("parallel_tool_calls", json!(true)),
            ("tool_choice", json!("required")),
        ] {
            let mut invalid = base.clone();
            invalid[key] = value;
            assert!(validate_request(&invalid, "guidance").is_err());
        }
    }

    #[test]
    fn rejects_oversized_or_remote_input_before_upstream_dispatch() {
        let mut request = json!({"model":"guidance","input":"show me","store":false,"max_output_tokens":1024,"parallel_tool_calls":false,"tools":crate::provider::cursor_tools::test_cursor_tools()});
        request["input"] = json!("x".repeat(MAX_TEXT_CHARS + 1));
        assert_eq!(
            validate_request(&request, "guidance"),
            Err("text_too_large")
        );
        request["input"] = json!(format!(
            "data:image/png;base64,{}",
            "A".repeat(MAX_IMAGE_URL_CHARS)
        ));
        assert_eq!(
            validate_request(&request, "guidance"),
            Err("image_too_large")
        );
        request["input"] = json!("x".repeat(MAX_INPUT_BYTES + 1));
        assert!(validate_request(&request, "guidance").is_err());
        request["input"] = json!("https://example.com/private-image");
        assert_eq!(
            validate_request(&request, "guidance"),
            Err("remote_input_url")
        );
    }

    #[test]
    fn accepts_bounded_data_image_above_the_text_limit() {
        let image = format!("data:image/png;base64,{}", "A".repeat(MAX_TEXT_CHARS + 1));
        let request = json!({"model":"guidance","input":[{"role":"user","content":[{"type":"input_image","image_url":image}]}],"store":false,"max_output_tokens":1024,"parallel_tool_calls":false,"tools":crate::provider::cursor_tools::test_cursor_tools()});
        assert!(validate_request(&request, "guidance").is_ok());
    }
}
