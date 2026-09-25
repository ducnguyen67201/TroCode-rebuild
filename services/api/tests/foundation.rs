use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, Set,
    entity::prelude::ChronoDateTimeUtc, sea_query::Expr,
};
use serde_json::{Value, json};
use std::sync::Arc;
use tower::ServiceExt;
use tro_api::{
    AppState, auth,
    config::Config,
    db,
    entities::{fixture_account, fixture_session},
    router, storage,
};
fn utc_now() -> ChronoDateTimeUtc {
    std::time::SystemTime::now().into()
}
async fn body(response: axum::response::Response) -> Value {
    serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes()).unwrap()
}
#[tokio::test]
async fn liveness_and_missing_credential_need_no_database() {
    let pool = DatabaseConnection::default();
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
    let protected_digest = auth::digest(profiles["teacher"].as_str().unwrap());
    fixture_session::Entity::update_many()
        .col_expr(
            fixture_session::Column::ExpiresAt,
            Expr::value(utc_now() - std::time::Duration::from_secs(24 * 60 * 60)),
        )
        .col_expr(fixture_session::Column::Revoked, Expr::value(true))
        .filter(fixture_session::Column::TokenDigest.eq(protected_digest.clone()))
        .exec(&pool)
        .await
        .unwrap();
    auth::seed(&pool, &profiles).await.unwrap();
    let protected_session = fixture_session::Entity::find_by_id(protected_digest.clone())
        .one(&pool)
        .await
        .unwrap()
        .unwrap();
    assert!(protected_session.revoked);
    assert!(protected_session.expires_at < utc_now());
    fixture_session::Entity::update_many()
        .col_expr(
            fixture_session::Column::ExpiresAt,
            Expr::value(utc_now() + std::time::Duration::from_secs(30 * 24 * 60 * 60)),
        )
        .col_expr(fixture_session::Column::Revoked, Expr::value(false))
        .filter(fixture_session::Column::TokenDigest.eq(protected_digest))
        .exec(&pool)
        .await
        .unwrap();
    let count = fixture_account::Entity::find().count(&pool).await.unwrap();
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
        let expires_at = if expired {
            utc_now() - std::time::Duration::from_secs(24 * 60 * 60)
        } else {
            utc_now() + std::time::Duration::from_secs(24 * 60 * 60)
        };
        fixture_session::Entity::insert(fixture_session::ActiveModel {
            token_digest: Set(digest.clone()),
            account_id: Set(uuid::Uuid::from_u128(1)),
            expires_at: Set(expires_at),
            revoked: Set(!expired),
        })
        .exec(&pool)
        .await
        .unwrap();
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
        fixture_session::Entity::delete_by_id(digest)
            .exec(&pool)
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
