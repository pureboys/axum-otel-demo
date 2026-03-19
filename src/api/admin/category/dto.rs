use serde::{Deserialize, Serialize};

/// 创建分类请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub slug: String,
    pub description: String,
    /// 分类类型: "product" 或 "news"
    pub category_type: String,
    pub parent_id: Option<i32>,
}

/// 更新分类请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub category_type: Option<String>,
    pub parent_id: Option<Option<i32>>,
}

/// 分类响应
#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub category_type: String,
    pub parent_id: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::models::category::Model> for CategoryResponse {
    fn from(model: crate::models::category::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            slug: model.slug,
            description: model.description,
            category_type: model.category_type,
            parent_id: model.parent_id,
            created_at: model.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: model.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}
