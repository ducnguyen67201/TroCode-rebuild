use crate::{
    auth::{
        PendingWorkspaceMembershipStore, WorkspaceMembershipStore, google::GoogleVerifier,
        handlers, jwt::JwtService, sessions::SessionService,
    },
    config::HostedConfig,
    persistence,
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
}

impl HostedState {
    pub async fn connect(config: &HostedConfig) -> Result<Self, &'static str> {
        let database = persistence::connect(&config.database_url)
            .await
            .map_err(|_| "Hosted database unavailable.")?;
        let memberships: Arc<dyn WorkspaceMembershipStore> =
            Arc::new(PendingWorkspaceMembershipStore);
        Self::from_database(config, database, memberships)
    }

    pub fn from_database(
        config: &HostedConfig,
        database: DatabaseConnection,
        memberships: Arc<dyn WorkspaceMembershipStore>,
    ) -> Result<Self, &'static str> {
        let google = Arc::new(GoogleVerifier::production(config.google_client_id.clone())?);
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
        Ok(Self {
            database,
            google,
            sessions,
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
