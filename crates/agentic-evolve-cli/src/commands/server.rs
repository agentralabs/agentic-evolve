//! Serve subcommand — start the MCP server.

use anyhow::Result;
use clap::Args;

use std::sync::Arc;
use tokio::sync::Mutex;

use agentic_evolve_mcp::protocol::ProtocolHandler;
use agentic_evolve_mcp::session::SessionManager;
use agentic_evolve_mcp::transport::StdioTransport;

#[derive(Args)]
pub struct ServeArgs;

pub fn run(_args: ServeArgs, data_dir: &str) -> Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let session = SessionManager::new(data_dir)?;
        let session = Arc::new(Mutex::new(session));
        let handler = ProtocolHandler::new(session);
        let transport = StdioTransport::new(handler);
        transport.run().await?;
        Ok(())
    })
}
