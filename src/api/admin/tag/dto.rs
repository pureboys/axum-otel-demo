use serde::{Deserialize, Serialize};

/// 创建标签请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub slug: String,
}

/// 更新标签请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTagRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
}

/// 标签响应
#[derive(Debug, Serialize)]
pub struct TagResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::models::tag::Model> for TagResponse {
    fn from(model: crate::models::tag::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            slug: model.slug,
            created_at: model.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: model.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}
