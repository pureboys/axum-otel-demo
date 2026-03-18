//! 文件上传接口

use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;

use crate::dto::response::ApiResponse;

/// 上传文件
pub async fn upload_file() -> impl IntoResponse {
    ApiResponse::success("file_url_placeholder")
}

/// 构建公共模块路由
pub fn routes() -> Router<crate::app::AppState> {
    Router::new()
        .route("/upload", post(upload_file))
}
