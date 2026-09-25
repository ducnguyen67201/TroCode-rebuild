//! Isolated proof identity and bounded nonstreaming model access. No native authority.
use crate::{
    auth::digest,
    db,
    entities::{proof_session, runtime_grant},
    error::ApiError,
};
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, ExprTrait, QueryFilter, QuerySelect, Set,
    TransactionTrait, entity::prelude::ChronoDateTimeUtc, sea_query::Expr,
};
use serde::Deserialize;
use serde_json::{Value, json};
use std::time::{Duration, SystemTime};
use uuid::Uuid;

fn utc_now() -> ChronoDateTimeUtc {
    SystemTime::now().into()
}

#[derive(Clone)]
pub struct Gateway {
    pub pool: DatabaseConnection,
    client: reqwest::Client,
    key: String,
    model: String,
    upstream: String,
}
impl Gateway {
    pub fn new(pool: DatabaseConnection, key: String, model: String) -> Result<Self, &'static str> {
        if key.is_empty() || model.is_empty() || model.len() > 128 {
            return Err("Proof model configuration is incomplete.");
        }
        Ok(Self {
            pool,
            key,
            model,
            upstream: "https://api.openai.com/v1/responses".into(),
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(25))
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .map_err(|_| "Model client unavailable.")?,
        })
    }
}
pub fn router(state: Gateway) -> Router {
    Router::new()
        .route(
            "/healthz",
            get(|| async { Json(json!({"status":"alive"})) }),
        )
        .route("/readyz", get(ready))
        .route("/v1/me", get(me))
        .route("/v1/runtime-grants", post(grant))
        .route("/v1/responses", post(responses))
        .with_state(state)
        .layer(DefaultBodyLimit::max(8 * 1024 * 1024))
        .layer(axum::middleware::from_fn(super::correlation))
}
fn bearer(headers: &HeaderMap, id: Uuid) -> Result<&str, ApiError> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .filter(|v| v.len() == 64 && v.bytes().all(|b| b.is_ascii_hexdigit()))
        .ok_or(ApiError::unauthorized(id))
}
async fn identity(state: &Gateway, token: &str, id: Uuid) -> Result<Uuid, ApiError> {
    proof_session::Entity::find_by_id(digest(token))
        .filter(Expr::col(proof_session::Column::ExpiresAt).gt(Expr::current_timestamp()))
        .filter(proof_session::Column::Revoked.eq(false))
        .one(&state.pool)
        .await
        .map_err(|_| ApiError::internal(id))?
        .map(|session| session.account_id)
        .ok_or(ApiError::unauthorized(id))
}
async fn me(
    State(state): State<Gateway>,
    axum::Extension(id): axum::Extension<Uuid>,
    headers: HeaderMap,
) -> Result<Json<Value>, ApiError> {
    let account = identity(&state, bearer(&headers, id)?, id).await?;
    Ok(Json(json!({"accountId":account})))
}
async fn ready(State(state): State<Gateway>) -> (StatusCode, Json<Value>) {
    let ready = db::ready(&state.pool).await;
    (
        if ready {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        },
        Json(json!({"database":ready})),
    )
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct GrantRequest {
    teaching_session_id: Uuid,
}
async fn grant(
    State(state): State<Gateway>,
    axum::Extension(id): axum::Extension<Uuid>,
    headers: HeaderMap,
    Json(body): Json<GrantRequest>,
) -> Result<Json<Value>, ApiError> {
    let token = bearer(&headers, id)?;
    let account = identity(&state, token, id).await?;
    let session_digest = digest(token);
    let grant = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
    // One grant per live account session/teaching session prevents budget-reset retries.
    // Issuance always locks the proof session before checking/inserting grants.
    // Reservation below locks an existing grant before its parent proof session;
    // reassess this order if a future path ever locks both while issuing a grant.
    let tx = state
        .pool
        .begin()
        .await
        .map_err(|_| ApiError::internal(id))?;
    let locked_session = proof_session::Entity::find_by_id(session_digest.clone())
        .filter(Expr::col(proof_session::Column::ExpiresAt).gt(Expr::current_timestamp()))
        .filter(proof_session::Column::Revoked.eq(false))
        .lock_exclusive()
        .one(&tx)
        .await
        .map_err(|_| ApiError::internal(id))?
        .filter(|session| session.account_id == account)
        .ok_or(ApiError::unauthorized(id))?;
    let exists = runtime_grant::Entity::find()
        .filter(runtime_grant::Column::SessionDigest.eq(locked_session.token_digest))
        .filter(runtime_grant::Column::TeachingSessionId.eq(body.teaching_session_id))
        .one(&tx)
        .await
        .map_err(|_| ApiError::internal(id))?
        .is_some();
    if exists {
        return Err(ApiError::unauthorized(id));
    }
    runtime_grant::Entity::insert(runtime_grant::ActiveModel {
        token_digest: Set(digest(&grant)),
        session_digest: Set(session_digest),
        account_id: Set(account),
        teaching_session_id: Set(body.teaching_session_id),
        expires_at: Set(utc_now() + Duration::from_secs(300)),
        remaining_calls: Set(4),
        revoked: Set(false),
    })
    .exec(&tx)
    .await
    .map_err(|_| ApiError::internal(id))?;
    tx.commit().await.map_err(|_| ApiError::internal(id))?;
    Ok(Json(
        json!({"grant":grant,"model":state.model,"accountId":account,"expiresIn":300}),
    ))
}
pub fn validate_request(body: &Value, model: &str) -> bool {
    let Some(object) = body.as_object() else {
        return false;
    };
    let allowed = [
        "model",
        "input",
        "instructions",
        "max_output_tokens",
        "parallel_tool_calls",
        "store",
        "stream",
        "tools",
        "text",
        "include",
        "tool_choice",
    ];
    object.keys().all(|key| allowed.contains(&key.as_str()))
        && body["model"] == model
        && body
            .get("input")
            .is_some_and(|v| v.is_array() || v.is_string())
        && body["max_output_tokens"]
            .as_u64()
            .is_some_and(|v| v > 0 && v <= 1024)
        && body.get("store").is_none_or(|v| v == false)
        && body.get("stream").is_none_or(|v| v == false)
        && body.get("parallel_tool_calls").is_none_or(|v| v == false)
        && body
            .get("tools")
            .is_none_or(|v| v.as_array().is_some_and(Vec::is_empty))
        && body
            .get("include")
            .is_none_or(|v| v.as_array().is_some_and(Vec::is_empty))
        && body
            .get("tool_choice")
            .is_none_or(|v| v == "none" || v == "auto")
}
async fn responses(
    State(state): State<Gateway>,
    axum::Extension(id): axum::Extension<Uuid>,
    headers: HeaderMap,
    Json(mut body): Json<Value>,
) -> Result<Json<Value>, ApiError> {
    if !validate_request(&body, &state.model) {
        return Err(ApiError::unauthorized(id));
    }
    let grant_digest = digest(bearer(&headers, id)?);
    // Keep the grant lock and parent-session validation in one transaction, and
    // consume budget before dispatch so provider failures still spend a call.
    let tx = state
        .pool
        .begin()
        .await
        .map_err(|_| ApiError::internal(id))?;
    let grant = runtime_grant::Entity::find_by_id(grant_digest.clone())
        .filter(runtime_grant::Column::Revoked.eq(false))
        .filter(Expr::col(runtime_grant::Column::ExpiresAt).gt(Expr::current_timestamp()))
        .filter(runtime_grant::Column::RemainingCalls.gt(0))
        .lock_exclusive()
        .one(&tx)
        .await
        .map_err(|_| ApiError::internal(id))?
        .ok_or(ApiError::unauthorized(id))?;
    proof_session::Entity::find_by_id(grant.session_digest)
        .filter(proof_session::Column::AccountId.eq(grant.account_id))
        .filter(proof_session::Column::Revoked.eq(false))
        .filter(Expr::col(proof_session::Column::ExpiresAt).gt(Expr::current_timestamp()))
        .lock_exclusive()
        .one(&tx)
        .await
        .map_err(|_| ApiError::internal(id))?
        .ok_or(ApiError::unauthorized(id))?;
    let reserved = runtime_grant::Entity::update_many()
        .col_expr(
            runtime_grant::Column::RemainingCalls,
            Expr::col(runtime_grant::Column::RemainingCalls).sub(1),
        )
        .filter(runtime_grant::Column::TokenDigest.eq(grant_digest))
        .filter(runtime_grant::Column::Revoked.eq(false))
        .filter(Expr::col(runtime_grant::Column::ExpiresAt).gt(Expr::current_timestamp()))
        .filter(runtime_grant::Column::RemainingCalls.gt(0))
        .exec(&tx)
        .await
        .map_err(|_| ApiError::internal(id))?;
    if reserved.rows_affected != 1 {
        return Err(ApiError::unauthorized(id));
    }
    tx.commit().await.map_err(|_| ApiError::internal(id))?;
    body["store"] = json!(false);
    body["stream"] = json!(false);
    // A fixed provider origin; caller supplied URLs and redirects are never accepted.
    let mut response = state
        .client
        .post(&state.upstream)
        .bearer_auth(&state.key)
        .json(&body)
        .send()
        .await
        .map_err(|_| ApiError::internal(id))?;
    if !response.status().is_success() {
        return Err(ApiError::internal(id));
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(|_| ApiError::internal(id))? {
        if bytes.len() + chunk.len() > 2 * 1024 * 1024 {
            return Err(ApiError::internal(id));
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(Json(
        serde_json::from_slice(&bytes).map_err(|_| ApiError::internal(id))?,
    ))
}

pub async fn run() -> Result<(), &'static str> {
    let database = std::env::var("DATABASE_URL").map_err(|_| "Proof database is required.")?;
    let url = url::Url::parse(&database).map_err(|_| "Invalid proof database.")?;
    if !matches!(url.scheme(), "postgres" | "postgresql") || url.path() != "/tro_rebuild_proof" {
        return Err("Proof mode requires the isolated tro_rebuild_proof database.");
    }
    let bind: std::net::SocketAddr = std::env::var("TRO_API_BIND")
        .map_err(|_| "Proof bind is required.")?
        .parse()
        .map_err(|_| "Invalid proof bind.")?;
    if !bind.ip().is_loopback() {
        return Err("Bind behind a trusted HTTPS reverse proxy on loopback.");
    }
    let pool = db::connect_url(&database)
        .await
        .map_err(|_| "Proof database unavailable.")?;
    db::validate_target(&pool, "tro_rebuild_proof")
        .await
        .map_err(|_| "Refusing legacy database state.")?;
    match std::env::args().nth(1).as_deref().unwrap_or("serve") {
        "migrate" => db::MIGRATOR
            .run(pool.get_postgres_connection_pool())
            .await
            .map_err(|_| "Proof migration failed."),
        "serve" => {
            if !db::ready(&pool).await {
                return Err("Apply proof migrations first.");
            }
            let gateway = Gateway::new(
                pool,
                std::env::var("OPENAI_API_KEY").map_err(|_| "Provider key required on backend.")?,
                std::env::var("TRO_MODEL").map_err(|_| "Proof model required.")?,
            )?;
            let listener = tokio::net::TcpListener::bind(bind)
                .await
                .map_err(|_| "Proof bind unavailable.")?;
            axum::serve(listener, router(gateway))
                .with_graceful_shutdown(async {
                    let _ = tokio::signal::ctrl_c().await;
                })
                .await
                .map_err(|_| "Proof server failed.")
        }
        _ => Err("Proof mode supports migrate or serve; provision accounts explicitly."),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_hosted_tools_storage_and_budget_expansion() {
        let valid = json!({"model":"proof-model","input":"Help","max_output_tokens":1024,"tools":[],"store":false});
        assert!(validate_request(&valid, "proof-model"));
        for (key, value) in [
            ("tools", json!([{"type":"computer_use_preview"}])),
            ("store", json!(true)),
            ("max_output_tokens", json!(1025)),
            ("previous_response_id", json!("remote")),
            ("background", json!(true)),
            ("model", json!("other")),
            ("stream", json!(true)),
        ] {
            let mut body = valid.clone();
            body[key] = value;
            assert!(!validate_request(&body, "proof-model"));
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::entities::proof_account;
    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    use tower::ServiceExt;

    async fn call(app: Router, path: &str, token: &str, body: Value) -> (StatusCode, Value) {
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(path)
                    .header("authorization", format!("Bearer {token}"))
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        (status, serde_json::from_slice(&bytes).unwrap())
    }
    #[tokio::test]
    #[ignore = "isolated fixture database; run through npm run test:integration"]
    async fn grants_enforce_identity_expiry_revocation_and_atomic_provider_budget() {
        let config = crate::config::Config::from_env().unwrap();
        let pool = db::connect(&config).await.unwrap();
        db::migrate(&pool).await.unwrap();
        let count = Arc::new(AtomicUsize::new(0));
        let counter = count.clone();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let mock = Router::new().route(
            "/responses",
            post(move |headers: HeaderMap, Json(body): Json<Value>| {
                let counter = counter.clone();
                async move {
                    assert_eq!(headers["authorization"], "Bearer fake-provider-key");
                    assert_eq!(body["store"], false);
                    assert_eq!(body["stream"], false);
                    counter.fetch_add(1, Ordering::SeqCst);
                    if body["input"] == "slow" {
                        tokio::time::sleep(Duration::from_millis(300)).await;
                    }
                    if body["input"] == "oversize" {
                        return Json(json!({"output":"x".repeat(2 * 1024 * 1024)}));
                    }
                    Json(json!({"id":"mock-response","output":[]}))
                }
            }),
        );
        let server = tokio::spawn(async move {
            axum::serve(listener, mock).await.unwrap();
        });
        let mut gateway = Gateway::new(
            pool.clone(),
            "fake-provider-key".into(),
            "proof-model".into(),
        )
        .unwrap();
        // Only this private test module can replace the fixed production origin.
        gateway.upstream = format!("http://{address}/responses");
        gateway.client = reqwest::Client::builder()
            .timeout(Duration::from_millis(100))
            .build()
            .unwrap();
        let app = router(gateway);
        let account = Uuid::new_v4();
        let token = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
        proof_account::Entity::insert(proof_account::ActiveModel {
            id: Set(account),
            role: Set("student".to_owned()),
        })
        .exec(&pool)
        .await
        .unwrap();
        proof_session::Entity::insert(proof_session::ActiveModel {
            token_digest: Set(digest(&token)),
            account_id: Set(account),
            expires_at: Set(utc_now() + Duration::from_secs(3600)),
            revoked: Set(false),
        })
        .exec(&pool)
        .await
        .unwrap();
        let session = Uuid::new_v4();
        let grant_request = json!({"teachingSessionId":session});
        assert_eq!(
            call(
                app.clone(),
                "/v1/runtime-grants",
                &"0".repeat(64),
                grant_request.clone()
            )
            .await
            .0,
            StatusCode::UNAUTHORIZED
        );
        let (status, issued) = call(
            app.clone(),
            "/v1/runtime-grants",
            &token,
            grant_request.clone(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(issued["accountId"], account.to_string());
        assert_eq!(
            call(app.clone(), "/v1/runtime-grants", &token, grant_request)
                .await
                .0,
            StatusCode::UNAUTHORIZED
        );
        let concurrent_request = json!({"teachingSessionId":Uuid::new_v4()});
        let (first, second) = tokio::join!(
            call(
                app.clone(),
                "/v1/runtime-grants",
                &token,
                concurrent_request.clone()
            ),
            call(
                app.clone(),
                "/v1/runtime-grants",
                &token,
                concurrent_request
            )
        );
        let issuance = [first, second];
        assert_eq!(
            issuance
                .iter()
                .filter(|(status, _)| *status == StatusCode::OK)
                .count(),
            1
        );
        assert_eq!(
            issuance
                .iter()
                .filter(|(status, _)| *status == StatusCode::UNAUTHORIZED)
                .count(),
            1
        );
        let concurrent_grant = issuance
            .iter()
            .find(|(status, _)| *status == StatusCode::OK)
            .and_then(|(_, body)| body["grant"].as_str())
            .unwrap()
            .to_owned();
        let grant = issued["grant"].as_str().unwrap();
        let body = json!({"model":"proof-model","input":"Help","max_output_tokens":1024});
        let mut tasks = Vec::new();
        for _ in 0..6 {
            let app = app.clone();
            let grant = grant.to_owned();
            let body = body.clone();
            tasks.push(tokio::spawn(async move {
                call(app, "/v1/responses", &grant, body).await.0
            }));
        }
        let mut accepted = 0;
        for task in tasks {
            let status = task.await.unwrap();
            if status == StatusCode::OK {
                accepted += 1;
            } else {
                assert_eq!(status, StatusCode::UNAUTHORIZED);
            }
        }
        assert_eq!(accepted, 4);
        assert_eq!(count.load(Ordering::SeqCst), 4);
        runtime_grant::Entity::update_many()
            .col_expr(runtime_grant::Column::RemainingCalls, Expr::value(1))
            .col_expr(
                runtime_grant::Column::ExpiresAt,
                Expr::value(utc_now() - Duration::from_secs(1)),
            )
            .filter(runtime_grant::Column::TokenDigest.eq(digest(grant)))
            .exec(&pool)
            .await
            .unwrap();
        assert_eq!(
            call(app.clone(), "/v1/responses", grant, body.clone())
                .await
                .0,
            StatusCode::UNAUTHORIZED
        );
        runtime_grant::Entity::update_many()
            .col_expr(
                runtime_grant::Column::ExpiresAt,
                Expr::value(utc_now() + Duration::from_secs(3600)),
            )
            .filter(runtime_grant::Column::TokenDigest.eq(digest(grant)))
            .exec(&pool)
            .await
            .unwrap();
        let mismatched_account = Uuid::new_v4();
        proof_account::Entity::insert(proof_account::ActiveModel {
            id: Set(mismatched_account),
            role: Set("student".to_owned()),
        })
        .exec(&pool)
        .await
        .unwrap();
        runtime_grant::Entity::update_many()
            .col_expr(
                runtime_grant::Column::AccountId,
                Expr::value(mismatched_account),
            )
            .filter(runtime_grant::Column::TokenDigest.eq(digest(grant)))
            .exec(&pool)
            .await
            .unwrap();
        assert_eq!(
            call(app.clone(), "/v1/responses", grant, body.clone())
                .await
                .0,
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(count.load(Ordering::SeqCst), 4);
        runtime_grant::Entity::update_many()
            .col_expr(runtime_grant::Column::AccountId, Expr::value(account))
            .filter(runtime_grant::Column::TokenDigest.eq(digest(grant)))
            .exec(&pool)
            .await
            .unwrap();
        proof_session::Entity::update_many()
            .col_expr(proof_session::Column::Revoked, Expr::value(true))
            .filter(proof_session::Column::TokenDigest.eq(digest(&token)))
            .exec(&pool)
            .await
            .unwrap();
        let (status, error) = call(app.clone(), "/v1/responses", grant, body).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert!(!error.to_string().contains("fake-provider-key"));
        assert_eq!(count.load(Ordering::SeqCst), 4);
        proof_session::Entity::update_many()
            .col_expr(proof_session::Column::Revoked, Expr::value(false))
            .filter(proof_session::Column::TokenDigest.eq(digest(&token)))
            .exec(&pool)
            .await
            .unwrap();
        runtime_grant::Entity::update_many()
            .col_expr(runtime_grant::Column::RemainingCalls, Expr::value(2))
            .filter(runtime_grant::Column::TokenDigest.eq(digest(grant)))
            .exec(&pool)
            .await
            .unwrap();
        for input in ["slow", "oversize"] {
            let (status, error) = call(
                app.clone(),
                "/v1/responses",
                grant,
                json!({"model":"proof-model","input":input,"max_output_tokens":1024}),
            )
            .await;
            assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
            assert!(!error.to_string().contains("fake-provider-key"));
        }
        assert_eq!(count.load(Ordering::SeqCst), 6);
        let oversized=app.clone().oneshot(Request::builder().method("POST").uri("/v1/responses")
            .header("authorization",format!("Bearer {grant}")).header("content-type","application/json")
            .body(Body::from(json!({"model":"proof-model","input":"x".repeat(8*1024*1024),"max_output_tokens":1024}).to_string())).unwrap()).await.unwrap();
        assert_eq!(oversized.status(), StatusCode::PAYLOAD_TOO_LARGE);
        assert_eq!(count.load(Ordering::SeqCst), 6);
        runtime_grant::Entity::delete_by_id(digest(&concurrent_grant))
            .exec(&pool)
            .await
            .unwrap();
        runtime_grant::Entity::delete_by_id(digest(grant))
            .exec(&pool)
            .await
            .unwrap();
        proof_session::Entity::delete_by_id(digest(&token))
            .exec(&pool)
            .await
            .unwrap();
        proof_account::Entity::delete_by_id(account)
            .exec(&pool)
            .await
            .unwrap();
        proof_account::Entity::delete_by_id(mismatched_account)
            .exec(&pool)
            .await
            .unwrap();
        server.abort();
    }
}
