use super::{bounded_body, grants};
use crate::{error::ApiError, hosted::HostedState};
use axum::{Extension, Json, extract::State, http::HeaderMap};
use serde_json::{Map, Value};
use uuid::Uuid;

const MAX_PROVIDER_RESPONSE: usize = 2 * 1024 * 1024;

pub async fn responses(
    State(state): State<HostedState>,
    Extension(correlation): Extension<Uuid>,
    headers: HeaderMap,
    Json(request): Json<Value>,
) -> Result<Json<Value>, ApiError> {
    if validate_request(&request, state.providers.action_model.as_ref()).is_err() {
        tracing::warn!(event="provider.responses.request_rejected", correlation_id=%correlation);
        return Err(ApiError::provider_invalid(correlation));
    }
    let token = grants::grant_bearer(&headers, correlation)?;
    grants::consume(&state.providers, token, "agent", None, None, correlation).await?;
    let started = std::time::Instant::now();
    tracing::info!(event="provider.responses.started", correlation_id=%correlation, model=%state.providers.action_model);
    let response = state
        .providers
        .client
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(state.providers.api_key.as_ref())
        .json(&request)
        .send()
        .await
        .map_err(|_| {
            tracing::warn!(event="provider.responses.transport_failed", correlation_id=%correlation, provider_elapsed_ms=started.elapsed().as_millis() as u64);
            ApiError::provider_unavailable(correlation)
        })?;
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

fn validate_request(value: &Value, model: &str) -> Result<(), ()> {
    let object = value.as_object().ok_or(())?;
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
            .is_some_and(|value| value > 2_048)
    {
        return Err(());
    }
    let tools = object.get("tools").and_then(Value::as_array).ok_or(())?;
    if tools.len() != 1 || !valid_computer_tool(tools[0].as_object().ok_or(())?) {
        return Err(());
    }
    let input = object.get("input").ok_or(())?;
    let serialized = serde_json::to_vec(input).map_err(|_| ())?;
    if serialized.len() > 1_500_000 {
        return Err(());
    }
    let mut images = 0_usize;
    inspect_input(input, &mut images)?;
    if images > 8 {
        return Err(());
    }
    Ok(())
}

fn valid_computer_tool(tool: &Map<String, Value>) -> bool {
    let allowed = ["type", "display_width", "display_height", "environment"];
    tool.keys().all(|key| allowed.contains(&key.as_str()))
        && tool
            .get("type")
            .and_then(Value::as_str)
            .is_some_and(|value| matches!(value, "computer" | "computer_use_preview"))
        && tool
            .get("display_width")
            .and_then(Value::as_u64)
            .is_none_or(|value| (1..=8_192).contains(&value))
        && tool
            .get("display_height")
            .and_then(Value::as_u64)
            .is_none_or(|value| (1..=8_192).contains(&value))
        && tool
            .get("environment")
            .and_then(Value::as_str)
            .is_none_or(|value| matches!(value, "mac" | "windows"))
}

fn inspect_input(value: &Value, images: &mut usize) -> Result<(), ()> {
    match value {
        Value::String(text) => {
            if text.len() > 100_000 || text.starts_with("http://") || text.starts_with("https://") {
                return Err(());
            }
            if text.starts_with("data:image/") {
                *images += 1;
                if text.len() > 750_000 {
                    return Err(());
                }
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
    fn accepts_only_one_computer_tool() {
        let valid = json!({"model":"action","input":"click settings","tools":[{"type":"computer","display_width":800,"display_height":600,"environment":"mac"}],"store":false});
        assert!(validate_request(&valid, "action").is_ok());
        let mut invalid = valid.clone();
        invalid["tools"] = json!([{"type":"shell"}]);
        assert!(validate_request(&invalid, "action").is_err());
    }
}
