use crate::app::AppState;
use crate::error::AppError;
use crate::repositories::category::CategoryRepository;
use super::dto::{CreateCategoryRequest, UpdateCategoryRequest, CategoryResponse};

pub struct CategoryService;

impl CategoryService {
    /// 获取所有分类
    pub async fn list_categories(state: &AppState) -> Result<Vec<CategoryResponse>, AppError> {
        let categories = CategoryRepository::find_all(&state.db)
            .await
            .map_err(AppError::from)?;
        Ok(categories.into_iter().map(CategoryResponse::from).collect())
    }

    /// 获取分类详情
    pub async fn get_category(state: &AppState, id: i32) -> Result<CategoryResponse, AppError> {
        let category = CategoryRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("分类不存在".to_string()))?;
        Ok(CategoryResponse::from(category))
    }

    /// 创建分类
    pub async fn create_category(
        state: &AppState,
        req: CreateCategoryRequest,
    ) -> Result<CategoryResponse, AppError> {
        let category = CategoryRepository::create(
            &state.db,
            req.name,
            req.slug,
            req.description,
            req.category_type,
            req.parent_id,
        )
        .await
        .map_err(AppError::from)?;
        Ok(CategoryResponse::from(category))
    }

    /// 更新分类
    pub async fn update_category(
        state: &AppState,
        id: i32,
        req: UpdateCategoryRequest,
    ) -> Result<CategoryResponse, AppError> {
        let existing = CategoryRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("分类不存在".to_string()))?;

        let active_model = existing.into();
        let category = CategoryRepository::update(
            &state.db,
            active_model,
            req.name,
            req.slug,
            req.description,
            req.category_type,
            req.parent_id,
        )
        .await
        .map_err(AppError::from)?;
        Ok(CategoryResponse::from(category))
    }

    /// 删除分类
    pub async fn delete_category(state: &AppState, id: i32) -> Result<(), AppError> {
        CategoryRepository::delete(&state.db, id)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}
