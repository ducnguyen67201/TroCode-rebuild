use crate::config::Config;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbErr,
    sea_query::{Alias, Expr, Func, Order, Query},
};
use std::time::Duration;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn connect(config: &Config) -> Result<DatabaseConnection, DbErr> {
    connect_url(&config.database_url).await
}

pub async fn connect_url(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(database_url);
    options
        .max_connections(5)
        .connect_timeout(Duration::from_secs(3))
        .acquire_timeout(Duration::from_secs(3))
        .sqlx_logging(false);
    Database::connect(options).await
}

async fn current_database(pool: &DatabaseConnection) -> Result<String, DbErr> {
    let statement = Query::select()
        .expr(Func::cust(Alias::new("current_database")))
        .to_owned();
    pool.query_one(pool.get_database_backend().build(&statement))
        .await?
        .ok_or_else(|| DbErr::RecordNotFound("database identity".into()))?
        .try_get_by_index(0)
}

async fn has_legacy_users(pool: &DatabaseConnection) -> Result<bool, DbErr> {
    let statement = Query::select()
        .expr(Expr::value(1))
        .from((Alias::new("information_schema"), Alias::new("tables")))
        .and_where(Expr::col(Alias::new("table_schema")).eq("public"))
        .and_where(Expr::col(Alias::new("table_name")).eq("users"))
        .limit(1)
        .to_owned();
    Ok(pool
        .query_one(pool.get_database_backend().build(&statement))
        .await?
        .is_some())
}

pub async fn validate_target(
    pool: &DatabaseConnection,
    expected_database: &str,
) -> Result<(), DbErr> {
    if current_database(pool).await? != expected_database || has_legacy_users(pool).await? {
        return Err(DbErr::Custom("Refusing an unknown database".into()));
    }
    Ok(())
}

pub async fn migrate(pool: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
    validate_target(pool, "tro_rebuild_test").await?;
    MIGRATOR.run(pool.get_postgres_connection_pool()).await?;
    Ok(())
}

pub async fn ready(pool: &DatabaseConnection) -> bool {
    let result = tokio::time::timeout(Duration::from_secs(2), async {
        let statement = Query::select()
            .columns([Alias::new("version"), Alias::new("checksum")])
            .from(Alias::new("_sqlx_migrations"))
            .and_where(Expr::col(Alias::new("success")).eq(true))
            .order_by(Alias::new("version"), Order::Asc)
            .to_owned();
        pool.query_all(pool.get_database_backend().build(&statement))
            .await?
            .into_iter()
            .map(|row| Ok((row.try_get_by_index(0)?, row.try_get_by_index(1)?)))
            .collect::<Result<Vec<(i64, Vec<u8>)>, DbErr>>()
    })
    .await;
    result.is_ok_and(|result| {
        result.is_ok_and(|rows| {
            let migrations: Vec<_> = MIGRATOR.iter().collect();
            rows.len() == migrations.len()
                && rows
                    .iter()
                    .zip(migrations)
                    .all(|((version, checksum), migration)| {
                        *version == migration.version && checksum == migration.checksum.as_ref()
                    })
        })
    })
}
