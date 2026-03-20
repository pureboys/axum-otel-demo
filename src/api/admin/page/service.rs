use crate::app::AppState;
use crate::error::AppError;
use crate::repositories::page::PageRepository;
use super::dto::{CreatePageRequest, UpdatePageRequest, PageResponse, PaginatedPageResponse};

pub struct PageService;

impl PageService {
    /// 获取所有页面（不分页）
    pub async fn list_pages(state: &AppState) -> Result<Vec<PageResponse>, AppError> {
        let pages = PageRepository::find_all(&state.db)
            .await
            .map_err(AppError::from)?;
        Ok(pages.into_iter().map(PageResponse::from).collect())
    }

    /// 分页获取页面
    pub async fn list_pages_paginated(
        state: &AppState,
        page: u32,
        limit: u32,
        status: Option<i8>,
    ) -> Result<PaginatedPageResponse, AppError> {
        let (pages, total) = PageRepository::find_paginated(&state.db, page, limit, status)
            .await
            .map_err(AppError::from)?;
        Ok(PaginatedPageResponse {
            items: pages.into_iter().map(PageResponse::from).collect(),
            total,
            page,
            limit,
        })
    }

    /// 获取页面详情
    pub async fn get_page(state: &AppState, id: i32) -> Result<PageResponse, AppError> {
        let page = PageRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("页面不存在".to_string()))?;
        Ok(PageResponse::from(page))
    }

    /// 按slug获取页面
    pub async fn get_page_by_slug(state: &AppState, slug: &str) -> Result<PageResponse, AppError> {
        let page = PageRepository::find_by_slug(&state.db, slug)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("页面不存在".to_string()))?;
        Ok(PageResponse::from(page))
    }

    /// 创建页面
    pub async fn create_page(
        state: &AppState,
        req: CreatePageRequest,
    ) -> Result<PageResponse, AppError> {
        let page = PageRepository::create(
            &state.db,
            req.title,
            req.slug,
            req.content,
            req.meta_title,
            req.meta_description,
            req.status.unwrap_or(0),
        )
        .await
        .map_err(AppError::from)?;
        Ok(PageResponse::from(page))
    }

    /// 更新页面
    pub async fn update_page(
        state: &AppState,
        id: i32,
        req: UpdatePageRequest,
    ) -> Result<PageResponse, AppError> {
        let existing = PageRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("页面不存在".to_string()))?;

        let active_model = existing.into();
        let page = PageRepository::update(
            &state.db,
            active_model,
            req.title,
            req.slug,
            req.content,
            req.meta_title,
            req.meta_description,
            req.status,
        )
        .await
        .map_err(AppError::from)?;
        Ok(PageResponse::from(page))
    }

    /// 删除页面
    pub async fn delete_page(state: &AppState, id: i32) -> Result<(), AppError> {
        PageRepository::delete(&state.db, id)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}
