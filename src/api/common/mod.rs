//! 公共 API 模块 - 前后台共用的接口

pub mod captcha;
pub mod upload;

use axum::Router;

use crate::app::AppState;

/// 构建公共模块路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(captcha::routes())
        .merge(upload::routes())
}
