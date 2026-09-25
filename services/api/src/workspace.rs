pub mod handlers;

use crate::{
    auth::{WorkspaceMembershipStore, WorkspaceSummary},
    entities::{account, workspace, workspace_audit_event, workspace_membership},
    error::ApiError,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect, Set, TransactionTrait,
};
use serde::Serialize;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use uuid::Uuid;

const MAX_MEMBERS: u64 = 500;

#[derive(Clone)]
pub struct WorkspaceService {
    database: DatabaseConnection,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceMember {
    pub membership_id: Uuid,
    pub email: String,
    pub display_name: Option<String>,
    pub role: String,
    pub state: &'static str,
    pub joined_at: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceMemberList {
    pub workspace: WorkspaceSummary,
    pub members: Vec<WorkspaceMember>,
}

impl WorkspaceService {
    #[must_use]
    pub fn new(database: DatabaseConnection) -> Self {
        Self { database }
    }

    pub async fn list_members(
        &self,
        actor_account_id: Uuid,
        workspace_id: Uuid,
        correlation: Uuid,
    ) -> Result<WorkspaceMemberList, ApiError> {
        let transaction = self
            .database
            .begin()
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        let workspace = require_owner(
            &transaction,
            actor_account_id,
            workspace_id,
            false,
            correlation,
        )
        .await?;
        let members = member_rows(&transaction, workspace_id, correlation).await?;
        transaction
            .commit()
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        Ok(WorkspaceMemberList { workspace, members })
    }

    pub async fn add_member(
        &self,
        actor_account_id: Uuid,
        workspace_id: Uuid,
        email: &str,
        role: &str,
        correlation: Uuid,
    ) -> Result<WorkspaceMember, ApiError> {
        let (email, email_normalized) = normalize_email(email)
            .ok_or_else(|| ApiError::workspace_invalid_request(correlation))?;
        if !matches!(role, "teacher" | "student") {
            return Err(ApiError::workspace_invalid_request(correlation));
        }
        let transaction = self
            .database
            .begin()
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        require_owner(
            &transaction,
            actor_account_id,
            workspace_id,
            true,
            correlation,
        )
        .await?;

        if let Some(existing) = workspace_membership::Entity::find()
            .filter(workspace_membership::Column::WorkspaceId.eq(workspace_id))
            .filter(workspace_membership::Column::EmailNormalized.eq(&email_normalized))
            .filter(workspace_membership::Column::RemovedAt.is_null())
            .one(&transaction)
            .await
            .map_err(|_| ApiError::internal(correlation))?
        {
            if existing.role != role {
                return Err(ApiError::workspace_membership_conflict(correlation));
            }
            let member = member_from_model(&transaction, existing, correlation).await?;
            transaction
                .commit()
                .await
                .map_err(|_| ApiError::internal(correlation))?;
            return Ok(member);
        }

        let now = OffsetDateTime::now_utc();
        let inserted = workspace_membership::ActiveModel {
            id: Set(Uuid::new_v4()),
            workspace_id: Set(workspace_id),
            account_id: Set(None),
            email: Set(email),
            email_normalized: Set(email_normalized),
            role: Set(role.to_owned()),
            added_by_account_id: Set(actor_account_id),
            created_at: Set(now),
            joined_at: Set(None),
            removed_at: Set(None),
        }
        .insert(&transaction)
        .await
        .map_err(|_| ApiError::workspace_membership_conflict(correlation))?;
        insert_audit(
            &transaction,
            workspace_id,
            actor_account_id,
            inserted.id,
            "workspace.member_added",
            now,
        )
        .await
        .map_err(|_| ApiError::internal(correlation))?;
        let member = workspace_member(&inserted, None);
        transaction
            .commit()
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        Ok(member)
    }

    pub async fn remove_member(
        &self,
        actor_account_id: Uuid,
        workspace_id: Uuid,
        membership_id: Uuid,
        correlation: Uuid,
    ) -> Result<(), ApiError> {
        let transaction = self
            .database
            .begin()
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        require_owner(
            &transaction,
            actor_account_id,
            workspace_id,
            true,
            correlation,
        )
        .await?;
        let target = workspace_membership::Entity::find_by_id(membership_id)
            .filter(workspace_membership::Column::WorkspaceId.eq(workspace_id))
            .filter(workspace_membership::Column::RemovedAt.is_null())
            .lock_exclusive()
            .one(&transaction)
            .await
            .map_err(|_| ApiError::internal(correlation))?
            .ok_or_else(|| ApiError::workspace_membership_not_found(correlation))?;
        if target.role == "owner" {
            return Err(ApiError::workspace_owner_removal_forbidden(correlation));
        }
        let now = OffsetDateTime::now_utc();
        let target_id = target.id;
        let mut updated: workspace_membership::ActiveModel = target.into();
        updated.removed_at = Set(Some(now));
        updated
            .update(&transaction)
            .await
            .map_err(|_| ApiError::internal(correlation))?;
        insert_audit(
            &transaction,
            workspace_id,
            actor_account_id,
            target_id,
            "workspace.member_removed",
            now,
        )
        .await
        .map_err(|_| ApiError::internal(correlation))?;
        transaction
            .commit()
            .await
            .map_err(|_| ApiError::internal(correlation))
    }
}

#[async_trait::async_trait]
impl WorkspaceMembershipStore for WorkspaceService {
    async fn claim_pending_memberships(
        &self,
        transaction: &DatabaseTransaction,
        account_id: Uuid,
        verified_email_normalized: &str,
    ) -> Result<Vec<WorkspaceSummary>, sea_orm::DbErr> {
        let pending = workspace_membership::Entity::find()
            .filter(workspace_membership::Column::EmailNormalized.eq(verified_email_normalized))
            .filter(workspace_membership::Column::AccountId.is_null())
            .filter(workspace_membership::Column::RemovedAt.is_null())
            .order_by_asc(workspace_membership::Column::WorkspaceId)
            .lock_exclusive()
            .all(transaction)
            .await?;
        let now = OffsetDateTime::now_utc();
        for membership in pending {
            let membership_id = membership.id;
            let workspace_id = membership.workspace_id;
            let mut claimed: workspace_membership::ActiveModel = membership.into();
            claimed.account_id = Set(Some(account_id));
            claimed.joined_at = Set(Some(now));
            claimed.update(transaction).await?;
            insert_audit(
                transaction,
                workspace_id,
                account_id,
                membership_id,
                "workspace.member_claimed",
                now,
            )
            .await?;
        }
        active_summaries(transaction, account_id).await
    }

    async fn active_memberships(
        &self,
        transaction: &DatabaseTransaction,
        account_id: Uuid,
    ) -> Result<Vec<WorkspaceSummary>, sea_orm::DbErr> {
        active_summaries(transaction, account_id).await
    }
}

async fn active_summaries(
    transaction: &DatabaseTransaction,
    account_id: Uuid,
) -> Result<Vec<WorkspaceSummary>, sea_orm::DbErr> {
    let rows = workspace_membership::Entity::find()
        .filter(workspace_membership::Column::AccountId.eq(account_id))
        .filter(workspace_membership::Column::RemovedAt.is_null())
        .find_also_related(workspace::Entity)
        .filter(workspace::Column::Status.eq("active"))
        .order_by_asc(workspace::Column::Name)
        .order_by_asc(workspace::Column::Id)
        .all(transaction)
        .await?;
    Ok(rows
        .into_iter()
        .filter_map(|(membership, workspace)| {
            workspace.map(|workspace| WorkspaceSummary {
                workspace_id: workspace.id,
                name: workspace.name,
                role: membership.role,
            })
        })
        .collect())
}

async fn require_owner(
    transaction: &DatabaseTransaction,
    account_id: Uuid,
    workspace_id: Uuid,
    lock_workspace: bool,
    correlation: Uuid,
) -> Result<WorkspaceSummary, ApiError> {
    let mut workspace_query =
        workspace::Entity::find_by_id(workspace_id).filter(workspace::Column::Status.eq("active"));
    if lock_workspace {
        workspace_query = workspace_query.lock_exclusive();
    }
    let workspace = workspace_query
        .one(transaction)
        .await
        .map_err(|_| ApiError::internal(correlation))?
        .ok_or_else(|| ApiError::workspace_owner_required(correlation))?;
    let membership = workspace_membership::Entity::find()
        .filter(workspace_membership::Column::WorkspaceId.eq(workspace_id))
        .filter(workspace_membership::Column::AccountId.eq(account_id))
        .filter(workspace_membership::Column::Role.eq("owner"))
        .filter(workspace_membership::Column::RemovedAt.is_null())
        .one(transaction)
        .await
        .map_err(|_| ApiError::internal(correlation))?
        .ok_or_else(|| ApiError::workspace_owner_required(correlation))?;
    Ok(WorkspaceSummary {
        workspace_id: workspace.id,
        name: workspace.name,
        role: membership.role,
    })
}

async fn member_rows(
    transaction: &DatabaseTransaction,
    workspace_id: Uuid,
    correlation: Uuid,
) -> Result<Vec<WorkspaceMember>, ApiError> {
    let rows = workspace_membership::Entity::find()
        .filter(workspace_membership::Column::WorkspaceId.eq(workspace_id))
        .filter(workspace_membership::Column::RemovedAt.is_null())
        .find_also_related(account::Entity)
        .order_by_asc(workspace_membership::Column::Role)
        .order_by_asc(workspace_membership::Column::CreatedAt)
        .order_by_asc(workspace_membership::Column::Id)
        .limit(MAX_MEMBERS)
        .all(transaction)
        .await
        .map_err(|_| ApiError::internal(correlation))?;
    Ok(rows
        .into_iter()
        .map(|(membership, account)| workspace_member(&membership, account.as_ref()))
        .collect())
}

async fn member_from_model(
    transaction: &DatabaseTransaction,
    membership: workspace_membership::Model,
    correlation: Uuid,
) -> Result<WorkspaceMember, ApiError> {
    let account = match membership.account_id {
        Some(account_id) => account::Entity::find_by_id(account_id)
            .one(transaction)
            .await
            .map_err(|_| ApiError::internal(correlation))?,
        None => None,
    };
    Ok(workspace_member(&membership, account.as_ref()))
}

fn workspace_member(
    membership: &workspace_membership::Model,
    account: Option<&account::Model>,
) -> WorkspaceMember {
    WorkspaceMember {
        membership_id: membership.id,
        email: membership.email.clone(),
        display_name: account.map(|account| account.display_name.clone()),
        role: membership.role.clone(),
        state: if membership.account_id.is_some() {
            "active"
        } else {
            "pending"
        },
        joined_at: membership.joined_at.map(format_time),
    }
}

async fn insert_audit(
    transaction: &DatabaseTransaction,
    workspace_id: Uuid,
    actor_account_id: Uuid,
    target_membership_id: Uuid,
    action: &str,
    created_at: OffsetDateTime,
) -> Result<(), sea_orm::DbErr> {
    workspace_audit_event::ActiveModel {
        id: Set(Uuid::new_v4()),
        workspace_id: Set(workspace_id),
        actor_account_id: Set(actor_account_id),
        target_membership_id: Set(target_membership_id),
        action: Set(action.to_owned()),
        created_at: Set(created_at),
    }
    .insert(transaction)
    .await?;
    Ok(())
}

fn normalize_email(value: &str) -> Option<(String, String)> {
    let email = value.trim();
    let normalized = email.to_lowercase();
    let (local, domain) = normalized.split_once('@')?;
    if !(3..=254).contains(&email.len())
        || normalized.matches('@').count() != 1
        || local.is_empty()
        || domain.is_empty()
        || domain.starts_with('.')
        || domain.ends_with('.')
        || normalized.bytes().any(|value| value.is_ascii_whitespace())
    {
        return None;
    }
    Some((email.to_owned(), normalized))
}

fn format_time(value: OffsetDateTime) -> String {
    value.format(&Rfc3339).expect("UTC timestamp formatting")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_normalization_is_exact_and_provider_agnostic() {
        assert_eq!(
            normalize_email(" Student.Name+lab@GMAIL.com "),
            Some((
                "Student.Name+lab@GMAIL.com".to_owned(),
                "student.name+lab@gmail.com".to_owned(),
            ))
        );
        assert!(normalize_email("missing-domain").is_none());
        assert!(normalize_email("two@@example.com").is_none());
        assert!(normalize_email("white space@example.com").is_none());
    }
}
