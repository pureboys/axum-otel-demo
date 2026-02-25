use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::dto::response::ApiResponse;

/// 统一错误类型
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Validation(String),
    Internal(String),
    Database(sea_orm::DbErr),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, 404, msg.clone()),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, 400, msg.clone()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, 500, msg.clone()),
            AppError::Database(err) => {
                tracing::error!(error = %err, "Database error");
                (StatusCode::INTERNAL_SERVER_ERROR, 500, "Internal server error".into())
            }
        };

        (status, Json(ApiResponse::<()>::error(code, message))).into_response()
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::Database(err)
    }
}
