//! 后台分类管理模块

mod dto;
mod service;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use crate::app::AppState;
use crate::dto::response::ApiResponse;
use crate::error::AppError;
use service::CategoryService;
use dto::{CreateCategoryRequest, UpdateCategoryRequest};

/// GET /admin/categories - 获取所有分类
#[tracing::instrument(skip_all)]
pub async fn list_categories(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let categories = CategoryService::list_categories(&state).await?;
    Ok(ApiResponse::success(categories))
}

/// GET /admin/categories/:id - 获取分类详情
#[tracing::instrument(skip(state))]
pub async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let category = CategoryService::get_category(&state, id).await?;
    Ok(ApiResponse::success(category))
}

/// POST /admin/categories - 创建分类
#[tracing::instrument(skip_all)]
pub async fn create_category(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<CreateCategoryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let category = CategoryService::create_category(&state, payload).await?;
    Ok(ApiResponse::success(category))
}

/// PUT /admin/categories/:id - 更新分类
#[tracing::instrument(skip(state, payload))]
pub async fn update_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    axum::Json(payload): axum::Json<UpdateCategoryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let category = CategoryService::update_category(&state, id, payload).await?;
    Ok(ApiResponse::success(category))
}

/// DELETE /admin/categories/:id - 删除分类
#[tracing::instrument(skip(state))]
pub async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    CategoryService::delete_category(&state, id).await?;
    Ok(ApiResponse::<()>::success_empty())
}

/// 构建后台分类管理路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/categories", get(list_categories).post(create_category))
        .route("/categories/{id}", get(get_category).put(update_category).delete(delete_category))
}
