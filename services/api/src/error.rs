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
}
impl ApiError {
    pub fn unauthorized(id: Uuid) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: "UNAUTHORIZED",
            message: "A valid development credential is required.",
            correlation: id,
        }
    }
    pub fn internal(id: Uuid) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "INTERNAL",
            message: "The request could not be completed.",
            correlation: id,
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(json!({"code": self.code, "message": self.message, "retryable": false, "correlationId": self.correlation}))).into_response()
    }
}
