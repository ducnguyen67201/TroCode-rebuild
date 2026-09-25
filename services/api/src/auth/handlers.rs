use super::{google::GoogleError, sessions::SessionEnvelope};
use crate::{error::ApiError, hosted::HostedState};
use axum::{
    Extension, Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GoogleExchangeRequest {
    code: String,
    code_verifier: String,
    redirect_uri: String,
    nonce: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RefreshRequest {
    refresh_token: String,
}

pub async fn exchange_google(
    State(state): State<HostedState>,
    Extension(correlation): Extension<Uuid>,
    Json(request): Json<GoogleExchangeRequest>,
) -> Result<Json<SessionEnvelope>, ApiError> {
    let identity = state
        .google
        .exchange(
            &request.code,
            &request.code_verifier,
            &request.redirect_uri,
            &request.nonce,
        )
        .await
        .map_err(|error| match error {
            GoogleError::Invalid => ApiError::invalid_google(correlation),
            GoogleError::Unavailable => ApiError::unavailable(correlation),
        })?;
    let session = state.sessions.login(identity, correlation).await?;
    tracing::info!(
        event = "auth.login.completed",
        correlation_id = %correlation,
        account_id = %session.account.account_id,
        workspace_count = session.workspaces.len(),
    );
    Ok(Json(session))
}

pub async fn refresh_session(
    State(state): State<HostedState>,
    Extension(correlation): Extension<Uuid>,
    Json(request): Json<RefreshRequest>,
) -> Result<Json<SessionEnvelope>, ApiError> {
    if request.refresh_token.len() > 128 {
        return Err(ApiError::invalid_request(correlation));
    }
    Ok(Json(
        state
            .sessions
            .refresh(&request.refresh_token, correlation)
            .await?,
    ))
}

pub async fn logout(
    State(state): State<HostedState>,
    Extension(correlation): Extension<Uuid>,
    headers: HeaderMap,
) -> Result<StatusCode, ApiError> {
    let context = state
        .sessions
        .authenticate(bearer(&headers, correlation)?, correlation)
        .await?;
    state.sessions.logout(context, correlation).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn me(
    State(state): State<HostedState>,
    Extension(correlation): Extension<Uuid>,
    headers: HeaderMap,
) -> Result<Json<super::sessions::MeResponse>, ApiError> {
    let context = state
        .sessions
        .authenticate(bearer(&headers, correlation)?, correlation)
        .await?;
    Ok(Json(
        state.sessions.current_account(context, correlation).await?,
    ))
}

fn bearer(headers: &HeaderMap, correlation: Uuid) -> Result<&str, ApiError> {
    headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|value| !value.is_empty() && value.len() <= 16 * 1024)
        .ok_or_else(|| ApiError::session_expired(correlation))
}
