use sea_orm_migration::prelude::*;

mod m20260925_000001_google_auth;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260925_000001_google_auth::Migration)]
    }
}

pub async fn migrate(
    database: &sea_orm_migration::sea_orm::DatabaseConnection,
) -> Result<(), sea_orm_migration::sea_orm::DbErr> {
    Migrator::up(database, None).await
}
