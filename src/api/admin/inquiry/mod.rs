//! 询盘管理模块

mod dto;
mod service;

use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::Router;
use serde::Deserialize;

use crate::app::AppState;
use crate::dto::response::ApiResponse;
use crate::error::AppError;
use dto::{BatchDeleteRequest, CreateInquiryRequest};
use service::InquiryService;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

/// POST /api/admin/inquiries - 提交询盘（公开接口，产品详情页调用）
#[tracing::instrument(skip_all)]
pub async fn create_inquiry(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<CreateInquiryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let inquiry = InquiryService::create_inquiry(&state, payload).await?;
    Ok(ApiResponse::success(inquiry))
}

/// GET /api/admin/inquiries - 获取询盘列表（受保护）
#[tracing::instrument(skip_all)]
pub async fn list_inquiries(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> Result<impl IntoResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(20).min(100);
    let result = InquiryService::list_inquiries(&state, page, limit).await?;
    Ok(ApiResponse::success(result))
}

/// DELETE /api/admin/inquiries/:id - 删除询盘（受保护）
#[tracing::instrument(skip(state))]
pub async fn delete_inquiry(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    InquiryService::delete_inquiry(&state, id).await?;
    Ok(ApiResponse::<()>::success_empty())
}

/// DELETE /api/admin/inquiries - 批量删除询盘（受保护）
#[tracing::instrument(skip_all)]
pub async fn batch_delete_inquiries(
    State(state): State<AppState>,
    axum::Json(payload): axum::Json<BatchDeleteRequest>,
) -> Result<impl IntoResponse, AppError> {
    let count = InquiryService::batch_delete_inquiries(&state, payload).await?;
    Ok(ApiResponse::success(serde_json::json!({ "deleted": count })))
}

/// 公开路由（无需认证）
pub fn public_routes() -> Router<AppState> {
    Router::new().route("/inquiries", post(create_inquiry))
}

/// 受保护路由（需认证）
pub fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/inquiries", get(list_inquiries).delete(batch_delete_inquiries))
        .route("/inquiries/{id}", delete(delete_inquiry))
}
