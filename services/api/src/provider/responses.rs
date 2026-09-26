use super::grants;
use crate::{error::ApiError, hosted::HostedState};
use axum::{Extension, Json, extract::State, http::HeaderMap, response::Response};
use serde_json::Value;
use uuid::Uuid;

pub async fn responses(
    State(state): State<HostedState>,
    Extension(correlation): Extension<Uuid>,
    headers: HeaderMap,
    Json(request): Json<Value>,
) -> Result<Response, ApiError> {
    let request = state.providers.responses.prepare(request).map_err(|_| {
        tracing::warn!(
            event = "provider.responses.rejected",
            correlation_id = %correlation,
            reason = "request_not_object"
        );
        ApiError::provider_invalid(correlation)
    })?;
    #[cfg(debug_assertions)]
    {
        let shape = super::responses_relay::request_debug_shape(&request);
        tracing::info!(
            event = "provider.responses.request_shape",
            correlation_id = %correlation,
            request_bytes = shape.bytes,
            input_items = shape.input_items,
            text_inputs = shape.text_inputs,
            image_inputs = shape.image_inputs,
            objective_chars = shape.objective_chars,
            completed_guidance_items = shape.completed_guidance_items,
            accessibility_targets = shape.accessibility_targets,
            accessibility_complete = ?shape.accessibility_complete,
            tool_names = ?shape.tool_names,
            tool_choice = shape.tool_choice,
        );
    }
    let token = grants::grant_bearer(&headers, correlation)?;
    grants::consume(&state.providers, token, "agent", None, None, correlation).await?;

    let started = std::time::Instant::now();
    let response = state
        .providers
        .responses
        .send(request)
        .await
        .map_err(|_| ApiError::provider_unavailable(correlation))?;
    tracing::info!(
        event = "provider.responses.completed",
        correlation_id = %correlation,
        response_bytes = response.body_len(),
        provider_elapsed_ms = started.elapsed().as_millis() as u64,
        status = response.status().as_u16()
    );
    #[cfg(debug_assertions)]
    {
        let shape = response.debug_shape();
        tracing::info!(
            event = "provider.responses.response_shape",
            correlation_id = %correlation,
            output_items = shape.output_items,
            item_types = ?shape.item_types,
            cursor_tool = shape.cursor_tool.unwrap_or("none"),
            argument_bytes = shape.argument_bytes,
            argument_fields = ?shape.argument_fields,
            target_kind = shape.target_kind,
            arguments_valid = shape.arguments_valid,
            caption_chars = shape.caption_chars,
            target_label_chars = shape.target_label_chars,
            destination_kind = shape.destination_kind,
            has_expected = shape.has_expected,
            direction = shape.direction,
        );
    }
    Ok(response.into_response())
}
