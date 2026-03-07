//! Tool registration and dispatch.

use std::sync::Arc;
use tokio::sync::Mutex;

use serde_json::Value;

use crate::session::SessionManager;
use crate::types::{McpError, McpResult, ToolCallResult, ToolDefinition};

use super::{
    evolve_compose, evolve_confidence, evolve_coverage, evolve_crystallize, evolve_get_body,
    evolve_match_context, evolve_match_signature, evolve_optimize, evolve_pattern_delete,
    evolve_pattern_get, evolve_pattern_list, evolve_pattern_search, evolve_pattern_store,
    evolve_update_usage,
};

/// Registry of all available MCP tools.
pub struct ToolRegistry;

impl ToolRegistry {
    /// List all available tool definitions.
    pub fn list_tools() -> Vec<ToolDefinition> {
        vec![
            evolve_pattern_store::definition(),
            evolve_pattern_get::definition(),
            evolve_pattern_search::definition(),
            evolve_pattern_list::definition(),
            evolve_pattern_delete::definition(),
            evolve_match_signature::definition(),
            evolve_match_context::definition(),
            evolve_crystallize::definition(),
            evolve_get_body::definition(),
            evolve_compose::definition(),
            evolve_coverage::definition(),
            evolve_confidence::definition(),
            evolve_update_usage::definition(),
            evolve_optimize::definition(),
        ]
    }

    /// Dispatch a tool call to the appropriate handler.
    pub async fn call(
        name: &str,
        arguments: Option<Value>,
        session: &Arc<Mutex<SessionManager>>,
    ) -> McpResult<ToolCallResult> {
        let args = arguments.unwrap_or(Value::Object(serde_json::Map::new()));

        match name {
            "evolve_pattern_store" => evolve_pattern_store::execute(args, session).await,
            "evolve_pattern_get" => evolve_pattern_get::execute(args, session).await,
            "evolve_pattern_search" => evolve_pattern_search::execute(args, session).await,
            "evolve_pattern_list" => evolve_pattern_list::execute(args, session).await,
            "evolve_pattern_delete" => evolve_pattern_delete::execute(args, session).await,
            "evolve_match_signature" => evolve_match_signature::execute(args, session).await,
            "evolve_match_context" => evolve_match_context::execute(args, session).await,
            "evolve_crystallize" => evolve_crystallize::execute(args, session).await,
            "evolve_get_body" => evolve_get_body::execute(args, session).await,
            "evolve_compose" => evolve_compose::execute(args, session).await,
            "evolve_coverage" => evolve_coverage::execute(args, session).await,
            "evolve_confidence" => evolve_confidence::execute(args, session).await,
            "evolve_update_usage" => evolve_update_usage::execute(args, session).await,
            "evolve_optimize" => evolve_optimize::execute(args, session).await,
            _ => Err(McpError::ToolNotFound(name.to_string())),
        }
    }
}
