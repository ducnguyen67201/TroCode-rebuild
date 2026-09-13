//! Isolated proof identity and bounded nonstreaming model access. No native authority.
use crate::{auth::digest, db, error::ApiError};
use axum::{
    Json, Router,
    extract::{DefaultBodyLimit, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{PgPool, Row};
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone)]
pub struct Gateway {
    pub pool: PgPool,
    client: reqwest::Client,
    key: String,
    model: String,
    upstream: String,
}
impl Gateway {
    pub fn new(pool: PgPool, key: String, model: String) -> Result<Self, &'static str> {
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
    sqlx::query_scalar("SELECT account_id FROM proof_sessions WHERE token_digest=$1 AND expires_at>NOW() AND NOT revoked")
        .bind(digest(token)).fetch_optional(&state.pool).await.map_err(|_| ApiError::internal(id))?
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
    let grant = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
    // One grant per live account session/teaching session prevents budget-reset retries.
    let mut tx = state
        .pool
        .begin()
        .await
        .map_err(|_| ApiError::internal(id))?;
    sqlx::query("SELECT token_digest FROM proof_sessions WHERE token_digest=$1 FOR UPDATE")
        .bind(digest(token))
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| ApiError::internal(id))?;
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM runtime_grants WHERE session_digest=$1 AND teaching_session_id=$2)")
        .bind(digest(token)).bind(body.teaching_session_id).fetch_one(&mut *tx).await.map_err(|_| ApiError::internal(id))?;
    if exists {
        return Err(ApiError::unauthorized(id));
    }
    sqlx::query(
        "INSERT INTO runtime_grants VALUES ($1,$2,$3,$4,NOW()+INTERVAL '5 minutes',4,FALSE)",
    )
    .bind(digest(&grant))
    .bind(digest(token))
    .bind(account)
    .bind(body.teaching_session_id)
    .execute(&mut *tx)
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
    let token = bearer(&headers, id)?;
    let reserved = sqlx::query("UPDATE runtime_grants g SET remaining_calls=remaining_calls-1 FROM proof_sessions s WHERE g.token_digest=$1 AND g.session_digest=s.token_digest AND g.account_id=s.account_id AND NOT g.revoked AND NOT s.revoked AND g.expires_at>NOW() AND s.expires_at>NOW() AND g.remaining_calls>0 RETURNING g.account_id")
        .bind(digest(token)).fetch_optional(&state.pool).await.map_err(|_| ApiError::internal(id))?;
    if reserved.is_none() {
        return Err(ApiError::unauthorized(id));
    }
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
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database)
        .await
        .map_err(|_| "Proof database unavailable.")?;
    let legacy: bool = sqlx::query(
        "SELECT current_database() AS name, to_regclass('public.users') IS NOT NULL AS legacy",
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| "Proof database unavailable.")?
    .get("legacy");
    if legacy {
        return Err("Refusing legacy database state.");
    }
    match std::env::args().nth(1).as_deref().unwrap_or("serve") {
        "migrate" => db::MIGRATOR
            .run(&pool)
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
        sqlx::query("INSERT INTO proof_accounts VALUES ($1,'student')")
            .bind(account)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO proof_sessions VALUES ($1,$2,NOW()+INTERVAL '1 hour',FALSE)")
            .bind(digest(&token))
            .bind(account)
            .execute(&pool)
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
        sqlx::query("UPDATE runtime_grants SET remaining_calls=1,expires_at=NOW()-INTERVAL '1 second' WHERE token_digest=$1")
            .bind(digest(grant)).execute(&pool).await.unwrap();
        assert_eq!(
            call(app.clone(), "/v1/responses", grant, body.clone())
                .await
                .0,
            StatusCode::UNAUTHORIZED
        );
        sqlx::query(
            "UPDATE runtime_grants SET expires_at=NOW()+INTERVAL '1 hour' WHERE token_digest=$1",
        )
        .bind(digest(grant))
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query("UPDATE proof_sessions SET revoked=TRUE WHERE token_digest=$1")
            .bind(digest(&token))
            .execute(&pool)
            .await
            .unwrap();
        let (status, error) = call(app.clone(), "/v1/responses", grant, body).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert!(!error.to_string().contains("fake-provider-key"));
        assert_eq!(count.load(Ordering::SeqCst), 4);
        sqlx::query("UPDATE proof_sessions SET revoked=FALSE WHERE token_digest=$1")
            .bind(digest(&token))
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("UPDATE runtime_grants SET remaining_calls=2 WHERE token_digest=$1")
            .bind(digest(grant))
            .execute(&pool)
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
        sqlx::query("DELETE FROM runtime_grants WHERE token_digest=$1")
            .bind(digest(grant))
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM proof_sessions WHERE token_digest=$1")
            .bind(digest(&token))
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM proof_accounts WHERE id=$1")
            .bind(account)
            .execute(&pool)
            .await
            .unwrap();
        server.abort();
    }
}
