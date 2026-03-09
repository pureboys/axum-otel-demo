mod logger;
mod profiling;
mod tracer;

use opentelemetry_sdk::logs::SdkLoggerProvider;
use pyroscope::{PyroscopeAgent, pyroscope::PyroscopeAgentRunning};
use tracing_subscriber::{filter::Targets, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::TelemetryConfig;

/// 可观测性资源句柄，持有需要在关闭时清理的 provider
pub struct TelemetryGuard {
    pub logger_provider: Option<SdkLoggerProvider>,
    pub pyroscope_agent: Option<PyroscopeAgent<PyroscopeAgentRunning>>,
}

/// 初始化全部可观测性组件：tracing + logging + profiling
pub fn init_telemetry(config: &TelemetryConfig) -> TelemetryGuard {
    let filter = Targets::new()
        .with_default(tracing::Level::INFO)
        .with_target("axum_otel_demo", tracing::Level::DEBUG)
        .with_target("tower_http", tracing::Level::DEBUG)
        .with_target("sea_orm", tracing::Level::DEBUG)
        .with_target("pyroscope", tracing::level_filters::LevelFilter::OFF)
        .with_target("pyroscope_pprofrs", tracing::level_filters::LevelFilter::OFF)
        .with_target("Pyroscope", tracing::level_filters::LevelFilter::OFF)
        .with_target("log", tracing::level_filters::LevelFilter::OFF);

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339())
        .with_file(true)
        .with_line_number(true);

    let (telemetry_layer, logging_layer, logger_provider, pyroscope_agent) =
        if config.otel_enabled {
            let resource = tracer::create_resource();

            let otel_tracer = tracer::init_tracer(resource.clone(), &config.otel_endpoint);
            let logger_provider = logger::init_logger(resource.clone(), &config.otel_endpoint);
            let pyroscope_agent = profiling::init_pyroscope(&config.pyroscope_endpoint)
                .start()
                .expect("Failed to start Pyroscope agent");

            let telemetry_layer = tracing_opentelemetry::layer()
                .with_tracer(otel_tracer)
                .with_location(true);

            let logging_layer =
                opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge::new(
                    &logger_provider,
                );

            (
                Some(telemetry_layer),
                Some(logging_layer),
                Some(logger_provider),
                Some(pyroscope_agent),
            )
        } else {
            (None, None, None, None)
        };

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
        if let Some(agent) = self.pyroscope_agent {
            let agent_ready = agent.stop().expect("Failed to stop Pyroscope agent");
            agent_ready.shutdown();
        }

        if let Some(provider) = self.logger_provider {
            if let Err(e) = provider.shutdown() {
                eprintln!("Failed to shutdown logger provider: {:?}", e);
            }
        }
    }
}
