use axum::middleware;
use axum::routing::get;
use axum::Router;
use sea_orm::DatabaseConnection;

use crate::api::admin::{auth as admin_auth, user as admin_user};
use crate::api::common;
use crate::api::front::user as front_user;
use crate::middleware::{auth, log_bodies};

/// 共享应用状态
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

/// 构建完整的 Router（路由 + 中间件 + 状态注入）
pub fn create_router(state: AppState) -> Router {
    // 前台路由
    let front_routes = Router::new()
        .merge(front_user::routes());

    // 后台认证公开路由
    let auth_public = admin_auth::public_routes();

    // 后台受保护路由（需要认证）
    let admin_protected = Router::new()
        .merge(admin_auth::protected_routes())
        .merge(admin_user::routes())
        .layer(middleware::from_fn_with_state(state.clone(), auth::auth_middleware));

    // 公共路由
    let common_routes = common::routes();

    Router::new()
        // 前台 API - /api/*
        .nest("/api", front_routes)
        // 后台认证公开 API - /api/admin/auth/login
        .nest("/api/admin/auth", auth_public)
        // 后台受保护 API - /api/admin/* (需要认证)
        .nest("/api/admin", admin_protected)
        // 公共 API - /api/common/*
        .nest("/api/common", common_routes)
        .layer(middleware::from_fn(log_bodies))
        .with_state(state)
}
