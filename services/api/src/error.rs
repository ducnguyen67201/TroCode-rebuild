use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use uuid::Uuid;
#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: &'static str,
    pub correlation: Uuid,
    pub retryable: bool,
}
impl ApiError {
    pub fn new(
        status: StatusCode,
        code: &'static str,
        message: &'static str,
        correlation: Uuid,
        retryable: bool,
    ) -> Self {
        Self {
            status,
            code,
            message,
            correlation,
            retryable,
        }
    }
    pub fn unauthorized(id: Uuid) -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            "UNAUTHORIZED",
            "A valid development credential is required.",
            id,
            false,
        )
    }
    pub fn internal(id: Uuid) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL",
            "The request could not be completed.",
            id,
            false,
        )
    }
    pub fn invalid_google(id: Uuid) -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            "AUTH_INVALID_GOOGLE",
            "Google could not verify this sign-in.",
            id,
            false,
        )
    }
    pub fn session_expired(id: Uuid) -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            "SESSION_EXPIRED",
            "Your session has expired. Sign in again.",
            id,
            false,
        )
    }
    pub fn session_revoked(id: Uuid) -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            "SESSION_REVOKED",
            "Your session is no longer active. Sign in again.",
            id,
            false,
        )
    }
    pub fn session_replayed(id: Uuid) -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            "SESSION_REPLAYED",
            "This session can no longer be renewed. Sign in again.",
            id,
            false,
        )
    }
    pub fn identity_conflict(id: Uuid) -> Self {
        Self::new(
            StatusCode::CONFLICT,
            "AUTH_IDENTITY_CONFLICT",
            "This verified email needs account support before it can be used.",
            id,
            false,
        )
    }
    pub fn invalid_request(id: Uuid) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "INVALID_REQUEST",
            "The authentication request is invalid.",
            id,
            false,
        )
    }
    pub fn unavailable(id: Uuid) -> Self {
        Self::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "AUTH_UNAVAILABLE",
            "Authentication is temporarily unavailable. Try again.",
            id,
            true,
        )
    }
    pub fn provider_grant_conflict(id: Uuid) -> Self {
        Self::new(
            StatusCode::CONFLICT,
            "PROVIDER_GRANT_EXISTS",
            "Model access was already prepared for this instruction.",
            id,
            false,
        )
    }
    pub fn provider_unauthorized(id: Uuid) -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            "PROVIDER_GRANT_INVALID",
            "Model access has expired. Retry the instruction.",
            id,
            false,
        )
    }
    pub fn provider_budget(id: Uuid) -> Self {
        Self::new(
            StatusCode::TOO_MANY_REQUESTS,
            "PROVIDER_BUDGET_EXHAUSTED",
            "This instruction reached its model budget.",
            id,
            false,
        )
    }
    pub fn provider_sequence(id: Uuid) -> Self {
        Self::new(
            StatusCode::CONFLICT,
            "TRANSCRIPTION_SEQUENCE_INVALID",
            "The voice instruction is incomplete. Retry it.",
            id,
            false,
        )
    }
    pub fn provider_invalid(id: Uuid) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "PROVIDER_REQUEST_INVALID",
            "The model request is invalid.",
            id,
            false,
        )
    }
    pub fn provider_unavailable(id: Uuid) -> Self {
        Self::new(
            StatusCode::BAD_GATEWAY,
            "PROVIDER_UNAVAILABLE",
            "Model service is temporarily unavailable. Try again.",
            id,
            true,
        )
    }
    pub fn workspace_invalid_request(id: Uuid) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "WORKSPACE_INVALID_REQUEST",
            "A valid workspace membership request is required.",
            id,
            false,
        )
    }
    pub fn workspace_owner_required(id: Uuid) -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            "WORKSPACE_OWNER_REQUIRED",
            "Workspace owner access is required.",
            id,
            false,
        )
    }
    pub fn workspace_membership_conflict(id: Uuid) -> Self {
        Self::new(
            StatusCode::CONFLICT,
            "WORKSPACE_MEMBERSHIP_CONFLICT",
            "This workspace membership conflicts with an existing assignment.",
            id,
            false,
        )
    }
    pub fn workspace_membership_not_found(id: Uuid) -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            "WORKSPACE_MEMBERSHIP_NOT_FOUND",
            "The workspace membership was not found.",
            id,
            false,
        )
    }
    pub fn workspace_owner_removal_forbidden(id: Uuid) -> Self {
        Self::new(
            StatusCode::FORBIDDEN,
            "WORKSPACE_OWNER_REMOVAL_FORBIDDEN",
            "The workspace owner cannot be removed here.",
            id,
            false,
        )
    }
    pub fn workspace_member_limit_reached(id: Uuid) -> Self {
        Self::new(
            StatusCode::CONFLICT,
            "WORKSPACE_MEMBER_LIMIT_REACHED",
            "This workspace has reached its member limit.",
            id,
            false,
        )
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(json!({
                "code": self.code,
                "message": self.message,
                "retryable": self.retryable,
                "correlationId": self.correlation,
            })),
        )
            .into_response()
    }
}
