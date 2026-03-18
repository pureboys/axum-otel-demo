use crate::app::AppState;
use crate::error::AppError;
use crate::repositories::tag::TagRepository;
use super::dto::{CreateTagRequest, UpdateTagRequest, TagResponse};

pub struct TagService;

impl TagService {
    /// 获取所有标签
    pub async fn list_tags(state: &AppState) -> Result<Vec<TagResponse>, AppError> {
        let tags = TagRepository::find_all(&state.db)
            .await
            .map_err(AppError::from)?;
        Ok(tags.into_iter().map(TagResponse::from).collect())
    }

    /// 获取标签详情
    pub async fn get_tag(state: &AppState, id: i32) -> Result<TagResponse, AppError> {
        let tag = TagRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("标签不存在".to_string()))?;
        Ok(TagResponse::from(tag))
    }

    /// 创建标签
    pub async fn create_tag(
        state: &AppState,
        req: CreateTagRequest,
    ) -> Result<TagResponse, AppError> {
        let tag = TagRepository::create(
            &state.db,
            req.name,
            req.slug,
        )
        .await
        .map_err(AppError::from)?;
        Ok(TagResponse::from(tag))
    }

    /// 更新标签
    pub async fn update_tag(
        state: &AppState,
        id: i32,
        req: UpdateTagRequest,
    ) -> Result<TagResponse, AppError> {
        let existing = TagRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("标签不存在".to_string()))?;

        let active_model = existing.into();
        let tag = TagRepository::update(
            &state.db,
            active_model,
            req.name,
            req.slug,
        )
        .await
        .map_err(AppError::from)?;
        Ok(TagResponse::from(tag))
    }

    /// 删除标签
    pub async fn delete_tag(state: &AppState, id: i32) -> Result<(), AppError> {
        TagRepository::delete(&state.db, id)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}
