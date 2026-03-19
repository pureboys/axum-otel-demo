use serde::{Deserialize, Serialize};

/// 创建新闻请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNewsRequest {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub cover_image: String,
    pub category_id: i32,
    pub author: String,
    pub status: Option<i8>,
    pub is_featured: Option<i8>,
    pub published_at: Option<String>,
    /// SEO 标题
    pub meta_title: Option<String>,
    /// SEO 描述
    pub meta_description: Option<String>,
}

/// 更新新闻请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateNewsRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub excerpt: Option<Option<String>>,
    pub cover_image: Option<String>,
    pub category_id: Option<i32>,
    pub author: Option<String>,
    pub status: Option<i8>,
    pub is_featured: Option<i8>,
    pub published_at: Option<String>,
    /// SEO 标题
    pub meta_title: Option<Option<String>>,
    /// SEO 描述
    pub meta_description: Option<Option<String>>,
}

/// 新闻响应
#[derive(Debug, Serialize)]
pub struct NewsResponse {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub excerpt: Option<String>,
    pub cover_image: String,
    pub category_id: i32,
    pub author: String,
    pub view_count: i32,
    pub status: i8,
    pub is_featured: i8,
    pub published_at: Option<String>,
    /// SEO 标题
    pub meta_title: Option<String>,
    /// SEO 描述
    pub meta_description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::models::news::Model> for NewsResponse {
    fn from(model: crate::models::news::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            slug: model.slug,
            content: model.content,
            excerpt: model.excerpt,
            cover_image: model.cover_image,
            category_id: model.category_id,
            author: model.author,
            view_count: model.view_count,
            status: model.status,
            is_featured: model.is_featured,
            published_at: model.published_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            meta_title: model.meta_title,
            meta_description: model.meta_description,
            created_at: model.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: model.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

/// 分页响应
#[derive(Debug, Serialize)]
pub struct PaginatedNewsResponse {
    pub items: Vec<NewsResponse>,
    pub total: u32,
    pub page: u32,
    pub limit: u32,
}
