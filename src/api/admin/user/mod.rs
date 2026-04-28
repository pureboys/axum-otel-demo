//! 后台管理员管理模块

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

use dto::{ChangePasswordRequest, CreateAdminRequest, UpdateAdminRequest};
use service::AdminUserService;

/// GET /admin/admins - 获取所有管理员
#[tracing::instrument(skip_all)]
pub async fn list_admins(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let admins = AdminUserService::list_admins(&state).await?;
    Ok(ApiResponse::success(admins))
}

/// GET /admin/admins/:id - 获取管理员详情
#[tracing::instrument(skip(state))]
pub async fn get_admin(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let admin = AdminUserService::get_admin(&state, id).await?;
    Ok(ApiResponse::success(admin))
}

/// POST /admin/admins - 创建管理员
#[tracing::instrument(skip_all)]
pub async fn create_admin(
    State(state): State<AppState>,
    Json(payload): Json<CreateAdminRequest>,
) -> Result<impl IntoResponse, AppError> {
    tracing::debug!(username = %payload.username, "Creating new admin");
    let admin = AdminUserService::create_admin(&state, payload).await?;
    tracing::info!(admin_id = admin.id, "Admin created successfully");
    Ok(ApiResponse::success_with_status(admin, StatusCode::CREATED))
}

/// PUT /admin/admins/:id - 更新管理员
#[tracing::instrument(skip(state, payload))]
pub async fn update_admin(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateAdminRequest>,
) -> Result<impl IntoResponse, AppError> {
    let admin = AdminUserService::update_admin(&state, id, payload).await?;
    tracing::info!(admin_id = admin.id, "Admin updated");
    Ok(ApiResponse::success(admin))
}

/// PUT /admin/admins/:id/password - 修改管理员密码
#[tracing::instrument(skip(state, payload))]
pub async fn change_admin_password(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<impl IntoResponse, AppError> {
    AdminUserService::change_password(&state, id, &payload.new_password).await?;
    tracing::info!(admin_id = id, "Admin password changed");
    Ok(ApiResponse::<()>::success_empty())
}

/// DELETE /admin/admins/:id - 删除管理员
#[tracing::instrument(skip(state))]
pub async fn delete_admin(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    AdminUserService::delete_admin(&state, id).await?;
    tracing::info!(admin_id = id, "Admin deleted");
    Ok(ApiResponse::<()>::success_empty())
}

/// 构建后台管理员管理路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/admins", get(list_admins).post(create_admin))
        .route("/admins/{id}", get(get_admin).put(update_admin).delete(delete_admin))
        .route("/admins/{id}/password", axum::routing::put(change_admin_password))
}
