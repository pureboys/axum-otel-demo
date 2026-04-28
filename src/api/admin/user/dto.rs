use serde::{Deserialize, Serialize};

/// 创建管理员请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAdminRequest {
    pub username: String,
    pub password: String,
    pub nickname: Option<String>,
    pub role: Option<String>,
}

/// 更新管理员请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateAdminRequest {
    pub nickname: Option<String>,
    pub role: Option<String>,
    pub status: Option<i8>,
}

/// 修改密码请求
#[derive(Debug, Deserialize, Serialize)]
pub struct ChangePasswordRequest {
    pub new_password: String,
}

/// 管理员响应
#[derive(Debug, Serialize)]
pub struct AdminUserResponse {
    pub id: i32,
    pub username: String,
    pub nickname: Option<String>,
    pub role: String,
    pub status: i8,
    pub last_login_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<crate::models::admin::Model> for AdminUserResponse {
    fn from(model: crate::models::admin::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            nickname: model.nickname,
            role: model.role,
            status: model.status,
            last_login_at: model.last_login_at.map(|t| t.to_string()),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
