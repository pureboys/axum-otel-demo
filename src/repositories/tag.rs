use sea_orm::*;

use crate::models::tag;

pub struct TagRepository;

impl TagRepository {
    /// 查询全部标签
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<tag::Model>, DbErr> {
        tag::Entity::find().all(db).await
    }

    /// 按 ID 查询标签
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<tag::Model>, DbErr> {
        tag::Entity::find_by_id(id).one(db).await
    }

    /// 批量查询标签
    pub async fn find_by_ids(db: &DatabaseConnection, ids: &[i32]) -> Result<Vec<tag::Model>, DbErr> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        tag::Entity::find()
            .filter(tag::Column::Id.is_in(ids.iter().cloned()))
            .all(db)
            .await
    }

    /// 创建标签
    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        slug: String,
    ) -> Result<tag::Model, DbErr> {
        let model = tag::ActiveModel {
            name: Set(name),
            slug: Set(slug),
            created_at: Set(crate::utils::time::now()),
            updated_at: Set(crate::utils::time::now()),
            ..Default::default()
        };
        model.insert(db).await
    }

    /// 更新标签
    pub async fn update(
        db: &DatabaseConnection,
        mut model: tag::ActiveModel,
        name: Option<String>,
        slug: Option<String>,
    ) -> Result<tag::Model, DbErr> {
        if let Some(name) = name {
            model.name = Set(name);
        }
        if let Some(slug) = slug {
            model.slug = Set(slug);
        }
        model.updated_at = Set(crate::utils::time::now());
        model.update(db).await
    }

    /// 删除标签
    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
        tag::Entity::delete_by_id(id).exec(db).await
    }
}