use sea_orm::*;

use crate::models::category;

pub struct CategoryRepository;

impl CategoryRepository {
    /// 查询全部分类
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<category::Model>, DbErr> {
        category::Entity::find().all(db).await
    }

    /// 按类型查询分类
    pub async fn find_by_type(
        db: &DatabaseConnection,
        category_type: &str,
    ) -> Result<Vec<category::Model>, DbErr> {
        category::Entity::find()
            .filter(category::Column::CategoryType.eq(category_type))
            .order_by(category::Column::Id, Order::Asc)
            .all(db)
            .await
    }

    /// 按 ID 查询分类
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<category::Model>, DbErr> {
        category::Entity::find_by_id(id).one(db).await
    }



    /// 创建分类
    pub async fn create(
        db: &DatabaseConnection,
        name: String,
        slug: String,
        description: String,
        category_type: String,
        parent_id: Option<i32>,
    ) -> Result<category::Model, DbErr> {
        let model = category::ActiveModel {
            name: Set(name),
            slug: Set(slug),
            description: Set(description),
            category_type: Set(category_type),
            parent_id: Set(parent_id),
            created_at: Set(crate::utils::time::now()),
            updated_at: Set(crate::utils::time::now()),
            ..Default::default()
        };
        model.insert(db).await
    }

    /// 更新分类
    pub async fn update(
        db: &DatabaseConnection,
        mut model: category::ActiveModel,
        name: Option<String>,
        slug: Option<String>,
        description: Option<String>,
        category_type: Option<String>,
        parent_id: Option<Option<i32>>,
    ) -> Result<category::Model, DbErr> {
        if let Some(name) = name {
            model.name = Set(name);
        }
        if let Some(slug) = slug {
            model.slug = Set(slug);
        }
        if let Some(description) = description {
            model.description = Set(description);
        }
        if let Some(category_type) = category_type {
            model.category_type = Set(category_type);
        }
        if let Some(parent_id) = parent_id {
            model.parent_id = Set(parent_id);
        }
        model.updated_at = Set(crate::utils::time::now());
        model.update(db).await
    }

    /// 删除分类
    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
        category::Entity::delete_by_id(id).exec(db).await
    }
}
