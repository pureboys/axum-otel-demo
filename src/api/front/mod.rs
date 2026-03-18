//! 前台 API 模块

pub mod user;

use axum::Router;

use crate::app::AppState;

/// 构建前台路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(user::routes())
}
