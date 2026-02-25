/// 应用配置，从环境变量读取，提供默认值
pub struct AppConfig {
    pub server_addr: String,
    pub database_url: String,
    pub otel_endpoint: String,
    pub pyroscope_endpoint: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            server_addr: std::env::var("SERVER_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:8000".into()),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:./demo.db?mode=rwc".into()),
            otel_endpoint: std::env::var("OTEL_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:4317".into()),
            pyroscope_endpoint: std::env::var("PYROSCOPE_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:4040".into()),
        }
    }
}
