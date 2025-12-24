use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use opentelemetry::{trace::TracerProvider, KeyValue};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_sdk::{
    logs::SdkLoggerProvider,
    trace::SdkTracerProvider,
    Resource,
};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_semantic_conventions::resource::{SERVICE_NAME, SERVICE_VERSION};
use pyroscope::{PyroscopeAgent, pyroscope::PyroscopeAgentReady};
use pyroscope_pprofrs::{pprof_backend, PprofConfig};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::Span;
use tracing_subscriber::{
    filter::Targets,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

/// 创建共享的 Resource
fn create_resource() -> Resource {
    Resource::builder()
        .with_attributes([
            KeyValue::new(SERVICE_NAME, "axum-otel-demo"),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
        ])
        .build()
}

/// 初始化 OpenTelemetry Tracer
fn init_tracer(resource: Resource) -> opentelemetry_sdk::trace::SdkTracer {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317")
        .build()
        .expect("Failed to create OTLP span exporter");

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build();

    let tracer = provider.tracer("axum-otel-demo");
    opentelemetry::global::set_tracer_provider(provider);
    tracer
}

/// 初始化 OpenTelemetry Logger
fn init_logger(resource: Resource) -> SdkLoggerProvider {
    let exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317")
        .build()
        .expect("Failed to create OTLP log exporter");

    SdkLoggerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build()
}

/// 初始化 Pyroscope Agent
fn init_pyroscope() -> PyroscopeAgent<PyroscopeAgentReady> {
    PyroscopeAgent::builder("http://localhost:4040", "axum-otel-demo")
        .tags(vec![
            ("version", env!("CARGO_PKG_VERSION")),
            ("env", "development"),
        ])
        .backend(pprof_backend(PprofConfig::new().sample_rate(100)))
        .build()
        .expect("Failed to create Pyroscope agent")
}

#[tokio::main]
async fn main() {
    // 创建共享资源
    let resource = create_resource();

    // 初始化 OpenTelemetry tracer
    let tracer = init_tracer(resource.clone());

    // 初始化 OpenTelemetry logger
    let logger_provider = init_logger(resource.clone());

    // 初始化 Pyroscope agent
    let pyroscope_agent = init_pyroscope();
    let pyroscope_agent_running = pyroscope_agent.start().expect("Failed to start Pyroscope agent");

    // 创建 OpenTelemetry tracing layer (for spans)
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    // 创建 OpenTelemetry logging layer (for logs)
    // 启用 experimental_use_tracing_span_context feature 后会自动关联 trace context
    let logging_layer = OpenTelemetryTracingBridge::new(&logger_provider);

    // 创建可读格式的 fmt layer，使用本地时间
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339());

    // 使用 Targets filter 过滤日志
    let filter = Targets::new()
        .with_default(tracing::Level::INFO)
        .with_target("rust_demo", tracing::Level::DEBUG)
        .with_target("tower_http", tracing::Level::DEBUG)
        .with_target("pyroscope", tracing::level_filters::LevelFilter::OFF)
        .with_target("pyroscope_pprofrs", tracing::level_filters::LevelFilter::OFF)
        .with_target("Pyroscope", tracing::level_filters::LevelFilter::OFF)
        .with_target("log", tracing::level_filters::LevelFilter::OFF);

    // 初始化 tracing subscriber
    tracing_subscriber::registry()
        .with(filter)
        .with(telemetry_layer)
        .with(logging_layer)
        .with(fmt_layer)
        .init();

    tracing::info!("Starting server with OpenTelemetry tracing, logging and Pyroscope profiling enabled");

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
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
                .on_response(|response: &axum::http::Response<_>, latency: std::time::Duration, _span: &Span| {
                    tracing::info!(
                        status = %response.status().as_u16(),
                        latency_ms = %latency.as_millis(),
                        "response"
                    );
                }),
        );

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();
    tracing::info!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    // 优雅关闭
    tracing::info!("Shutting down monitoring providers...");

    let pyroscope_agent_ready = pyroscope_agent_running.stop().expect("Failed to stop Pyroscope agent");
    pyroscope_agent_ready.shutdown();

    if let Err(e) = logger_provider.shutdown() {
        eprintln!("Failed to shutdown logger provider: {:?}", e);
    }

    println!("Server shutdown complete");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C, shutting down...");
        },
        _ = terminate => {
            tracing::info!("Received terminate signal, shutting down...");
        },
    }
}

#[tracing::instrument]
async fn root() -> &'static str {
    tracing::debug!("Handling root request");
    "Hello, World!"
}

#[tracing::instrument(skip(payload))]
async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    tracing::debug!(username = %payload.username, "Creating new user");
    let user = User::new(1111, payload.username);
    tracing::info!(user_id = user.id, "User created successfully");
    (StatusCode::CREATED, Json(user))
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Debug, Serialize)]
struct User {
    id: i32,
    username: String,
}

impl User {
    fn new(id: i32, username: String) -> Self {
        Self { id, username }
    }
}
