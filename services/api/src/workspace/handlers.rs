use super::{WorkspaceMember, WorkspaceMemberList};
use crate::{auth::handlers::bearer, error::ApiError, hosted::HostedState};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AddMemberRequest {
    email: String,
    role: String,
}

pub async fn list_members(
    State(state): State<HostedState>,
    Extension(correlation): Extension<Uuid>,
    headers: HeaderMap,
    Path(workspace_id): Path<Uuid>,
) -> Result<Json<WorkspaceMemberList>, ApiError> {
    let context = state
        .sessions
        .authenticate(bearer(&headers, correlation)?, correlation)
        .await?;
    Ok(Json(
        state
            .workspaces
            .list_members(context.account_id, workspace_id, correlation)
            .await?,
    ))
}

pub async fn add_member(
    State(state): State<HostedState>,
    Extension(correlation): Extension<Uuid>,
    headers: HeaderMap,
    Path(workspace_id): Path<Uuid>,
    Json(request): Json<AddMemberRequest>,
) -> Result<(StatusCode, Json<WorkspaceMember>), ApiError> {
    let context = state
        .sessions
        .authenticate(bearer(&headers, correlation)?, correlation)
        .await?;
    let member = state
        .workspaces
        .add_member(
            context.account_id,
            workspace_id,
            &request.email,
            &request.role,
            correlation,
        )
        .await?;
    tracing::info!(
        event = "workspace.member.added",
        correlation_id = %correlation,
        account_id = %context.account_id,
        workspace_id = %workspace_id,
        membership_id = %member.membership_id,
        role = %member.role,
    );
    Ok((StatusCode::CREATED, Json(member)))
}

pub async fn remove_member(
    State(state): State<HostedState>,
    Extension(correlation): Extension<Uuid>,
    headers: HeaderMap,
    Path((workspace_id, membership_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, ApiError> {
    let context = state
        .sessions
        .authenticate(bearer(&headers, correlation)?, correlation)
        .await?;
    state
        .workspaces
        .remove_member(context.account_id, workspace_id, membership_id, correlation)
        .await?;
    tracing::info!(
        event = "workspace.member.removed",
        correlation_id = %correlation,
        account_id = %context.account_id,
        workspace_id = %workspace_id,
        membership_id = %membership_id,
    );
    Ok(StatusCode::NO_CONTENT)
}
