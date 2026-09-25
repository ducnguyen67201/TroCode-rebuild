use sea_orm_migration::prelude::*;

mod m20260925_000001_google_auth;
mod m20260925_000002_workspace_membership;
mod m20260925_000003_provider_grants;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260925_000001_google_auth::Migration),
            Box::new(m20260925_000002_workspace_membership::Migration),
            Box::new(m20260925_000003_provider_grants::Migration),
        ]
    }
}

pub async fn migrate(
    database: &sea_orm_migration::sea_orm::DatabaseConnection,
) -> Result<(), sea_orm_migration::sea_orm::DbErr> {
    Migrator::up(database, None).await
}
