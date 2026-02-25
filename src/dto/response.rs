use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

/// 统一 API 响应格式
/// {"code": 0, "msg": "", "data": ...}
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    /// 成功响应，code=0
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data: Some(data),
        }
    }

    /// 带自定义 HTTP 状态码的成功响应
    pub fn success_with_status(data: T, status: StatusCode) -> Response {
        (status, Json(Self::success(data))).into_response()
    }
}

impl ApiResponse<()> {
    /// 成功响应，无 data
    pub fn success_empty() -> Self {
        Self {
            code: 0,
            msg: String::new(),
            data: None,
        }
    }

    /// 错误响应
    pub fn error(code: i32, msg: impl Into<String>) -> Self {
        Self {
            code,
            msg: msg.into(),
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
