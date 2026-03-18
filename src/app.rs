use axum::middleware;
use axum::Router;
use sea_orm::DatabaseConnection;

use crate::api::admin;
use crate::api::common;
use crate::api::front;
use crate::middleware::log_bodies;

/// 共享应用状态
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

/// 构建完整的 Router（路由 + 中间件 + 状态注入）
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // 前台 API - /api/*
        .nest("/api", front::routes())
        // 后台公开 API - /api/admin/*
        .nest("/api/admin", admin::public_routes())
        // 后台受保护 API - /api/admin/* (需要认证)
        .nest("/api/admin", admin::protected_routes(state.clone()))
        // 公共 API - /api/common/*
        .nest("/api/common", common::routes())
        .layer(middleware::from_fn(log_bodies))
        .with_state(state)
}
