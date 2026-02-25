use axum::middleware;
use axum::routing::get;
use axum::Router;
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;
use tracing::Span;

use crate::handlers::user;
use crate::middleware::log_bodies;

/// 共享应用状态
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

/// 构建完整的 Router（路由 + 中间件 + 状态注入）
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(user::root))
        .route("/users", get(user::list_users).post(user::create_user))
        .route(
            "/users/{id}",
            get(user::get_user)
                .put(user::update_user)
                .delete(user::delete_user),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    let method = request.method();
                    let uri = request.uri();
                    let path = uri.path();

                    tracing::info_span!(
                        "http_request",
                        otel.name = %format!("{} {}", method, path),
                        http.method = %method,
                        http.uri = %uri,
                        http.route = %path,
                        trace_id = tracing::field::Empty,
                        span_id = tracing::field::Empty,
                    )
                })
                .on_response(
                    |response: &axum::http::Response<_>,
                     latency: std::time::Duration,
                     _span: &Span| {
                        tracing::info!(
                            status = %response.status().as_u16(),
                            latency_ms = %latency.as_millis(),
                            "response"
                        );
                    },
                ),
        )
        .layer(middleware::from_fn(log_bodies))
        .with_state(state)
}
