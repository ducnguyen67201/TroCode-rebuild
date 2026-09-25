use sea_orm_migration::{MigratorTrait, sea_orm::Database};
use tro_api_migration::Migrator;

#[tokio::main]
async fn main() {
    if std::env::var("TRO_ALLOW_HOSTED_MIGRATION").as_deref() != Ok("1") {
        eprintln!("Set TRO_ALLOW_HOSTED_MIGRATION=1 for the intended hosted database.");
        std::process::exit(1);
    }
    let database_url = std::env::var("DATABASE_URL").unwrap_or_default();
    if database_url.is_empty() {
        eprintln!("DATABASE_URL is required.");
        std::process::exit(1);
    }
    let result = async {
        let database = Database::connect(database_url).await?;
        Migrator::up(&database, None).await
    }
    .await;
    if result.is_err() {
        eprintln!("Hosted migration failed.");
        std::process::exit(1);
    }
}
