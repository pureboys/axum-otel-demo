use opentelemetry::{trace::TracerProvider, KeyValue};
use opentelemetry_sdk::{trace::SdkTracerProvider, Resource};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_semantic_conventions::resource::{SERVICE_NAME, SERVICE_VERSION};

/// 创建共享的 OTel Resource
pub fn create_resource() -> Resource {
    Resource::builder()
        .with_attributes([
            KeyValue::new(SERVICE_NAME, "axum-otel-demo"),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
        ])
        .build()
}

/// 初始化 OTel Tracer Provider，导出 spans 到 OTLP gRPC
pub fn init_tracer(
    resource: Resource,
    endpoint: &str,
) -> opentelemetry_sdk::trace::SdkTracer {
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
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
