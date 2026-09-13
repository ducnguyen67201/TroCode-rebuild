use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use std::sync::Arc;
use tower::ServiceExt;
use tro_api::{AppState, auth, config::Config, db, router, storage};
async fn body(response: axum::response::Response) -> Value {
    serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes()).unwrap()
}
#[tokio::test]
async fn liveness_and_missing_credential_need_no_database() {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://unused@127.0.0.1/tro_rebuild_test")
        .unwrap();
    let app = router(AppState {
        pool,
        store: Arc::new(object_store::memory::InMemory::new()),
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().contains_key("x-request-id"));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/v1/me")
                .header("x-account-id", "teacher")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(body(response).await["code"], "UNAUTHORIZED");
}
#[tokio::test]
#[ignore = "requires isolated fixture database and private S3 service; npm run test:integration"]
async fn database_accounts_and_private_objects() {
    let config = Config::from_env().unwrap();
    let pool = db::connect(&config).await.unwrap();
    db::migrate(&pool).await.unwrap();
    let profiles: Value = serde_json::from_slice(
        &std::fs::read(std::env::var("TRO_PROFILES_FILE").unwrap()).unwrap(),
    )
    .unwrap();
    auth::seed(&pool, &profiles).await.unwrap();
    auth::seed(&pool, &profiles).await.unwrap();
    let count: i64 = sqlx::query_scalar("SELECT count(*) FROM fixture_accounts")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 3);
    let store = storage::connect(&config).unwrap();
    storage::seed(store.as_ref()).await.unwrap();
    let app = router(AppState {
        pool: pool.clone(),
        store,
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    for (index, profile) in ["teacher", "student-a", "student-b"].iter().enumerate() {
        let token = profiles[profile].as_str().unwrap();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/me")
                    .header("authorization", format!("Bearer {token}"))
                    .header("x-account-id", "spoofed")
                    .header("x-role", "teacher")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let identity = body(response).await;
        assert_eq!(
            identity["accountId"],
            uuid::Uuid::from_u128(index as u128 + 1).to_string()
        );
        assert_eq!(
            identity["role"],
            if index == 0 { "teacher" } else { "student" }
        );
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/model-access/check")
                    .header("authorization", format!("Bearer {token}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let access = body(response).await;
        assert_eq!(access["providerEnabled"], false);
        assert_eq!(access["accountId"], identity["accountId"]);
    }
    for expired in [true, false] {
        let token = format!(
            "{}{}",
            uuid::Uuid::new_v4().simple(),
            uuid::Uuid::new_v4().simple()
        );
        let digest = auth::digest(&token);
        sqlx::query("INSERT INTO fixture_sessions (token_digest,account_id,expires_at,revoked) VALUES ($1,$2,NOW()+CASE WHEN $3 THEN INTERVAL '-1 day' ELSE INTERVAL '1 day' END,$4)")
            .bind(&digest).bind(uuid::Uuid::from_u128(1)).bind(expired).bind(!expired).execute(&pool).await.unwrap();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/v1/me")
                    .header("authorization", format!("Bearer {token}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        sqlx::query("DELETE FROM fixture_sessions WHERE token_digest=$1")
            .bind(digest)
            .execute(&pool)
            .await
            .unwrap();
    }
    let response = reqwest::get(format!(
        "{}/{}/{}",
        config.s3_endpoint,
        storage::BUCKET,
        storage::SAMPLE
    ))
    .await
    .unwrap();
    assert!(matches!(response.status().as_u16(), 401 | 403));
    let schema: Value = serde_json::from_str(include_str!(
        "../../../packages/contracts/schema/model-access.schema.json"
    ))
    .unwrap();
    assert_eq!(
        schema["properties"]["providerEnabled"]["enum"],
        json!([false])
    );
}
