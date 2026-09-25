use crate::{
    auth::{
        WorkspaceMembershipStore, google::GoogleVerifier, handlers, jwt::JwtService,
        sessions::SessionService,
    },
    config::HostedConfig,
    persistence,
    provider::{
        ProviderService, grants as provider_grants, responses as provider_responses,
        transcription as provider_transcription,
    },
    workspace::{WorkspaceService, handlers as workspace_handlers},
};
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    http::StatusCode,
    routing::{delete, get, post},
};
use sea_orm::DatabaseConnection;
use serde_json::{Value, json};
use std::sync::Arc;

#[derive(Clone)]
pub struct HostedState {
    pub database: DatabaseConnection,
    pub google: Arc<GoogleVerifier>,
    pub sessions: Arc<SessionService>,
    pub workspaces: Arc<WorkspaceService>,
    pub providers: Arc<ProviderService>,
}

impl HostedState {
    pub async fn connect(config: &HostedConfig) -> Result<Self, &'static str> {
        let database = persistence::connect(&config.database_url)
            .await
            .map_err(|_| "Hosted database unavailable.")?;
        let workspaces = Arc::new(WorkspaceService::new(database.clone()));
        let memberships: Arc<dyn WorkspaceMembershipStore> = workspaces.clone();
        Self::from_database(config, database, memberships, workspaces)
    }

    pub fn from_database(
        config: &HostedConfig,
        database: DatabaseConnection,
        memberships: Arc<dyn WorkspaceMembershipStore>,
        workspaces: Arc<WorkspaceService>,
    ) -> Result<Self, &'static str> {
        let google = Arc::new(GoogleVerifier::production(
            config.google_client_id.clone(),
            config.google_client_secret.clone(),
        )?);
        let jwt = JwtService::new(
            &config.jwt_key,
            config.issuer.clone(),
            config.audience.clone(),
            config.access_ttl,
        )?;
        let sessions = Arc::new(SessionService::new(
            database.clone(),
            jwt,
            config.refresh_key.clone(),
            config.refresh_ttl,
            memberships,
        )?);
        let providers = Arc::new(ProviderService::new(
            database.clone(),
            config.openai_api_key.clone(),
            config.transcription_model.clone(),
            config.action_model.clone(),
        )?);
        Ok(Self {
            database,
            google,
            sessions,
            workspaces,
            providers,
        })
    }
}

pub fn router(state: HostedState) -> Router {
    Router::new()
        .route(
            "/healthz",
            get(|| async { Json(json!({"status":"alive"})) }),
        )
        .route("/readyz", get(readiness))
        .route("/v1/auth/google/exchange", post(handlers::exchange_google))
        .route("/v1/auth/session/refresh", post(handlers::refresh_session))
        .route("/v1/auth/session", delete(handlers::logout))
        .route("/v1/me", get(handlers::me))
        .route("/v1/voice-grants", post(provider_grants::voice_grant))
        .route("/v1/runtime-grants", post(provider_grants::runtime_grant))
        .route(
            "/v1/audio/transcriptions",
            post(provider_transcription::transcribe).layer(DefaultBodyLimit::max(1024 * 1024)),
        )
        .route(
            "/v1/responses",
            post(provider_responses::responses).layer(DefaultBodyLimit::max(2 * 1024 * 1024)),
        )
        .route(
            "/v1/workspaces/{workspace_id}/members",
            get(workspace_handlers::list_members).post(workspace_handlers::add_member),
        )
        .route(
            "/v1/workspaces/{workspace_id}/members/{membership_id}",
            delete(workspace_handlers::remove_member),
        )
        .layer(DefaultBodyLimit::max(16 * 1024))
        .with_state(state)
        .layer(axum::middleware::from_fn(crate::correlation))
}

async fn readiness(State(state): State<HostedState>) -> (StatusCode, Json<Value>) {
    let database = state.database.ping().await.is_ok();
    (
        if database {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        },
        Json(json!({"database": database})),
    )
}
