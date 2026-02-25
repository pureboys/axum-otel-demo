mod app;
mod config;
mod db;
mod dto;
mod error;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod services;
mod shutdown;
mod telemetry;

use tokio::net::TcpListener;

use crate::app::AppState;
use crate::config::AppConfig;

#[tokio::main]
async fn main() {
    // 读取配置
    let config = AppConfig::from_env();

    // 初始化可观测性（tracing + logging + profiling）
    let telemetry_guard = telemetry::init_telemetry(&config);

    tracing::info!("Starting server with OpenTelemetry tracing, logging and Pyroscope profiling enabled");

    // 初始化数据库
    let db = db::init_db(&config.database_url)
        .await
        .expect("Failed to initialize database");
    tracing::info!("Database initialized");

    // 构建应用
    let state = AppState { db };
    let router = app::create_router(state);

    // 启动服务器
    let listener = TcpListener::bind(&config.server_addr).await.unwrap();
    tracing::info!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await
        .unwrap();

    // 优雅关闭
    tracing::info!("Shutting down monitoring providers...");
    telemetry_guard.shutdown();
    println!("Server shutdown complete");
}
