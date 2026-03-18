use std::sync::OnceLock;

use config::{Config, File};
use serde::Deserialize;

/// 全局原始配置对象，业务代码可直接按 key 动态取值
pub static RAW_CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub telemetry: TelemetryConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub addr: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct TelemetryConfig {
    #[serde(default = "default_true")]
    pub otel_enabled: bool,
    pub otel_endpoint: String,
    pub pyroscope_endpoint: String,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct JwtConfig {
    pub secret: String,
    pub expire_seconds: i64,
}

impl AppConfig {
    pub fn from_file(env: &str) -> Self {
        let config = Config::builder()
            .add_source(File::with_name(&format!("config/{env}")))
            .build()
            .unwrap_or_else(|e| panic!("Failed to load config for env '{env}': {e}"));

        RAW_CONFIG
            .set(config.clone())
            .expect("Config already initialized");

        config
            .try_deserialize()
            .unwrap_or_else(|e| panic!("Failed to deserialize config: {e}"))
    }
}
