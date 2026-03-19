//! 后台新闻管理模块

mod dto;
mod service;

use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

use crate::app::AppState;
use crate::dto::response::ApiResponse;
use crate::error::AppError;
use service::NewsService;
use dto::{CreateNewsRequest, UpdateNewsRequest};

/// 分页查询参数
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub category_id: Option<i32>,
    pub status: Option<i8>,
}

/// GET /admin/news - 获取新闻列表（支持分页）
#[tracing::instrument(skip_all)]
pub async fn list_news(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(10).min(100);
    let news_list = NewsService::list_news_paginated(
        &state,
        page,
        limit,
        query.category_id,
        query.status,
    )
    .await?;
    Ok(ApiResponse::success(news_list))
}

/// GET /admin/news/all - 获取所有新闻（不分页）
#[tracing::instrument(skip_all)]
pub async fn list_all_news(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let news_list = NewsService::list_news(&state).await?;
    Ok(ApiResponse::success(news_list))
}

/// GET /admin/news/:id - 获取新闻详情
#[tracing::instrument(skip(state))]
pub async fn get_news(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let news = NewsService::get_news(&state, id).await?;
    Ok(ApiResponse::success(news))
}

/// POST /admin/news - 创建新闻
#[tracing::instrument(skip_all)]
pub async fn create_news(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<CreateNewsRequest>,
) -> Result<impl IntoResponse, AppError> {
    let news = NewsService::create_news(&state, payload).await?;
    Ok(ApiResponse::success(news))
}

/// PUT /admin/news/:id - 更新新闻
#[tracing::instrument(skip(state, payload))]
pub async fn update_news(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    axum::Json(payload): axum::Json<UpdateNewsRequest>,
) -> Result<impl IntoResponse, AppError> {
    let news = NewsService::update_news(&state, id, payload).await?;
    Ok(ApiResponse::success(news))
}

/// DELETE /admin/news/:id - 删除新闻
#[tracing::instrument(skip(state))]
pub async fn delete_news(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    NewsService::delete_news(&state, id).await?;
    Ok(ApiResponse::<()>::success_empty())
}

/// 构建后台新闻管理路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/news", get(list_news).post(create_news))
        .route("/news/all", get(list_all_news))
        .route("/news/{id}", get(get_news).put(update_news).delete(delete_news))
}
