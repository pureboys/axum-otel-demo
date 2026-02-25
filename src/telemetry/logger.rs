use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{logs::SdkLoggerProvider, Resource};

/// 初始化 OTel Logger Provider，导出 logs 到 OTLP gRPC
pub fn init_logger(resource: Resource, endpoint: &str) -> SdkLoggerProvider {
    let exporter = opentelemetry_otlp::LogExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()
        .expect("Failed to create OTLP log exporter");

    SdkLoggerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build()
}
