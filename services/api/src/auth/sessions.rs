use super::{
    AuthUser, WorkspaceMembershipStore, WorkspaceSummary,
    google::VerifiedGoogleIdentity,
    jwt::{AuthContext, JwtService},
};
use crate::{
    entities::{account, auth_identity, auth_session},
    error::ApiError,
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use rand::RngCore;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set,
    TransactionTrait, sea_query::Expr,
};
use serde::Serialize;
use sha2::Sha256;
use std::{sync::Arc, time::Duration};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct SessionService {
    database: DatabaseConnection,
    jwt: JwtService,
    refresh_key: Arc<Vec<u8>>,
    refresh_ttl: Duration,
    memberships: Arc<dyn WorkspaceMembershipStore>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionEnvelope {
    pub access_token: String,
    pub access_token_expires_at: String,
    pub refresh_token: String,
    pub refresh_token_expires_at: String,
    pub account: AuthUser,
    pub workspaces: Vec<WorkspaceSummary>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeResponse {
    pub account: AuthUser,
    pub workspaces: Vec<WorkspaceSummary>,
}

impl SessionService {
    pub fn new(
        database: DatabaseConnection,
        jwt: JwtService,
        refresh_key: Vec<u8>,
        refresh_ttl: Duration,
        memberships: Arc<dyn WorkspaceMembershipStore>,
    ) -> Result<Self, &'static str> {
        if refresh_key.len() < 32 || refresh_ttl.is_zero() {
            return Err("Invalid refresh-session configuration.");
        }
        Ok(Self {
            database,
            jwt,
            refresh_key: Arc::new(refresh_key),
            refresh_ttl,
            memberships,
        })
    }

    pub async fn login(
        &self,
        identity: VerifiedGoogleIdentity,
        correlation: Uuid,
    ) -> Result<SessionEnvelope, ApiError> {
        let transaction = self
            .database
            .begin()
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        let now = OffsetDateTime::now_utc();
        let account = self
            .upsert_google_account(&transaction, identity, now, correlation)
            .await?;
        if account.status != "active" {
            return Err(ApiError::session_revoked(correlation));
        }
        let workspaces = self
            .memberships
            .claim_pending_memberships(&transaction, account.id, &account.email_normalized)
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        let family_id = Uuid::new_v4();
        let (session, refresh_token) = self
            .insert_session(&transaction, account.id, family_id, None, now, correlation)
            .await?;
        let envelope =
            self.issue_envelope(account, session, refresh_token, workspaces, correlation)?;
        transaction
            .commit()
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        Ok(envelope)
    }

    pub async fn refresh(
        &self,
        refresh_token: &str,
        correlation: Uuid,
    ) -> Result<SessionEnvelope, ApiError> {
        validate_refresh_token(refresh_token)
            .map_err(|_| ApiError::session_revoked(correlation))?;
        let token_digest = self.refresh_digest(refresh_token);
        let transaction = self
            .database
            .begin()
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        let now = OffsetDateTime::now_utc();
        let current = auth_session::Entity::find()
            .filter(auth_session::Column::TokenDigest.eq(token_digest))
            .lock_exclusive()
            .one(&transaction)
            .await
            .map_err(|_| ApiError::internal(correlation))?
            .ok_or_else(|| ApiError::session_revoked(correlation))?;

        if current.replaced_by_id.is_some() {
            auth_session::Entity::update_many()
                .col_expr(auth_session::Column::RevokedAt, Expr::value(now))
                .col_expr(auth_session::Column::ReuseDetectedAt, Expr::value(now))
                .filter(auth_session::Column::FamilyId.eq(current.family_id))
                .exec(&transaction)
                .await
                .map_err(|_| ApiError::internal(correlation))?;
            transaction
                .commit()
                .await
                .map_err(|_| ApiError::internal(correlation))?;
            return Err(ApiError::session_replayed(correlation));
        }
        if current.revoked_at.is_some() {
            return Err(ApiError::session_revoked(correlation));
        }
        if current.expires_at <= now {
            return Err(ApiError::session_expired(correlation));
        }
        let account = account::Entity::find_by_id(current.account_id)
            .one(&transaction)
            .await
            .map_err(|_| ApiError::internal(correlation))?
            .filter(|value| value.status == "active")
            .ok_or_else(|| ApiError::session_revoked(correlation))?;
        let workspaces = self
            .memberships
            .claim_pending_memberships(&transaction, account.id, &account.email_normalized)
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        let (replacement, next_refresh_token) = self
            .insert_session(
                &transaction,
                account.id,
                current.family_id,
                Some(current.id),
                now,
                correlation,
            )
            .await?;
        let mut old: auth_session::ActiveModel = current.into();
        old.replaced_by_id = Set(Some(replacement.id));
        old.revoked_at = Set(Some(now));
        old.last_used_at = Set(now);
        old.update(&transaction)
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        let envelope = self.issue_envelope(
            account,
            replacement,
            next_refresh_token,
            workspaces,
            correlation,
        )?;
        transaction
            .commit()
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        Ok(envelope)
    }

    pub async fn authenticate(
        &self,
        access_token: &str,
        correlation: Uuid,
    ) -> Result<AuthContext, ApiError> {
        let context = self
            .jwt
            .verify(access_token)
            .map_err(|_| ApiError::session_expired(correlation))?;
        let now = OffsetDateTime::now_utc();
        let session = auth_session::Entity::find_by_id(context.session_id)
            .one(&self.database)
            .await
            .map_err(|_| ApiError::internal(correlation))?
            .filter(|value| {
                value.account_id == context.account_id
                    && value.revoked_at.is_none()
                    && value.expires_at > now
            })
            .ok_or_else(|| ApiError::session_revoked(correlation))?;
        let active = account::Entity::find_by_id(session.account_id)
            .one(&self.database)
            .await
            .map_err(|_| ApiError::internal(correlation))?
            .is_some_and(|value| value.status == "active");
        if !active {
            return Err(ApiError::session_revoked(correlation));
        }
        Ok(context)
    }

    pub async fn current_account(
        &self,
        context: AuthContext,
        correlation: Uuid,
    ) -> Result<MeResponse, ApiError> {
        let transaction = self
            .database
            .begin()
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        let account = account::Entity::find_by_id(context.account_id)
            .one(&transaction)
            .await
            .map_err(|_| ApiError::internal(correlation))?
            .filter(|value| value.status == "active")
            .ok_or_else(|| ApiError::session_revoked(correlation))?;
        let workspaces = self
            .memberships
            .active_memberships(&transaction, account.id)
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        transaction
            .commit()
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        Ok(MeResponse {
            account: user(&account),
            workspaces,
        })
    }

    pub async fn logout(&self, context: AuthContext, correlation: Uuid) -> Result<(), ApiError> {
        let session = auth_session::Entity::find_by_id(context.session_id)
            .one(&self.database)
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        let Some(session) = session else {
            return Ok(());
        };
        let now = OffsetDateTime::now_utc();
        auth_session::Entity::update_many()
            .col_expr(auth_session::Column::RevokedAt, Expr::value(now))
            .filter(auth_session::Column::FamilyId.eq(session.family_id))
            .filter(auth_session::Column::RevokedAt.is_null())
            .exec(&self.database)
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        Ok(())
    }

    async fn upsert_google_account(
        &self,
        transaction: &sea_orm::DatabaseTransaction,
        identity: VerifiedGoogleIdentity,
        now: OffsetDateTime,
        correlation: Uuid,
    ) -> Result<account::Model, ApiError> {
        let existing_identity = auth_identity::Entity::find()
            .filter(auth_identity::Column::Provider.eq("google"))
            .filter(auth_identity::Column::ProviderSubject.eq(&identity.subject))
            .one(transaction)
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        if let Some(existing_identity) = existing_identity {
            let account = account::Entity::find_by_id(existing_identity.account_id)
                .one(transaction)
                .await
                .map_err(|_| ApiError::internal(correlation))?
                .ok_or_else(|| ApiError::session_revoked(correlation))?;
            if account.email_normalized != identity.email_normalized {
                let collision = account::Entity::find()
                    .filter(account::Column::EmailNormalized.eq(&identity.email_normalized))
                    .filter(account::Column::Id.ne(account.id))
                    .one(transaction)
                    .await
                    .map_err(|_| ApiError::internal(correlation))?
                    .is_some();
                if collision {
                    return Err(ApiError::identity_conflict(correlation));
                }
            }
            let mut updated: account::ActiveModel = account.into();
            updated.display_name = Set(identity.display_name);
            updated.verified_email = Set(identity.email);
            updated.email_normalized = Set(identity.email_normalized);
            updated.updated_at = Set(now);
            return updated
                .update(transaction)
                .await
                .map_err(|_| ApiError::identity_conflict(correlation));
        }

        if account::Entity::find()
            .filter(account::Column::EmailNormalized.eq(&identity.email_normalized))
            .one(transaction)
            .await
            .map_err(|_| ApiError::internal(correlation))?
            .is_some()
        {
            return Err(ApiError::identity_conflict(correlation));
        }
        let account_id = Uuid::new_v4();
        let account = account::ActiveModel {
            id: Set(account_id),
            display_name: Set(identity.display_name),
            verified_email: Set(identity.email),
            email_normalized: Set(identity.email_normalized),
            status: Set("active".to_owned()),
            created_at: Set(now),
            updated_at: Set(now),
        }
        .insert(transaction)
        .await
        .map_err(|_| ApiError::identity_conflict(correlation))?;
        auth_identity::ActiveModel {
            id: Set(Uuid::new_v4()),
            account_id: Set(account_id),
            provider: Set("google".to_owned()),
            provider_subject: Set(identity.subject),
            issuer: Set(identity.issuer),
            created_at: Set(now),
            updated_at: Set(now),
        }
        .insert(transaction)
        .await
        .map_err(|_| ApiError::identity_conflict(correlation))?;
        Ok(account)
    }

    async fn insert_session(
        &self,
        transaction: &sea_orm::DatabaseTransaction,
        account_id: Uuid,
        family_id: Uuid,
        parent_id: Option<Uuid>,
        now: OffsetDateTime,
        correlation: Uuid,
    ) -> Result<(auth_session::Model, String), ApiError> {
        let refresh_token = generate_refresh_token();
        let expires_at = now + time::Duration::seconds(self.refresh_ttl.as_secs() as i64);
        let session = auth_session::ActiveModel {
            id: Set(Uuid::new_v4()),
            account_id: Set(account_id),
            family_id: Set(family_id),
            parent_id: Set(parent_id),
            replaced_by_id: Set(None),
            token_digest: Set(self.refresh_digest(&refresh_token)),
            created_at: Set(now),
            last_used_at: Set(now),
            expires_at: Set(expires_at),
            revoked_at: Set(None),
            reuse_detected_at: Set(None),
        }
        .insert(transaction)
        .await
        .map_err(|_| ApiError::internal(correlation))?;
        Ok((session, refresh_token))
    }

    fn issue_envelope(
        &self,
        account: account::Model,
        session: auth_session::Model,
        refresh_token: String,
        workspaces: Vec<WorkspaceSummary>,
        correlation: Uuid,
    ) -> Result<SessionEnvelope, ApiError> {
        let access = self
            .jwt
            .issue(account.id, session.id)
            .map_err(|_| ApiError::internal(correlation))?;
        Ok(SessionEnvelope {
            access_token: access.token,
            access_token_expires_at: format_time(access.expires_at),
            refresh_token,
            refresh_token_expires_at: format_time(session.expires_at),
            account: user(&account),
            workspaces,
        })
    }

    fn refresh_digest(&self, token: &str) -> Vec<u8> {
        let mut mac = HmacSha256::new_from_slice(&self.refresh_key)
            .expect("validated HMAC key has a supported length");
        mac.update(token.as_bytes());
        mac.finalize().into_bytes().to_vec()
    }
}

fn user(account: &account::Model) -> AuthUser {
    AuthUser {
        account_id: account.id,
        display_name: account.display_name.clone(),
        email: account.verified_email.clone(),
    }
}

fn generate_refresh_token() -> String {
    let mut bytes = [0_u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    format!("tro_refresh_{}", URL_SAFE_NO_PAD.encode(bytes))
}

fn validate_refresh_token(token: &str) -> Result<(), ()> {
    let encoded = token.strip_prefix("tro_refresh_").ok_or(())?;
    if encoded.len() != 43
        || !encoded
            .bytes()
            .all(|value| value.is_ascii_alphanumeric() || matches!(value, b'-' | b'_'))
    {
        return Err(());
    }
    Ok(())
}

fn format_time(value: OffsetDateTime) -> String {
    value.format(&Rfc3339).expect("UTC timestamp formatting")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refresh_tokens_have_fixed_random_shape() {
        let first = generate_refresh_token();
        let second = generate_refresh_token();
        assert_ne!(first, second);
        assert_eq!(first.len(), 55);
        assert!(validate_refresh_token(&first).is_ok());
        assert!(validate_refresh_token("tro_refresh_short").is_err());
    }
}
