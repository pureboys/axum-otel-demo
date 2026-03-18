use sea_orm::prelude::Decimal;
use std::str::FromStr;

use crate::app::AppState;
use crate::error::AppError;
use crate::repositories::product::ProductRepository;
use crate::repositories::product_tag::ProductTagRepository;
use crate::repositories::tag::TagRepository;
use super::dto::{CreateProductRequest, UpdateProductRequest, ProductResponse, ProductWithTagsResponse, SetProductTagsRequest};

pub struct ProductService;

impl ProductService {
    /// 获取所有产品
    pub async fn list_products(state: &AppState) -> Result<Vec<ProductResponse>, AppError> {
        let products = ProductRepository::find_all(&state.db)
            .await
            .map_err(AppError::from)?;
        Ok(products.into_iter().map(ProductResponse::from).collect())
    }

    /// 获取产品详情
    pub async fn get_product(state: &AppState, id: i32) -> Result<ProductResponse, AppError> {
        let product = ProductRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("产品不存在".to_string()))?;
        Ok(ProductResponse::from(product))
    }

    /// 获取产品详情（含标签）
    pub async fn get_product_with_tags(state: &AppState, id: i32) -> Result<ProductWithTagsResponse, AppError> {
        let product = ProductRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("产品不存在".to_string()))?;

        let product_tags = ProductTagRepository::find_by_product(&state.db, id)
            .await
            .map_err(AppError::from)?;

        let tag_ids: Vec<i32> = product_tags.iter().map(|pt| pt.tag_id).collect();

        // 批量查询标签，避免循环查数据库
        let all_tags = TagRepository::find_by_ids(&state.db, &tag_ids)
            .await
            .map_err(AppError::from)?;

        let tags = all_tags
            .into_iter()
            .map(super::dto::TagResponse::from)
            .collect();

        Ok(ProductWithTagsResponse {
            product: ProductResponse::from(product),
            tags,
        })
    }

    /// 创建产品
    pub async fn create_product(
        state: &AppState,
        req: CreateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        let price = Decimal::from_str(&req.price.to_string())
            .map_err(|_| AppError::Validation("无效的价格格式".to_string()))?;
        let product = ProductRepository::create(
            &state.db,
            req.name,
            req.description,
            price,
            req.stock,
            req.category_id,
            req.image_url,
            req.status.unwrap_or(1),
        )
        .await
        .map_err(AppError::from)?;
        Ok(ProductResponse::from(product))
    }

    /// 更新产品
    pub async fn update_product(
        state: &AppState,
        id: i32,
        req: UpdateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        let existing = ProductRepository::find_by_id(&state.db, id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("产品不存在".to_string()))?;

        let active_model = existing.into();
        let price = req.price.map(|p| {
            Decimal::from_str(&p.to_string()).unwrap_or(Decimal::from(0))
        });
        let product = ProductRepository::update(
            &state.db,
            active_model,
            req.name,
            req.description,
            price,
            req.stock,
            req.category_id,
            req.image_url,
            req.status,
        )
        .await
        .map_err(AppError::from)?;
        Ok(ProductResponse::from(product))
    }

    /// 删除产品
    pub async fn delete_product(state: &AppState, id: i32) -> Result<(), AppError> {
        // 先删除产品标签关联
        ProductTagRepository::delete_by_product(&state.db, id)
            .await
            .map_err(AppError::from)?;
        // 再删除产品
        ProductRepository::delete(&state.db, id)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }

    /// 设置产品标签
    pub async fn set_product_tags(
        state: &AppState,
        product_id: i32,
        req: SetProductTagsRequest,
    ) -> Result<Vec<super::dto::TagResponse>, AppError> {
        // 检查产品是否存在
        ProductRepository::find_by_id(&state.db, product_id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("产品不存在".to_string()))?;

        // 设置标签
        ProductTagRepository::set_tags(&state.db, product_id, &req.tag_ids)
            .await
            .map_err(AppError::from)?;

        // 批量查询标签，避免循环查数据库
        let all_tags = TagRepository::find_by_ids(&state.db, &req.tag_ids)
            .await
            .map_err(AppError::from)?;

        Ok(all_tags
            .into_iter()
            .map(super::dto::TagResponse::from)
            .collect())
    }

    /// 为产品添加标签
    pub async fn add_product_tag(
        state: &AppState,
        product_id: i32,
        tag_id: i32,
    ) -> Result<super::dto::TagResponse, AppError> {
        // 检查产品是否存在
        ProductRepository::find_by_id(&state.db, product_id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("产品不存在".to_string()))?;

        // 检查标签是否存在
        TagRepository::find_by_id(&state.db, tag_id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("标签不存在".to_string()))?;

        // 添加标签关联
        ProductTagRepository::add_tag(&state.db, product_id, tag_id)
            .await
            .map_err(AppError::from)?;

        // 返回标签信息
        let tag = TagRepository::find_by_id(&state.db, tag_id)
            .await
            .map_err(AppError::from)?
            .ok_or(AppError::NotFound("标签不存在".to_string()))?;
        Ok(super::dto::TagResponse::from(tag))
    }

    /// 为产品移除标签
    pub async fn remove_product_tag(
        state: &AppState,
        product_id: i32,
        tag_id: i32,
    ) -> Result<(), AppError> {
        ProductTagRepository::remove_tag(&state.db, product_id, tag_id)
            .await
            .map_err(AppError::from)?;
        Ok(())
    }
}
