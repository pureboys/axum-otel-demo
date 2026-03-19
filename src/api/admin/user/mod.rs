//! 后台用户管理模块

mod dto;
mod service;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::routing::get;
use axum::Router;

use crate::app::AppState;
use crate::dto::response::ApiResponse;
use crate::error::AppError;

use dto::{CreateUserRequest, UpdateUserRequest};
use service::UserService;

/// GET /admin/users - 获取所有用户（后台管理）
#[tracing::instrument(skip_all)]
pub async fn list_users(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let users = UserService::list_users(&state).await?;
    Ok(ApiResponse::success(users))
}

/// GET /admin/users/:id - 获取用户详情（后台管理）
#[tracing::instrument(skip(state))]
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user = UserService::get_user(&state, id).await?;
    Ok(ApiResponse::success(user))
}

/// POST /admin/users - 创建用户（后台管理）
#[tracing::instrument(skip_all)]
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    tracing::debug!(username = %payload.username, "Creating new user");
    let user = UserService::create_user(&state, payload).await?;
    tracing::info!(user_id = user.id, "User created successfully");
    Ok(ApiResponse::success_with_status(user, StatusCode::CREATED))
}

/// PUT /admin/users/:id - 更新用户（后台管理）
#[tracing::instrument(skip(state, payload))]
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = UserService::update_user(&state, id, payload).await?;
    tracing::info!(user_id = user.id, "User updated");
    Ok(ApiResponse::success(user))
}

/// DELETE /admin/users/:id - 删除用户（后台管理）
#[tracing::instrument(skip(state))]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    UserService::delete_user(&state, id).await?;
    tracing::info!(user_id = id, "User deleted");
    Ok(ApiResponse::<()>::success_empty())
}

/// 构建后台用户管理路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
}
