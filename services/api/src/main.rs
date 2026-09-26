use tro_api::{config::Config, db, storage};
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_env_filter("info")
        .init();
    if let Err(message) = run().await {
        eprintln!("{message}");
        std::process::exit(1);
    }
}
async fn run() -> Result<(), &'static str> {
    if std::env::var("TRO_API_MODE").as_deref() == Ok("proof") {
        return tro_api::model_gateway::run().await;
    }
    if std::env::var("TRO_API_MODE").as_deref() == Ok("hosted") {
        return run_hosted().await;
    }
    let config = Config::from_env()?;
    let pool = db::connect(&config)
        .await
        .map_err(|_| "Fixture database unavailable.")?;
    match std::env::args().nth(1).as_deref().unwrap_or("serve") {
        "migrate" => db::migrate(&pool)
            .await
            .map_err(|_| "Fixture migration refused or failed."),
        "seed" => {
            let path = std::env::var("TRO_PROFILES_FILE")
                .map_err(|_| "Fixture credential file is required.")?;
            let profiles = serde_json::from_slice(
                &std::fs::read(path).map_err(|_| "Unable to read fixture profiles.")?,
            )
            .map_err(|_| "Invalid profile file.")?;
            tro_api::auth::seed(&pool, &profiles)
                .await
                .map_err(|_| "Unable to seed fixture accounts.")?;
            let store = storage::connect(&config).map_err(|_| "Invalid storage configuration.")?;
            storage::seed(store.as_ref())
                .await
                .map_err(|_| "Unable to seed fixture material.")
        }
        "serve" => {
            let store = storage::connect(&config).map_err(|_| "Invalid storage configuration.")?;
            let listener = tokio::net::TcpListener::bind(config.bind)
                .await
                .map_err(|_| "Fixture API port unavailable.")?;
            axum::serve(listener, tro_api::router(tro_api::AppState { pool, store }))
                .with_graceful_shutdown(async {
                    let _ = tokio::signal::ctrl_c().await;
                })
                .await
                .map_err(|_| "Fixture API stopped unexpectedly.")
        }
        _ => Err("Use serve, migrate, or seed."),
    }
}

async fn run_hosted() -> Result<(), &'static str> {
    let config = tro_api::config::HostedConfig::from_env()?;
    let state = tro_api::hosted::HostedState::connect(&config).await?;
    match std::env::args().nth(1).as_deref().unwrap_or("serve") {
        "migrate" => {
            if std::env::var("TRO_ALLOW_HOSTED_MIGRATION").as_deref() != Ok("1") {
                return Err("Hosted migrations require TRO_ALLOW_HOSTED_MIGRATION=1.");
            }
            tro_api_migration::migrate(&state.database)
                .await
                .map_err(|_| "Hosted migration failed.")
        }
        "seed-local-workspace" => {
            if std::env::var("TRO_ALLOW_LOCAL_WORKSPACE_SEED").as_deref() != Ok("1")
                || url::Url::parse(&config.database_url)
                    .ok()
                    .and_then(|database| database.host_str().map(str::to_owned))
                    .as_deref()
                    != Some("127.0.0.1")
            {
                return Err("Local workspace seed requires an explicit loopback database.");
            }
            let email = std::env::args()
                .nth(2)
                .ok_or("Local workspace seed requires an email.")?;
            let workspace_name = std::env::args()
                .nth(3)
                .ok_or("Local workspace seed requires a workspace name.")?;
            tro_api::workspace::seed_local_development_workspace(
                &state.database,
                tro_api::workspace::LocalDevelopmentWorkspaceSeed {
                    owner_account_id: uuid::Uuid::from_u128(
                        0x8c13_7e2d_e35e_4cf8_8c54_31d8_b385_79a1,
                    ),
                    owner_membership_id: uuid::Uuid::from_u128(
                        0x4f98_adce_fa06_48c0_a42c_ab2c_2b62_743e,
                    ),
                    workspace_id: uuid::Uuid::from_u128(0xd0af_b9d5_ed0d_4e44_8b12_4ae3_62d4_e45f),
                    workspace_name: &workspace_name,
                    member_email: &email,
                },
            )
            .await
            .map_err(|_| "Unable to seed the local development workspace.")
        }
        "serve" => {
            let listener = tokio::net::TcpListener::bind(config.bind)
                .await
                .map_err(|_| "Hosted API port unavailable.")?;
            axum::serve(listener, tro_api::hosted::router(state))
                .with_graceful_shutdown(async {
                    let _ = tokio::signal::ctrl_c().await;
                })
                .await
                .map_err(|_| "Hosted API stopped unexpectedly.")
        }
        _ => Err("Hosted mode supports serve, migrate, or seed-local-workspace."),
    }
}
