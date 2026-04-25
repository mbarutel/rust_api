use crate::{application::error::AppError, domain::error::DomainError};
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug)]
pub struct HandlerError(pub AppError);

impl From<AppError> for HandlerError {
    fn from(e: AppError) -> Self {
        HandlerError(e)
    }
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self.0 {
            AppError::Domain(DomainError::NotFound) => (StatusCode::NOT_FOUND, "not found"),
            AppError::Domain(DomainError::Conflict) => (StatusCode::CONFLICT, "already exists"),
            AppError::Domain(DomainError::InvalidTransition(msg)) => (StatusCode::UNPROCESSABLE_ENTITY, msg.as_str()),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.as_str()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "forbidden"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "internal error"),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
