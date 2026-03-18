//! 后台 API 模块

pub mod auth;
pub mod user;
pub mod product;
pub mod category;
pub mod tag;

use axum::middleware;
use axum::Router;

use crate::app::AppState;
use crate::middleware::auth as auth_middleware;

/// 构建后台公开路由（无需认证）
pub fn public_routes() -> Router<AppState> {
    Router::new()
        .merge(auth::auth_public_routes())
}

/// 构建后台受保护路由（需认证）
pub fn protected_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .merge(auth::auth_protected_routes())
        .merge(user::routes())
        .merge(product::routes())
        .merge(category::routes())
        .merge(tag::routes())
        .layer(middleware::from_fn_with_state(state, auth_middleware::auth_middleware))
}
