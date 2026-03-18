//! 认证中间件

use axum::{
    extract::State,
    http::{header::AUTHORIZATION, Request},
    middleware::Next,
    response::IntoResponse,
};

use crate::api::admin::auth::service::AuthService;
use crate::app::AppState;
use crate::error::AppError;
use crate::middleware::CurrentUser;

/// 从 Authorization Header 解析 Token
fn extract_token(authorization: &str) -> Option<&str> {
    if authorization.starts_with("Bearer ") {
        Some(&authorization[7..])
    } else {
        None
    }
}

/// 认证中间件 - 验证 Token 并注入 CurrentUser
pub async fn auth_middleware(
    state: State<AppState>,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    // 获取 Token
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| extract_token(v))
        .ok_or_else(|| AppError::Unauthorized("缺少 Authorization header".into()))?;

    // 验证 Token
    let claims = AuthService::verify_token(token)?;

    // 查询管理员信息
    let admin = crate::repositories::admin::AdminRepository::find_by_id(&state.db, claims.sub.parse().unwrap_or(0))
        .await
        .map_err(|_| AppError::Unauthorized("用户不存在".into()))?
        .ok_or_else(|| AppError::Unauthorized("用户不存在".into()))?;

    // 检查状态
    if admin.status == 0 {
        return Err(AppError::Unauthorized("账号已被禁用".into()));
    }

    // 构建 CurrentUser
    let current_user = CurrentUser {
        id: admin.id,
        username: admin.username.clone(),
        nickname: admin.nickname,
        role: admin.role,
        created_at: admin.created_at.to_string(),
        token: token.to_string(),
    };

    // 注入到请求扩展中
    request.extensions_mut().insert(current_user);

    Ok(next.run(request).await)
}
