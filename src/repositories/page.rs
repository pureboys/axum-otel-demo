use sea_orm::*;

use crate::models::page;

pub struct PageRepository;

impl PageRepository {
    /// 查询全部页面
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<page::Model>, DbErr> {
        page::Entity::find().all(db).await
    }

    /// 按 ID 查询页面
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<page::Model>, DbErr> {
        page::Entity::find_by_id(id).one(db).await
    }

    /// 按slug查询页面
    pub async fn find_by_slug(
        db: &DatabaseConnection,
        slug: &str,
    ) -> Result<Option<page::Model>, DbErr> {
        page::Entity::find()
            .filter(page::Column::Slug.eq(slug))
            .one(db)
            .await
    }

    /// 按状态查询页面
    pub async fn find_by_status(
        db: &DatabaseConnection,
        status: i8,
    ) -> Result<Vec<page::Model>, DbErr> {
        page::Entity::find()
            .filter(page::Column::Status.eq(status))
            .all(db)
            .await
    }

    /// 创建页面
    pub async fn create(
        db: &DatabaseConnection,
        title: String,
        slug: String,
        content: String,
        meta_title: Option<String>,
        meta_description: Option<String>,
        status: i8,
    ) -> Result<page::Model, DbErr> {
        let model = page::ActiveModel {
            title: Set(title),
            slug: Set(slug),
            content: Set(content),
            meta_title: Set(meta_title),
            meta_description: Set(meta_description),
            status: Set(status),
            created_at: Set(crate::utils::time::now()),
            updated_at: Set(crate::utils::time::now()),
            ..Default::default()
        };
        model.insert(db).await
    }

    /// 更新页面
    pub async fn update(
        db: &DatabaseConnection,
        mut model: page::ActiveModel,
        title: Option<String>,
        slug: Option<String>,
        content: Option<String>,
        meta_title: Option<Option<String>>,
        meta_description: Option<Option<String>>,
        status: Option<i8>,
    ) -> Result<page::Model, DbErr> {
        if let Some(title) = title {
            model.title = Set(title);
        }
        if let Some(slug) = slug {
            model.slug = Set(slug);
        }
        if let Some(content) = content {
            model.content = Set(content);
        }
        if let Some(meta_title) = meta_title {
            model.meta_title = Set(meta_title);
        }
        if let Some(meta_description) = meta_description {
            model.meta_description = Set(meta_description);
        }
        if let Some(status) = status {
            model.status = Set(status);
        }
        model.updated_at = Set(crate::utils::time::now());
        model.update(db).await
    }

    /// 删除页面
    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
        page::Entity::delete_by_id(id).exec(db).await
    }
}
