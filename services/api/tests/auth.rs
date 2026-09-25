use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::{sync::Arc, time::Duration};
use tower::ServiceExt;
use tro_api::{
    auth::{
        PendingWorkspaceMembershipStore, google::VerifiedGoogleIdentity, jwt::JwtService,
        sessions::SessionService,
    },
    config::HostedConfig,
    entities::account,
    hosted::{HostedState, router},
    workspace::WorkspaceService,
};

fn config() -> HostedConfig {
    HostedConfig {
        database_url: "postgres://unused@127.0.0.1/tro".to_owned(),
        bind: "127.0.0.1:4318".parse().unwrap(),
        issuer: "https://api.tro.test".to_owned(),
        audience: "tro-desktop-api".to_owned(),
        google_client_id: "desktop.apps.googleusercontent.com".to_owned(),
        google_client_secret: None,
        jwt_key: vec![7_u8; 32],
        refresh_key: vec![8_u8; 32],
        access_ttl: Duration::from_secs(900),
        refresh_ttl: Duration::from_secs(30 * 24 * 60 * 60),
    }
}

fn app() -> axum::Router {
    router(
        HostedState::from_database(
            &config(),
            DatabaseConnection::default(),
            Arc::new(PendingWorkspaceMembershipStore),
            Arc::new(WorkspaceService::new(DatabaseConnection::default())),
        )
        .unwrap(),
    )
}

#[tokio::test]
async fn health_is_public_and_account_routes_require_app_jwt() {
    let response = app()
        .oneshot(Request::get("/healthz").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["cache-control"], "no-store");

    for authorization in [None, Some(format!("Bearer {}", "a".repeat(64)))] {
        let mut request = Request::get("/v1/me");
        if let Some(value) = authorization {
            request = request.header("authorization", value);
        }
        let response = app()
            .oneshot(request.body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        let body: serde_json::Value =
            serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes())
                .unwrap();
        assert!(body["code"].as_str().unwrap().starts_with("SESSION_"));
    }
}

#[tokio::test]
async fn exchange_rejects_unknown_or_oversized_input_before_provider_access() {
    let response = app()
        .oneshot(
            Request::post("/v1/auth/google/exchange")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "code": "x",
                        "codeVerifier": "a".repeat(64),
                        "redirectUri": "http://127.0.0.1:51000/oauth2/callback",
                        "nonce": "n".repeat(32),
                        "token": "must-not-be-accepted"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let response = app()
        .oneshot(
            Request::post("/v1/auth/google/exchange")
                .header("content-type", "application/json")
                .body(Body::from("x".repeat(17 * 1024)))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
#[ignore = "requires the isolated PostgreSQL fixture; npm run test:integration"]
async fn rotating_a_used_refresh_token_revokes_its_device_family() {
    let fixture = tro_api::config::Config::from_env().unwrap();
    let database = tro_api::persistence::connect(&fixture.database_url)
        .await
        .unwrap();
    tro_api_migration::migrate(&database).await.unwrap();
    let jwt = JwtService::new(
        &[7_u8; 32],
        "https://api.tro.test".to_owned(),
        "tro-desktop-api".to_owned(),
        Duration::from_secs(900),
    )
    .unwrap();
    let sessions = SessionService::new(
        database.clone(),
        jwt,
        vec![8_u8; 32],
        Duration::from_secs(30 * 24 * 60 * 60),
        Arc::new(PendingWorkspaceMembershipStore),
    )
    .unwrap();
    let email = format!("{}@example.com", uuid::Uuid::new_v4().simple());
    let identity = VerifiedGoogleIdentity {
        issuer: "https://accounts.google.com".to_owned(),
        subject: uuid::Uuid::new_v4().to_string(),
        email: email.clone(),
        email_normalized: email,
        display_name: "Rotation Test".to_owned(),
    };
    let first = sessions
        .login(identity, uuid::Uuid::new_v4())
        .await
        .unwrap();
    let account_id = first.account.account_id;
    let replacement = sessions
        .refresh(&first.refresh_token, uuid::Uuid::new_v4())
        .await
        .unwrap();

    let replay = sessions
        .refresh(&first.refresh_token, uuid::Uuid::new_v4())
        .await
        .unwrap_err();
    assert_eq!(replay.code, "SESSION_REPLAYED");
    let revoked = sessions
        .refresh(&replacement.refresh_token, uuid::Uuid::new_v4())
        .await
        .unwrap_err();
    assert_eq!(revoked.code, "SESSION_REVOKED");

    account::Entity::delete_by_id(account_id)
        .exec(&database)
        .await
        .unwrap();
}
