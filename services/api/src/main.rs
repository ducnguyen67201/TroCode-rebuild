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
