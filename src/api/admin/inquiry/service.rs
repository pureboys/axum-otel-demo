use crate::app::AppState;
use crate::error::AppError;
use crate::repositories::inquiry::InquiryRepository;
use super::dto::{BatchDeleteRequest, CreateInquiryRequest, InquiryResponse, PaginatedInquiryResponse};

pub struct InquiryService;

impl InquiryService {
    /// 分页获取询盘列表
    pub async fn list_inquiries(
        state: &AppState,
        page: u32,
        limit: u32,
    ) -> Result<PaginatedInquiryResponse, AppError> {
        let (items, total) = InquiryRepository::find_paginated(&state.db, page, limit)
            .await
            .map_err(AppError::from)?;

        Ok(PaginatedInquiryResponse {
            items: items.into_iter().map(InquiryResponse::from).collect(),
            total,
            page,
            limit,
        })
    }

    /// 提交询盘
    pub async fn create_inquiry(
        state: &AppState,
        req: CreateInquiryRequest,
    ) -> Result<InquiryResponse, AppError> {
        if req.name.trim().is_empty() {
            return Err(AppError::Validation("姓名不能为空".to_string()));
        }
        if req.message.trim().is_empty() {
            return Err(AppError::Validation("留言内容不能为空".to_string()));
        }

        let inquiry = InquiryRepository::create(
            &state.db,
            req.name,
            req.email,
            req.phone,
            req.message,
            req.product_id,
            req.product_name,
        )
        .await
        .map_err(AppError::from)?;

        Ok(InquiryResponse::from(inquiry))
    }

    /// 删除询盘
    pub async fn delete_inquiry(state: &AppState, id: i32) -> Result<(), AppError> {
        InquiryRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("询盘不存在".to_string()))?;

        InquiryRepository::delete(&state.db, id)
            .await
            .map_err(AppError::from)?;

        Ok(())
    }

    /// 批量删除询盘
    pub async fn batch_delete_inquiries(
        state: &AppState,
        req: BatchDeleteRequest,
    ) -> Result<u64, AppError> {
        if req.ids.is_empty() {
            return Ok(0);
        }
        let result = InquiryRepository::delete_batch(&state.db, &req.ids)
            .await
            .map_err(AppError::from)?;
        Ok(result.rows_affected)
    }
}
