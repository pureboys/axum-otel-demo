//! 后台标签管理模块

mod dto;
mod service;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use crate::app::AppState;
use crate::dto::response::ApiResponse;
use crate::error::AppError;
use service::TagService;
use dto::{CreateTagRequest, UpdateTagRequest};

/// GET /admin/tags - 获取所有标签
#[tracing::instrument(skip_all)]
pub async fn list_tags(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let tags = TagService::list_tags(&state).await?;
    Ok(ApiResponse::success(tags))
}

/// GET /admin/tags/:id - 获取标签详情
#[tracing::instrument(skip(state))]
pub async fn get_tag(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let tag = TagService::get_tag(&state, id).await?;
    Ok(ApiResponse::success(tag))
}

/// POST /admin/tags - 创建标签
#[tracing::instrument(skip_all)]
pub async fn create_tag(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<CreateTagRequest>,
) -> Result<impl IntoResponse, AppError> {
    let tag = TagService::create_tag(&state, payload).await?;
    Ok(ApiResponse::success(tag))
}

/// PUT /admin/tags/:id - 更新标签
#[tracing::instrument(skip(state, payload))]
pub async fn update_tag(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    axum::Json(payload): axum::Json<UpdateTagRequest>,
) -> Result<impl IntoResponse, AppError> {
    let tag = TagService::update_tag(&state, id, payload).await?;
    Ok(ApiResponse::success(tag))
}

/// DELETE /admin/tags/:id - 删除标签
#[tracing::instrument(skip(state))]
pub async fn delete_tag(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    TagService::delete_tag(&state, id).await?;
    Ok(ApiResponse::<()>::success_empty())
}

/// 构建后台标签管理路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/tags", get(list_tags).post(create_tag))
        .route("/tags/{id}", get(get_tag).put(update_tag).delete(delete_tag))
}
