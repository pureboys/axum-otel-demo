use axum::middleware;
use axum::routing::get;
use axum::Router;
use sea_orm::DatabaseConnection;

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
        .layer(middleware::from_fn(log_bodies))
        .with_state(state)
}
