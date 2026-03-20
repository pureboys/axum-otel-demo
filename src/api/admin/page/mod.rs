//! 后台单页面管理模块

mod dto;
mod service;

use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

use crate::app::AppState;
use crate::dto::response::ApiResponse;
use crate::error::AppError;
use service::PageService;
use dto::{CreatePageRequest, UpdatePageRequest};

/// 分页查询参数
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<i8>,
}

/// GET /admin/pages - 获取所有页面（支持分页）
#[tracing::instrument(skip_all)]
pub async fn list_pages(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(10).min(100);
    let pages = PageService::list_pages_paginated(&state, page, limit, query.status).await?;
    Ok(ApiResponse::success(pages))
}

/// GET /admin/pages/:id - 获取页面详情
#[tracing::instrument(skip(state))]
pub async fn get_page(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let page = PageService::get_page(&state, id).await?;
    Ok(ApiResponse::success(page))
}

/// POST /admin/pages - 创建页面
#[tracing::instrument(skip_all)]
pub async fn create_page(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<CreatePageRequest>,
) -> Result<impl IntoResponse, AppError> {
    let page = PageService::create_page(&state, payload).await?;
    Ok(ApiResponse::success(page))
}

/// PUT /admin/pages/:id - 更新页面
#[tracing::instrument(skip(state, payload))]
pub async fn update_page(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    axum::Json(payload): axum::Json<UpdatePageRequest>,
) -> Result<impl IntoResponse, AppError> {
    let page = PageService::update_page(&state, id, payload).await?;
    Ok(ApiResponse::success(page))
}

/// DELETE /admin/pages/:id - 删除页面
#[tracing::instrument(skip(state))]
pub async fn delete_page(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    PageService::delete_page(&state, id).await?;
    Ok(ApiResponse::<()>::success_empty())
}

/// 构建后台单页面管理路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/pages", get(list_pages).post(create_page))
        .route("/pages/{id}", get(get_page).put(update_page).delete(delete_page))
}
