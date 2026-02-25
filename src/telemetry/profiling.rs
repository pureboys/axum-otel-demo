use pyroscope::PyroscopeAgent;
use pyroscope::pyroscope::PyroscopeAgentReady;
use pyroscope_pprofrs::{pprof_backend, PprofConfig};

/// 初始化 Pyroscope Agent（CPU profiling，100Hz 采样）
pub fn init_pyroscope(endpoint: &str) -> PyroscopeAgent<PyroscopeAgentReady> {
    PyroscopeAgent::builder(endpoint, "axum-otel-demo")
        .tags(vec![
            ("version", env!("CARGO_PKG_VERSION")),
            ("env", "development"),
        ])
        .backend(pprof_backend(PprofConfig::new().sample_rate(100)))
        .build()
        .expect("Failed to create Pyroscope agent")
}
