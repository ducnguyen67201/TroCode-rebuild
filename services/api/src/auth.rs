use crate::error::ApiError;
use axum::http::HeaderMap;
use serde::Serialize;
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};
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
    pool: &PgPool,
    headers: &HeaderMap,
    correlation: Uuid,
) -> Result<Account, ApiError> {
    let token = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|value| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .ok_or(ApiError::unauthorized(correlation))?;
    let row = sqlx::query("SELECT a.id, a.role FROM fixture_sessions s JOIN fixture_accounts a ON a.id=s.account_id WHERE s.token_digest=$1 AND s.expires_at > NOW() AND NOT s.revoked")
        .bind(digest(token)).fetch_optional(pool).await.map_err(|_| ApiError::internal(correlation))?.ok_or(ApiError::unauthorized(correlation))?;
    Ok(Account {
        account_id: row.get("id"),
        role: row.get("role"),
    })
}
pub async fn seed(
    pool: &PgPool,
    profiles: &serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tx = pool.begin().await?;
    for (index, profile) in ["teacher", "student-a", "student-b"].iter().enumerate() {
        let token = profiles[profile]
            .as_str()
            .filter(|token| token.len() == 64 && token.bytes().all(|b| b.is_ascii_hexdigit()))
            .ok_or("Invalid fixture credential file")?;
        let id = Uuid::from_u128(index as u128 + 1);
        let role = if index == 0 { "teacher" } else { "student" };
        sqlx::query("INSERT INTO fixture_accounts (id,profile,role) VALUES ($1,$2,$3) ON CONFLICT (id) DO NOTHING").bind(id).bind(profile).bind(role).execute(&mut *tx).await?;
        // Re-seeding never revives revoked/expired sessions or rewrites account state.
        sqlx::query("INSERT INTO fixture_sessions (token_digest,account_id,expires_at) VALUES ($1,$2,NOW()+INTERVAL '30 days') ON CONFLICT DO NOTHING").bind(digest(token)).bind(id).execute(&mut *tx).await?;
    }
    tx.commit().await?;
    Ok(())
}
