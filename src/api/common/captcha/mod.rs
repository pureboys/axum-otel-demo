//! 验证码接口

use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

use crate::dto::response::ApiResponse;

/// 获取验证码
pub async fn get_captcha() -> impl IntoResponse {
    ApiResponse::success("captcha_id_placeholder")
}

/// 构建公共模块路由
pub fn routes() -> Router<crate::app::AppState> {
    Router::new()
        .route("/captcha", get(get_captcha))
}
