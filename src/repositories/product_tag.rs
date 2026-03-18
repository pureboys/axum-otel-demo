use sea_orm::*;

use crate::models::product_tag;

pub struct ProductTagRepository;

impl ProductTagRepository {
    /// 获取产品的所有标签ID
    pub async fn find_by_product(
        db: &DatabaseConnection,
        product_id: i32,
    ) -> Result<Vec<product_tag::Model>, DbErr> {
        product_tag::Entity::find()
            .filter(product_tag::Column::ProductId.eq(product_id))
            .all(db)
            .await
    }

    /// 获取标签的所有产品ID
    pub async fn find_by_tag(
        db: &DatabaseConnection,
        tag_id: i32,
    ) -> Result<Vec<product_tag::Model>, DbErr> {
        product_tag::Entity::find()
            .filter(product_tag::Column::TagId.eq(tag_id))
            .all(db)
            .await
    }

    /// 为产品添加标签
    pub async fn add_tag(
        db: &DatabaseConnection,
        product_id: i32,
        tag_id: i32,
    ) -> Result<product_tag::Model, DbErr> {
        let model = product_tag::ActiveModel {
            product_id: Set(product_id),
            tag_id: Set(tag_id),
            ..Default::default()
        };
        model.insert(db).await
    }

    /// 为产品移除标签
    pub async fn remove_tag(
        db: &DatabaseConnection,
        product_id: i32,
        tag_id: i32,
    ) -> Result<DeleteResult, DbErr> {
        // 先查询再删除
        let models = product_tag::Entity::find()
            .filter(product_tag::Column::ProductId.eq(product_id))
            .filter(product_tag::Column::TagId.eq(tag_id))
            .all(db)
            .await?;

        let mut deleted_count = 0;
        for model in models {
            model.delete(db).await?;
            deleted_count += 1;
        }

        Ok(DeleteResult {
            rows_affected: deleted_count,
        })
    }

    /// 删除产品的所有标签
    pub async fn delete_by_product(
        db: &DatabaseConnection,
        product_id: i32,
    ) -> Result<DeleteResult, DbErr> {
        // 先查询再删除
        let models = product_tag::Entity::find()
            .filter(product_tag::Column::ProductId.eq(product_id))
            .all(db)
            .await?;

        let mut deleted_count = 0;
        for model in models {
            model.delete(db).await?;
            deleted_count += 1;
        }

        Ok(DeleteResult {
            rows_affected: deleted_count,
        })
    }

    /// 设置产品的标签（先删除全部再添加）
    pub async fn set_tags(
        db: &DatabaseConnection,
        product_id: i32,
        tag_ids: &[i32],
    ) -> Result<(), DbErr> {
        // 删除原有标签
        Self::delete_by_product(db, product_id).await?;

        // 添加新标签
        for tag_id in tag_ids {
            Self::add_tag(db, product_id, *tag_id).await?;
        }
        Ok(())
    }
}
