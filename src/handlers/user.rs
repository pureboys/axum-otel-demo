use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::app::AppState;
use crate::dto::response::ApiResponse;
use crate::dto::user::{CreateUserRequest, UpdateUserRequest};
use crate::error::AppError;
use crate::services::user::UserService;

/// GET /
#[tracing::instrument(skip_all)]
pub async fn root() -> impl IntoResponse {
    ApiResponse::success("Hello, World!")
}

/// GET /users
#[tracing::instrument(skip_all)]
pub async fn list_users(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let users = UserService::list_users(&state.db).await?;
    Ok(ApiResponse::success(users))
}

/// GET /users/:id
#[tracing::instrument(skip(state))]
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user = UserService::get_user(&state.db, id).await?;
    Ok(ApiResponse::success(user))
}

/// POST /users
#[tracing::instrument(skip_all)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    tracing::debug!(username = %payload.username, "Creating new user");
    let user = UserService::create_user(&state.db, payload).await?;
    tracing::info!(user_id = user.id, "User created successfully");
    Ok(ApiResponse::success_with_status(user, StatusCode::CREATED))
}

/// PUT /users/:id
#[tracing::instrument(skip(state, payload))]
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = UserService::update_user(&state.db, id, payload).await?;
    tracing::info!(user_id = user.id, "User updated");
    Ok(ApiResponse::success(user))
}

/// DELETE /users/:id
#[tracing::instrument(skip(state))]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    UserService::delete_user(&state.db, id).await?;
    tracing::info!(user_id = id, "User deleted");
    Ok(ApiResponse::<()>::success_empty())
}
