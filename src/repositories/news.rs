use sea_orm::*;
use chrono::NaiveDateTime;

use crate::models::news;

pub struct NewsRepository;

impl NewsRepository {
    /// 查询全部新闻
    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<news::Model>, DbErr> {
        news::Entity::find().all(db).await
    }

    /// 按 ID 查询新闻
    pub async fn find_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<news::Model>, DbErr> {
        news::Entity::find_by_id(id).one(db).await
    }

    /// 分页查询新闻
    pub async fn find_paginated(
        db: &DatabaseConnection,
        page: u32,
        limit: u32,
        category_id: Option<i32>,
        status: Option<i8>,
    ) -> Result<(Vec<news::Model>, u32), DbErr> {
        let mut query = news::Entity::find();

        if let Some(cid) = category_id {
            query = query.filter(news::Column::CategoryId.eq(cid));
        }
        if let Some(s) = status {
            query = query.filter(news::Column::Status.eq(s));
        }

        // Count total
        let total = query.clone().count(db).await?;

        // Apply pagination
        let offset = ((page - 1) * limit) as u64;
        let limit = limit as u64;
        let items = query
            .order_by(news::Column::PublishedAt, Order::Desc)
            .offset(Some(offset))
            .limit(Some(limit))
            .all(db)
            .await?;

        Ok((items, total as u32))
    }

    /// 创建新闻
    pub async fn create(
        db: &DatabaseConnection,
        title: String,
        slug: String,
        content: String,
        excerpt: Option<String>,
        cover_image: String,
        category_id: i32,
        author: String,
        status: i8,
        is_featured: i8,
        published_at: Option<NaiveDateTime>,
        meta_title: Option<String>,
        meta_description: Option<String>,
    ) -> Result<news::Model, DbErr> {
        let model = news::ActiveModel {
            title: Set(title),
            slug: Set(slug),
            content: Set(content),
            excerpt: Set(excerpt),
            cover_image: Set(cover_image),
            category_id: Set(category_id),
            author: Set(author),
            view_count: Set(0),
            status: Set(status),
            is_featured: Set(is_featured),
            published_at: Set(published_at),
            meta_title: Set(meta_title),
            meta_description: Set(meta_description),
            created_at: Set(crate::utils::time::now()),
            updated_at: Set(crate::utils::time::now()),
            ..Default::default()
        };
        model.insert(db).await
    }

    /// 更新新闻
    pub async fn update(
        db: &DatabaseConnection,
        mut model: news::ActiveModel,
        title: Option<String>,
        slug: Option<String>,
        content: Option<String>,
        excerpt: Option<Option<String>>,
        cover_image: Option<String>,
        category_id: Option<i32>,
        author: Option<String>,
        status: Option<i8>,
        is_featured: Option<i8>,
        published_at: Option<Option<NaiveDateTime>>,
        meta_title: Option<Option<String>>,
        meta_description: Option<Option<String>>,
    ) -> Result<news::Model, DbErr> {
        if let Some(title) = title {
            model.title = Set(title);
        }
        if let Some(slug) = slug {
            model.slug = Set(slug);
        }
        if let Some(content) = content {
            model.content = Set(content);
        }
        if let Some(excerpt) = excerpt {
            model.excerpt = Set(excerpt);
        }
        if let Some(cover_image) = cover_image {
            model.cover_image = Set(cover_image);
        }
        if let Some(category_id) = category_id {
            model.category_id = Set(category_id);
        }
        if let Some(author) = author {
            model.author = Set(author);
        }
        if let Some(status) = status {
            model.status = Set(status);
        }
        if let Some(is_featured) = is_featured {
            model.is_featured = Set(is_featured);
        }
        if let Some(published_at) = published_at {
            model.published_at = Set(published_at);
        }
        if let Some(meta_title) = meta_title {
            model.meta_title = Set(meta_title);
        }
        if let Some(meta_description) = meta_description {
            model.meta_description = Set(meta_description);
        }
        model.updated_at = Set(crate::utils::time::now());
        model.update(db).await
    }

    /// 删除新闻
    pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<DeleteResult, DbErr> {
        news::Entity::delete_by_id(id).exec(db).await
    }
}