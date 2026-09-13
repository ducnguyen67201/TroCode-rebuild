use crate::config::Config;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");
pub async fn connect(config: &Config) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
}
pub async fn migrate(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let name: String = sqlx::query_scalar("SELECT current_database()")
        .fetch_one(pool)
        .await?;
    let legacy: bool = sqlx::query_scalar("SELECT to_regclass('public.users') IS NOT NULL")
        .fetch_one(pool)
        .await?;
    if name != "tro_rebuild_test" || legacy {
        return Err("Refusing to migrate an unknown database".into());
    }
    MIGRATOR.run(pool).await?;
    Ok(())
}
pub async fn ready(pool: &PgPool) -> bool {
    let result = tokio::time::timeout(
        Duration::from_secs(2),
        sqlx::query_scalar::<_, Vec<u8>>(
            "SELECT checksum FROM _sqlx_migrations WHERE version=1 AND success",
        )
        .fetch_one(pool),
    )
    .await;
    result.is_ok_and(|result| {
        result.is_ok_and(|checksum| {
            MIGRATOR
                .iter()
                .next()
                .is_some_and(|migration| migration.checksum.as_ref() == checksum)
        })
    })
}
