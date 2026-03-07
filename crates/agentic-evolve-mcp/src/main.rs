//! AgenticEvolve MCP Server — entry point.

use std::sync::Arc;
use tokio::sync::Mutex;

use clap::Parser;

use agentic_evolve_mcp::protocol::ProtocolHandler;
use agentic_evolve_mcp::session::SessionManager;
use agentic_evolve_mcp::transport::StdioTransport;

#[derive(Parser)]
#[command(name = "agentic-evolve-mcp")]
#[command(about = "MCP server for AgenticEvolve pattern library")]
struct Cli {
    /// Directory for persistent pattern storage.
    #[arg(long, default_value = ".agentic/evolve")]
    data_dir: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();
    let session = SessionManager::new(&cli.data_dir)?;
    let session = Arc::new(Mutex::new(session));
    let handler = ProtocolHandler::new(session);
    let transport = StdioTransport::new(handler);
    transport.run().await?;
    Ok(())
}
