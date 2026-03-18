//! 后台用户管理模块

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use crate::app::AppState;
use crate::dto::response::ApiResponse;
use crate::error::AppError;
use crate::api::front::user::{service::UserService, dto::{CreateUserRequest, UpdateUserRequest}};

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
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let user = UserService::get_user(&state, id).await?;
    Ok(ApiResponse::success(user))
}

/// POST /admin/users - 创建用户（后台管理）
#[tracing::instrument(skip_all)]
pub async fn create_user(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = UserService::create_user(&state, payload).await?;
    Ok(ApiResponse::success(user))
}

/// PUT /admin/users/:id - 更新用户（后台管理）
#[tracing::instrument(skip(state, payload))]
pub async fn update_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    axum::Json(payload): axum::Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user = UserService::update_user(&state, id, payload).await?;
    Ok(ApiResponse::success(user))
}

/// DELETE /admin/users/:id - 删除用户（后台管理）
#[tracing::instrument(skip(state))]
pub async fn delete_user(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    UserService::delete_user(&state, id).await?;
    Ok(ApiResponse::<()>::success_empty())
}

/// 构建后台用户管理路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
}
