use serde::{Deserialize, Serialize};

/// 创建用户请求
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
}

/// 更新用户请求
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
}

/// 用户响应
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
}

impl From<crate::models::user::Model> for UserResponse {
    fn from(model: crate::models::user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
        }
    }
}
