use serde::{Deserialize, Serialize};

/// 创建产品请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
    pub category_id: i32,
    pub image_url: String,
    pub status: Option<i8>,
}

/// 更新产品请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
    pub category_id: Option<i32>,
    pub image_url: Option<String>,
    pub status: Option<i8>,
}

/// 设置产品标签请求
#[derive(Debug, Deserialize, Serialize)]
pub struct SetProductTagsRequest {
    pub tag_ids: Vec<i32>,
}

/// 产品响应
#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
    pub category_id: i32,
    pub image_url: String,
    pub status: i8,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::models::product::Model> for ProductResponse {
    fn from(model: crate::models::product::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            price: model.price.to_string().parse().unwrap_or(0.0),
            stock: model.stock,
            category_id: model.category_id,
            image_url: model.image_url,
            status: model.status,
            created_at: model.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: model.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

/// 标签响应（供产品模块使用）
#[derive(Debug, Serialize)]
pub struct TagResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

impl From<crate::models::tag::Model> for TagResponse {
    fn from(model: crate::models::tag::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            slug: model.slug,
        }
    }
}

/// 产品（含标签）响应
#[derive(Debug, Serialize)]
pub struct ProductWithTagsResponse {
    pub product: ProductResponse,
    pub tags: Vec<TagResponse>,
}
