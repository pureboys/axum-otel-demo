//! ALTCHA 人机验证：PoW 挑战与登录侧校验

mod altcha_util;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Json;
use axum::Router;

use crate::app::AppState;
use crate::dto::response::ApiResponse;

pub use altcha_util::verify_client_payload;

/// 返回可供 `<altcha-widget challenge="...">` 拉取的纯 JSON 挑战（无统一 `ApiResponse` 包裹）
async fn get_altcha_challenge() -> impl IntoResponse {
    match altcha_util::create_signed_challenge() {
        Ok(c) => Json(c).into_response(),
        Err(e) => {
            let msg = match e {
                crate::error::AppError::Internal(m) => m,
                _ => "签发挑战失败".into(),
            };
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()>::error(500, msg)),
            )
                .into_response()
        }
    }
}

/// 保留旧路径别名：返回相同挑战 JSON
async fn get_captcha_legacy() -> impl IntoResponse {
    get_altcha_challenge().await
}

/// 构建公共模块路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/altcha/challenge", get(get_altcha_challenge))
        .route("/captcha", get(get_captcha_legacy))
}
