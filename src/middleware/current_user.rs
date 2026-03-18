//! 当前登录用户信息

/// 当前登录管理员信息（从 Token 解析）
#[derive(Clone, Debug)]
pub struct CurrentUser {
    pub id: i32,
    pub username: String,
    pub nickname: Option<String>,
    pub role: String,
    pub created_at: String,
    pub token: String,
}
