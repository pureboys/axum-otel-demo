//! 后台产品管理模块

mod dto;
mod service;

use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use serde::Deserialize;

use crate::app::AppState;
use crate::dto::response::ApiResponse;
use crate::error::AppError;
use service::ProductService;
use dto::{CreateProductRequest, UpdateProductRequest, SetProductTagsRequest};

/// 分页查询参数
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub category_id: Option<i32>,
    pub status: Option<i8>,
}

/// GET /admin/products - 获取产品列表（支持分页）
#[tracing::instrument(skip_all)]
pub async fn list_products(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(10).min(100);
    let products = ProductService::list_products_paginated(
        &state,
        page,
        limit,
        query.category_id,
        query.status,
    )
    .await?;
    Ok(ApiResponse::success(products))
}

/// GET /products/:id - 获取产品详情
#[tracing::instrument(skip(state))]
pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let product = ProductService::get_product(&state, id).await?;
    Ok(ApiResponse::success(product))
}

/// GET /admin/products/:id/tags - 获取产品详情（含标签）
#[tracing::instrument(skip(state))]
pub async fn get_product_with_tags(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let product = ProductService::get_product_with_tags(&state, id).await?;
    Ok(ApiResponse::success(product))
}

/// POST /admin/products - 创建产品
#[tracing::instrument(skip_all)]
pub async fn create_product(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<CreateProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    let product = ProductService::create_product(&state, payload).await?;
    Ok(ApiResponse::success(product))
}

/// PUT /admin/products/:id - 更新产品
#[tracing::instrument(skip(state, payload))]
pub async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    axum::Json(payload): axum::Json<UpdateProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    let product = ProductService::update_product(&state, id, payload).await?;
    Ok(ApiResponse::success(product))
}

/// DELETE /admin/products/:id - 删除产品
#[tracing::instrument(skip(state))]
pub async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    ProductService::delete_product(&state, id).await?;
    Ok(ApiResponse::<()>::success_empty())
}

/// PUT /admin/products/:id/tags - 设置产品标签
#[tracing::instrument(skip(state, payload))]
pub async fn set_product_tags(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    axum::Json(payload): axum::Json<SetProductTagsRequest>,
) -> Result<impl IntoResponse, AppError> {
    let tags = ProductService::set_product_tags(&state, id, payload).await?;
    Ok(ApiResponse::success(tags))
}

/// POST /admin/products/:id/tags/:tag_id - 添加产品标签
#[tracing::instrument(skip(state))]
pub async fn add_product_tag(
    State(state): State<AppState>,
    Path((product_id, tag_id)): Path<(i32, i32)>,
) -> Result<impl IntoResponse, AppError> {
    let tag = ProductService::add_product_tag(&state, product_id, tag_id).await?;
    Ok(ApiResponse::success(tag))
}

/// DELETE /admin/products/:id/tags/:tag_id - 移除产品标签
#[tracing::instrument(skip(state))]
pub async fn remove_product_tag(
    State(state): State<AppState>,
    Path((product_id, tag_id)): Path<(i32, i32)>,
) -> Result<impl IntoResponse, AppError> {
    ProductService::remove_product_tag(&state, product_id, tag_id).await?;
    Ok(ApiResponse::<()>::success_empty())
}

/// 构建后台产品管理路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/products", get(list_products).post(create_product))
        .route("/products/{id}", get(get_product).put(update_product).delete(delete_product))
        .route("/products/{id}/tags", get(get_product_with_tags).put(set_product_tags))
        .route("/products/{product_id}/tags/{tag_id}", post(add_product_tag).delete(remove_product_tag))
}
