use chrono::NaiveDateTime;

use crate::app::AppState;
use crate::error::AppError;
use crate::repositories::news::NewsRepository;
use super::dto::{CreateNewsRequest, UpdateNewsRequest, NewsResponse, PaginatedNewsResponse};

pub struct NewsService;

impl NewsService {
    /// 获取所有新闻
    pub async fn list_news(state: &AppState) -> Result<Vec<NewsResponse>, AppError> {
        let news_list = NewsRepository::find_all(&state.db)
            .await
            .map_err(AppError::from)?;
        Ok(news_list.into_iter().map(NewsResponse::from).collect())
    }

    /// 分页获取新闻
    pub async fn list_news_paginated(
        state: &AppState,
        page: u32,
        limit: u32,
        category_id: Option<i32>,
        status: Option<i8>,
    ) -> Result<PaginatedNewsResponse, AppError> {
        let (news_list, total) = NewsRepository::find_paginated(
            &state.db,
            page,
            limit,
            category_id,
            status,
        )
        .await
        .map_err(AppError::from)?;

        Ok(PaginatedNewsResponse {
            items: news_list.into_iter().map(NewsResponse::from).collect(),
            total,
            page,
            limit,
        })
    }

    /// 获取新闻详情
    pub async fn get_news(state: &AppState, id: i32) -> Result<NewsResponse, AppError> {
        let news = NewsRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("新闻不存在".to_string()))?;
        Ok(NewsResponse::from(news))
    }

    /// 创建新闻
    pub async fn create_news(
        state: &AppState,
        req: CreateNewsRequest,
    ) -> Result<NewsResponse, AppError> {
        let published_at = if let Some(published_at_str) = req.published_at {
            NaiveDateTime::parse_from_str(&published_at_str, "%Y-%m-%d %H:%M:%S").ok()
        } else {
            None
        };

        let news = NewsRepository::create(
            &state.db,
            req.title,
            req.slug,
            req.content,
            req.excerpt,
            req.cover_image,
            req.category_id,
            req.author,
            req.status.unwrap_or(0),
            req.is_featured.unwrap_or(0),
            published_at,
            req.meta_title,
            req.meta_description,
        )
        .await
        .map_err(AppError::from)?;
        Ok(NewsResponse::from(news))
    }

    /// 更新新闻
    pub async fn update_news(
        state: &AppState,
        id: i32,
        req: UpdateNewsRequest,
    ) -> Result<NewsResponse, AppError> {
        let existing = NewsRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("新闻不存在".to_string()))?;

        let active_model = existing.into();
        let published_at = if let Some(published_at_str) = req.published_at {
            NaiveDateTime::parse_from_str(&published_at_str, "%Y-%m-%d %H:%M:%S").ok()
        } else {
            None
        };

        let news = NewsRepository::update(
            &state.db,
            active_model,
            req.title,
            req.slug,
            req.content,
            req.excerpt,
            req.cover_image,
            req.category_id,
            req.author,
            req.status,
            req.is_featured,
            Some(published_at),
            req.meta_title,
            req.meta_description,
        )
        .await
        .map_err(AppError::from)?;
        Ok(NewsResponse::from(news))
    }

    /// 删除新闻
    pub async fn delete_news(state: &AppState, id: i32) -> Result<(), AppError> {
        NewsRepository::delete(&state.db, id)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}
