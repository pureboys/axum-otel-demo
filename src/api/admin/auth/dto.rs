use serde::{Deserialize, Serialize};

/// 登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 登录响应
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub admin: AdminInfo,
}

/// 管理员基本信息
#[derive(Debug, Serialize)]
pub struct AdminInfo {
    pub id: i32,
    pub username: String,
    pub nickname: Option<String>,
    pub role: String,
}

/// 当前管理员信息响应
#[derive(Debug, Serialize)]
pub struct AdminInfoResponse {
    pub id: i32,
    pub username: String,
    pub nickname: Option<String>,
    pub role: String,
    pub created_at: String,
}

/// 刷新 Token 请求
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    #[allow(dead_code)]
    pub refresh_token: Option<String>,
}

/// 通用 Token 响应
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub token_type: String,
    pub expires_in: i64,
}
