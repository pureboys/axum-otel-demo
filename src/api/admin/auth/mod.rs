//! 后台认证模块

pub mod dto;
pub mod service;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use axum::Json;
use axum::Extension;

use crate::app::AppState;
use crate::dto::response::ApiResponse;
use crate::error::AppError;
use crate::middleware::CurrentUser;

use dto::{AdminInfoResponse, LoginRequest, RefreshTokenRequest};

/// POST /admin/auth/login - 登录（公开）
#[tracing::instrument(skip_all, fields(username = %req.username))]
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    tracing::debug!("Admin login attempt for username: {}", req.username);

    let response = service::AuthService::login(&state, req.username, req.password).await?;

    tracing::info!(admin_id = response.admin.id, "Admin logged in successfully");

    Ok(ApiResponse::success_with_status(response, StatusCode::CREATED))
}

/// POST /admin/auth/logout - 登出（需认证）
#[tracing::instrument(skip_all)]
pub async fn logout(Extension(current_user): Extension<CurrentUser>) -> Result<impl IntoResponse, AppError> {
    tracing::info!(admin_id = current_user.id, "Admin logged out");
    Ok(ApiResponse::<()>::success_empty())
}

/// GET /admin/auth/info - 获取当前管理员信息（需认证）
#[tracing::instrument(skip_all)]
pub async fn info(Extension(current_user): Extension<CurrentUser>) -> Result<impl IntoResponse, AppError> {
    let response = AdminInfoResponse {
        id: current_user.id,
        username: current_user.username,
        nickname: current_user.nickname,
        role: current_user.role,
        created_at: current_user.created_at,
    };
    Ok(ApiResponse::success(response))
}

/// POST /admin/auth/refresh - 刷新 Token（需认证）
#[tracing::instrument(skip_all)]
pub async fn refresh(
    Extension(current_user): Extension<CurrentUser>,
    Json(_req): Json<RefreshTokenRequest>,
) -> Result<impl IntoResponse, AppError> {
    let claims = service::AuthService::verify_token(&current_user.token)?;
    let response = service::AuthService::refresh_token(&claims)?;
    Ok(ApiResponse::success(response))
}

/// 构建公开路由（无需认证）
pub fn public_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
}

/// 构建受保护路由（需认证）
pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/logout", post(logout))
        .route("/info", get(info))
        .route("/refresh", post(refresh))
}
