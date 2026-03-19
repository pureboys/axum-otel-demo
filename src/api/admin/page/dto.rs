use serde::{Deserialize, Serialize};

/// 创建页面请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePageRequest {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub status: Option<i8>,
}

/// 更新页面请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePageRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub meta_title: Option<Option<String>>,
    pub meta_description: Option<Option<String>>,
    pub status: Option<i8>,
}

/// 页面响应
#[derive(Debug, Serialize)]
pub struct PageResponse {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub status: i8,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::models::page::Model> for PageResponse {
    fn from(model: crate::models::page::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            slug: model.slug,
            content: model.content,
            meta_title: model.meta_title,
            meta_description: model.meta_description,
            status: model.status,
            created_at: model.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: model.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}
