//! Tool: evolve_optimize — Optimize pattern storage.

use std::sync::Arc;
use tokio::sync::Mutex;

use serde_json::{json, Value};

use crate::session::SessionManager;
use crate::types::{McpError, McpResult, ToolCallResult, ToolDefinition};

/// Return the tool definition for evolve_optimize.
pub fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "evolve_optimize".to_string(),
        description: Some("Optimize pattern storage by applying decay, promotions, and cache cleanup".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {}
        }),
    }
}

/// Execute the evolve_optimize tool.
pub async fn execute(
    _args: Value,
    session: &Arc<Mutex<SessionManager>>,
) -> McpResult<ToolCallResult> {
    let mut session = session.lock().await;
    let summary = session
        .optimize()
        .map_err(|e| McpError::AgenticEvolve(e.to_string()))?;

    Ok(ToolCallResult::json(&json!({
        "patterns_total": summary.patterns_total,
        "duplicates_found": summary.duplicates_found,
        "prunable": summary.prunable,
        "decay_healthy": summary.decay_healthy,
        "decay_decaying": summary.decay_decaying,
        "decay_critical": summary.decay_critical,
        "patterns_decayed": summary.patterns_decayed,
        "patterns_promoted": summary.patterns_promoted,
        "patterns_demoted": summary.patterns_demoted,
        "cache_cleared": summary.cache_cleared
    })))
}
