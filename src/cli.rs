use clap::Parser;

#[derive(Parser)]
#[command(about = "Axum OpenTelemetry Demo Server")]
pub struct Cli {
    /// 运行环境，对应 config/{env}.toml 配置文件
    #[arg(long, default_value = "dev")]
    pub env: String,
}
