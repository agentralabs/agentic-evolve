//! AgenticEvolve MCP Server — pattern library access for any MCP-compatible LLM client.
//!
//! This library implements an MCP (Model Context Protocol) server that exposes
//! AgenticEvolve pattern library functionality to any MCP-compatible LLM client.

pub mod protocol;
pub mod session;
pub mod tools;
pub mod transport;
pub mod types;

pub use protocol::ProtocolHandler;
pub use session::SessionManager;
pub use transport::StdioTransport;
