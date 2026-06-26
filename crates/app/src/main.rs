#![allow(warnings, clippy::all)]

//! Forge — unified AI agent CLI.

use clap::Parser;
use forge_engine::config::Config;
use forge_webui::{self, state::AppState};
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(name = "forge", about = "Unified AI agent with free LLM routing", version)]
struct Args {
    /// Host to bind to
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
    /// Port to bind to
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
    /// Log level
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| args.log_level.clone().into()),
        )
        .init();

    let config = Config::load().unwrap_or_default();
    let agent = forge_engine::Agent::new(config.clone());
    let state = AppState::new(agent);
    let addr: SocketAddr = format!("{}:{}", args.host, args.port).parse()?;

    tracing::info!("Forge starting on {}", addr);
    forge_webui::serve(state, addr).await?;

    Ok(())
}
