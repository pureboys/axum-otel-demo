mod logger;
mod profiling;
mod tracer;

use opentelemetry_sdk::logs::SdkLoggerProvider;
use pyroscope::{PyroscopeAgent, pyroscope::PyroscopeAgentRunning};
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::AppConfig;

/// 可观测性资源句柄，持有需要在关闭时清理的 provider
pub struct TelemetryGuard {
    pub logger_provider: SdkLoggerProvider,
    pub pyroscope_agent: PyroscopeAgent<PyroscopeAgentRunning>,
}

/// 初始化全部可观测性组件：tracing + logging + profiling
pub fn init_telemetry(config: &AppConfig) -> TelemetryGuard {
    let resource = tracer::create_resource();

    // OTel tracer (spans)
    let otel_tracer = tracer::init_tracer(resource.clone(), &config.otel_endpoint);

    // OTel logger (logs)
    let logger_provider = logger::init_logger(resource.clone(), &config.otel_endpoint);

    // Pyroscope profiling
    let pyroscope_agent = profiling::init_pyroscope(&config.pyroscope_endpoint)
        .start()
        .expect("Failed to start Pyroscope agent");

    // 组装 tracing subscriber layers
    let telemetry_layer = tracing_opentelemetry::layer()
        .with_tracer(otel_tracer)
        .with_location(true);

    // OTel logging layer（experimental_metadata_attributes 自动附加 code.filepath / code.lineno）
    let logging_layer =
        opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(&logger_provider);

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
        .with_file(true)
        .with_line_number(true);

    let filter = Targets::new()
        .with_default(tracing::Level::INFO)
        .with_target("axum_otel_demo", tracing::Level::DEBUG)
        .with_target("tower_http", tracing::Level::DEBUG)
        .with_target("sea_orm", tracing::Level::DEBUG)
        .with_target("pyroscope", tracing::level_filters::LevelFilter::OFF)
        .with_target("pyroscope_pprofrs", tracing::level_filters::LevelFilter::OFF)
        .with_target("Pyroscope", tracing::level_filters::LevelFilter::OFF)
        .with_target("log", tracing::level_filters::LevelFilter::OFF);

    tracing_subscriber::registry()
        .with(filter)
        .with(telemetry_layer)
        .with(logging_layer)
        .with(fmt_layer)
        .init();

    TelemetryGuard {
        logger_provider,
        pyroscope_agent,
    }
}

impl TelemetryGuard {
    /// 优雅关闭所有可观测性组件
    pub fn shutdown(self) {
        let agent_ready = self
            .pyroscope_agent
            .stop()
            .expect("Failed to stop Pyroscope agent");
        agent_ready.shutdown();

        if let Err(e) = self.logger_provider.shutdown() {
            eprintln!("Failed to shutdown logger provider: {:?}", e);
        }
    }
}
