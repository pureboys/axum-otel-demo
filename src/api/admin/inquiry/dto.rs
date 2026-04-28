use serde::{Deserialize, Serialize};

/// 提交询盘请求（公开接口）
#[derive(Debug, Deserialize)]
pub struct CreateInquiryRequest {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub message: String,
    pub product_id: Option<i32>,
    pub product_name: Option<String>,
}

/// 询盘响应
#[derive(Debug, Serialize)]
pub struct InquiryResponse {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub message: String,
    pub product_id: Option<i32>,
    pub product_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::models::inquiry::Model> for InquiryResponse {
    fn from(m: crate::models::inquiry::Model) -> Self {
        Self {
            id: m.id,
            name: m.name,
            email: m.email,
            phone: m.phone,
            message: m.message,
            product_id: m.product_id,
            product_name: m.product_name,
            created_at: m.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
            updated_at: m.updated_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}

/// 批量删除请求
#[derive(Debug, Deserialize)]
pub struct BatchDeleteRequest {
    pub ids: Vec<i32>,
}

/// 分页询盘响应
#[derive(Debug, Serialize)]
pub struct PaginatedInquiryResponse {
    pub items: Vec<InquiryResponse>,
    pub total: u32,
    pub page: u32,
    pub limit: u32,
}
