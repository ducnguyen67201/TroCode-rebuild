use crate::{
    entities::{fixture_account, fixture_session},
    error::ApiError,
};
use axum::http::HeaderMap;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TransactionTrait,
    entity::prelude::ChronoDateTimeUtc, sea_query::Expr,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use uuid::Uuid;
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_id: Uuid,
    pub role: String,
}
pub fn digest(token: &str) -> Vec<u8> {
    Sha256::digest(token.as_bytes()).to_vec()
}
pub async fn authenticate(
    pool: &DatabaseConnection,
    headers: &HeaderMap,
    correlation: Uuid,
) -> Result<Account, ApiError> {
    let token = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|value| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .ok_or(ApiError::unauthorized(correlation))?;
    let (_, account) = fixture_session::Entity::find()
        .filter(fixture_session::Column::TokenDigest.eq(digest(token)))
        .filter(fixture_session::Column::Revoked.eq(false))
        .filter(Expr::col(fixture_session::Column::ExpiresAt).gt(Expr::current_timestamp()))
        .find_also_related(fixture_account::Entity)
        .one(pool)
        .await
        .map_err(|_| ApiError::internal(correlation))?
        .ok_or(ApiError::unauthorized(correlation))?;
    let account = account.ok_or(ApiError::unauthorized(correlation))?;
    Ok(Account {
        account_id: account.id,
        role: account.role,
    })
}
pub async fn seed(
    pool: &DatabaseConnection,
    profiles: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let tx = pool.begin().await?;
    let now: ChronoDateTimeUtc = std::time::SystemTime::now().into();
    let expires_at = now + std::time::Duration::from_secs(30 * 24 * 60 * 60);
    for (index, profile) in ["teacher", "student-a", "student-b"].iter().enumerate() {
        let token = profiles[profile]
            .as_str()
            .filter(|token| token.len() == 64 && token.bytes().all(|b| b.is_ascii_hexdigit()))
            .ok_or("Invalid fixture credential file")?;
        let id = Uuid::from_u128(index as u128 + 1);
        let role = if index == 0 { "teacher" } else { "student" };
        fixture_account::Entity::insert(fixture_account::ActiveModel {
            id: Set(id),
            profile: Set((*profile).to_owned()),
            role: Set(role.to_owned()),
        })
        .on_conflict_do_nothing()
        .exec(&tx)
        .await?;
        // Re-seeding never revives revoked/expired sessions or rewrites account state.
        fixture_session::Entity::insert(fixture_session::ActiveModel {
            token_digest: Set(digest(token)),
            account_id: Set(id),
            expires_at: Set(expires_at),
            revoked: Set(false),
        })
        .on_conflict_do_nothing()
        .exec(&tx)
        .await?;
    }
    tx.commit().await?;
    Ok(())
}
