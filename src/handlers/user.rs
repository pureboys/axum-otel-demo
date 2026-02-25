use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

use crate::app::AppState;
use crate::dto::user::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::error::AppError;
use crate::services::user::UserService;

/// GET /
#[tracing::instrument(skip_all)]
pub async fn root() -> &'static str {
    tracing::debug!("Handling root request");
    "Hello, World!"
}

/// GET /users
#[tracing::instrument(skip_all)]
pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, AppError> {
    let users = UserService::list_users(&state.db).await?;
    Ok(Json(users))
}

/// GET /users/:id
#[tracing::instrument(skip(state))]
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<UserResponse>, AppError> {
    let user = UserService::get_user(&state.db, id).await?;
    Ok(Json(user))
}

/// POST /users
#[tracing::instrument(skip_all)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    tracing::debug!(username = %payload.username, "Creating new user");
    let user = UserService::create_user(&state.db, payload).await?;
    tracing::info!(user_id = user.id, "User created successfully");
    Ok((StatusCode::CREATED, Json(user)))
}

/// PUT /users/:id
#[tracing::instrument(skip(state, payload))]
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, AppError> {
    let user = UserService::update_user(&state.db, id, payload).await?;
    tracing::info!(user_id = user.id, "User updated");
    Ok(Json(user))
}

/// DELETE /users/:id
#[tracing::instrument(skip(state))]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    UserService::delete_user(&state.db, id).await?;
    tracing::info!(user_id = id, "User deleted");
    Ok(StatusCode::NO_CONTENT)
}
