pub mod auth;
pub mod config;
pub mod db;
pub mod entities;
pub mod error;
pub mod hosted;
pub mod model_gateway;
pub mod persistence;
pub mod proof_admin;
pub mod storage;
pub mod workspace;
use axum::{
    Extension, Json, Router,
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
};
use object_store::ObjectStore;
use serde_json::{Value, json};
use std::{sync::Arc, time::Instant};
use uuid::Uuid;
#[derive(Clone)]
pub struct AppState {
    pub pool: sea_orm::DatabaseConnection,
    pub store: Arc<dyn ObjectStore>,
}
pub fn router(state: AppState) -> Router {
    Router::new()
        .route(
            "/healthz",
            get(|| async { Json(json!({"status":"alive"})) }),
        )
        .route("/readyz", get(readiness))
        .route("/v1/me", get(me))
        .route("/v1/model-access/check", post(model_access))
        .with_state(state)
        .layer(middleware::from_fn(correlation))
}
pub(crate) async fn correlation(mut request: Request, next: Next) -> Response {
    let id = Uuid::new_v4();
    let start = Instant::now();
    request.extensions_mut().insert(id);
    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert("x-request-id", id.to_string().parse().expect("UUID header"));
    response.headers_mut().insert(
        "cache-control",
        "no-store".parse().expect("constant header"),
    );
    tracing::info!(event = "request.completed", correlation_id = %id, elapsed_ms = start.elapsed().as_millis() as u64, status = response.status().as_u16());
    response
}
async fn readiness(State(state): State<AppState>) -> (StatusCode, Json<Value>) {
    let (database, storage) =
        tokio::join!(db::ready(&state.pool), storage::ready(state.store.as_ref()));
    (
        if database && storage {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        },
        Json(json!({"database": database, "storage": storage})),
    )
}
async fn me(
    State(state): State<AppState>,
    Extension(id): Extension<Uuid>,
    headers: HeaderMap,
) -> Result<Json<auth::Account>, error::ApiError> {
    Ok(Json(auth::authenticate(&state.pool, &headers, id).await?))
}
async fn model_access(
    State(state): State<AppState>,
    Extension(id): Extension<Uuid>,
    headers: HeaderMap,
) -> Result<Json<Value>, error::ApiError> {
    let account = auth::authenticate(&state.pool, &headers, id).await?;
    Ok(Json(
        json!({"providerEnabled": false, "accountId": account.account_id, "reason": "P0_DIAGNOSTIC_ONLY"}),
    ))
}
