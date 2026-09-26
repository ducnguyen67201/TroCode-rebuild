use super::ProviderService;
use crate::{
    auth::{handlers::bearer, jwt::AuthContext},
    entities::{auth_session, provider_grant},
    error::ApiError,
    hosted::HostedState,
};
use axum::{Extension, Json, extract::State, http::HeaderMap};
use rand::RngCore;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect, Set,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use uuid::Uuid;

const VOICE_REQUESTS: i32 = 32;
const VOICE_AUDIO_MS: i64 = 45_000;
const AGENT_REQUESTS: i32 = 24;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GrantRequest {
    subject_id: Uuid,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GrantResponse {
    grant: String,
    expires_at: String,
    model: String,
}

pub async fn voice_grant(
    State(state): State<HostedState>,
    Extension(correlation): Extension<Uuid>,
    headers: HeaderMap,
    Json(request): Json<GrantRequest>,
) -> Result<Json<GrantResponse>, ApiError> {
    issue(&state, &headers, request.subject_id, "voice", correlation)
        .await
        .map(Json)
}

pub async fn runtime_grant(
    State(state): State<HostedState>,
    Extension(correlation): Extension<Uuid>,
    headers: HeaderMap,
    Json(request): Json<GrantRequest>,
) -> Result<Json<GrantResponse>, ApiError> {
    issue(&state, &headers, request.subject_id, "agent", correlation)
        .await
        .map(Json)
}

async fn issue(
    state: &HostedState,
    headers: &HeaderMap,
    subject_id: Uuid,
    kind: &'static str,
    correlation: Uuid,
) -> Result<GrantResponse, ApiError> {
    let context = state
        .sessions
        .authenticate(bearer(headers, correlation)?, correlation)
        .await?;
    if state
        .sessions
        .current_account(context, correlation)
        .await?
        .workspaces
        .is_empty()
    {
        return Err(ApiError::session_revoked(correlation));
    }
    let mut bytes = [0_u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    let token = bytes
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let digest = Sha256::digest(token.as_bytes()).to_vec();
    let now = OffsetDateTime::now_utc();
    let expires_at = now
        + if kind == "voice" {
            time::Duration::seconds(90)
        } else {
            time::Duration::minutes(5)
        };
    provider_grant::ActiveModel {
        token_digest: Set(digest),
        kind: Set(kind.to_owned()),
        account_id: Set(context.account_id),
        auth_session_id: Set(context.session_id),
        subject_id: Set(subject_id),
        expires_at: Set(expires_at),
        remaining_requests: Set(if kind == "voice" {
            VOICE_REQUESTS
        } else {
            AGENT_REQUESTS
        }),
        remaining_audio_ms: Set((kind == "voice").then_some(VOICE_AUDIO_MS)),
        last_sequence: Set(None),
        revoked: Set(false),
        created_at: Set(now),
    }
    .insert(&state.database)
    .await
    .map_err(|_| ApiError::provider_grant_conflict(correlation))?;
    Ok(GrantResponse {
        grant: token,
        expires_at: expires_at
            .format(&time::format_description::well_known::Rfc3339)
            .map_err(|_| ApiError::internal(correlation))?,
        model: if kind == "voice" {
            state.providers.transcription_model.to_string()
        } else {
            state.providers.action_model.to_string()
        },
    })
}

pub async fn consume(
    service: &ProviderService,
    token: &str,
    kind: &str,
    audio_ms: Option<i64>,
    sequence: Option<i32>,
    correlation: Uuid,
) -> Result<AuthContext, ApiError> {
    if token.len() != 64
        || !token
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(ApiError::provider_unauthorized(correlation));
    }
    let digest = Sha256::digest(token.as_bytes()).to_vec();
    let transaction = service
        .database
        .begin()
        .await
        .map_err(|_| ApiError::internal(correlation))?;
    let grant = provider_grant::Entity::find_by_id(digest)
        .filter(provider_grant::Column::Kind.eq(kind))
        .filter(provider_grant::Column::Revoked.eq(false))
        .filter(provider_grant::Column::RemainingRequests.gt(0))
        .lock_exclusive()
        .one(&transaction)
        .await
        .map_err(|_| ApiError::internal(correlation))?
        .filter(|grant| grant.expires_at > OffsetDateTime::now_utc())
        .ok_or_else(|| ApiError::provider_unauthorized(correlation))?;
    let session_active = auth_session::Entity::find_by_id(grant.auth_session_id)
        .filter(auth_session::Column::AccountId.eq(grant.account_id))
        .filter(auth_session::Column::RevokedAt.is_null())
        .one(&transaction)
        .await
        .map_err(|_| ApiError::internal(correlation))?
        .is_some_and(|session| session.expires_at > OffsetDateTime::now_utc());
    if !session_active {
        return Err(ApiError::provider_unauthorized(correlation));
    }
    if let Some(milliseconds) = audio_ms
        && (milliseconds <= 0
            || grant
                .remaining_audio_ms
                .is_none_or(|remaining| remaining < milliseconds))
    {
        return Err(ApiError::provider_budget(correlation));
    }
    if let Some(next) = sequence
        && (next < 0
            || grant
                .last_sequence
                .is_some_and(|previous| next != previous + 1))
    {
        return Err(ApiError::provider_sequence(correlation));
    }
    let context = AuthContext {
        account_id: grant.account_id,
        session_id: grant.auth_session_id,
    };
    let mut update = grant.into_active_model();
    update.remaining_requests = Set(update.remaining_requests.unwrap() - 1);
    if let Some(milliseconds) = audio_ms {
        update.remaining_audio_ms = Set(update
            .remaining_audio_ms
            .unwrap()
            .map(|remaining| remaining - milliseconds));
    }
    if sequence.is_some() {
        update.last_sequence = Set(sequence);
    }
    update
        .update(&transaction)
        .await
        .map_err(|_| ApiError::internal(correlation))?;
    transaction
        .commit()
        .await
        .map_err(|_| ApiError::internal(correlation))?;
    Ok(context)
}

pub fn grant_bearer(headers: &HeaderMap, correlation: Uuid) -> Result<&str, ApiError> {
    headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or_else(|| ApiError::provider_unauthorized(correlation))
}
