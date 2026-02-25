use axum::body::Body;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use http_body_util::BodyExt;

/// 记录请求和响应 body 的中间件
pub async fn log_bodies(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    // 提取并记录请求 body
    let (parts, body) = request.into_parts();
    let bytes = body.collect().await.unwrap_or_default().to_bytes();

    if !bytes.is_empty() {
        let body_str = String::from_utf8_lossy(&bytes);
        tracing::debug!(
            http.method = %method,
            http.uri = %uri,
            request.body = %body_str,
            "request body"
        );
    }

    let request = Request::from_parts(parts, Body::from(bytes));

    // 调用下游处理
    let response = next.run(request).await;

    // 提取并记录响应 body
    let (parts, body) = response.into_parts();
    let bytes = body.collect().await.unwrap_or_default().to_bytes();

    if !bytes.is_empty() {
        let body_str = String::from_utf8_lossy(&bytes);
        tracing::debug!(
            http.method = %method,
            http.uri = %uri,
            http.status = %parts.status.as_u16(),
            response.body = %body_str,
            "response body"
        );
    }

    Response::from_parts(parts, Body::from(bytes))
}
