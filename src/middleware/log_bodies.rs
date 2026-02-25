use axum::body::Body;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use http_body_util::BodyExt;
use opentelemetry::trace::TraceContextExt;
use tracing::Instrument;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// 记录请求/响应 body、trace 上下文、响应状态与延迟的中间件
pub async fn log_bodies(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let path = uri.path().to_owned();

    // 创建 tracing span，替代原来的 TraceLayer::make_span_with
    let span: tracing::Span = tracing::info_span!(
        "http_request",
        otel.name = %format!("{} {}", method, path),
        http.method = %method,
        http.uri = %uri,
        http.route = %path,
        trace_id = tracing::field::Empty,
        span_id = tracing::field::Empty,
    );

    // 进入 span 后从 OTel context 提取 trace_id / span_id
    let _enter = span.enter();
    let otel_ctx = span.context();
    let otel_span = otel_ctx.span();
    let span_context = otel_span.span_context();
    if span_context.is_valid() {
        span.record("trace_id", span_context.trace_id().to_string());
        span.record("span_id", span_context.span_id().to_string());
    }
    drop(_enter);

    async move {
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

        // 调用下游处理并计时
        let start = std::time::Instant::now();
        let response = next.run(request).await;
        let latency = start.elapsed();

        // 提取并记录响应 body
        let (parts, body) = response.into_parts();
        let bytes = body.collect().await.unwrap_or_default().to_bytes();

        if !bytes.is_empty() {
            let body_str = String::from_utf8_lossy(&bytes);
            tracing::debug!(
                http.method = %method,
                http.uri = %uri,
                status = %parts.status.as_u16(),
                latency_ms = %latency.as_millis(),
                response.body = %body_str,
                "response body"
            );
        } else {
            tracing::debug!(
                status = %parts.status.as_u16(),
                latency_ms = %latency.as_millis(),
                "response body"
            );
        }

        Response::from_parts(parts, Body::from(bytes))
    }
    .instrument(span)
    .await
}
