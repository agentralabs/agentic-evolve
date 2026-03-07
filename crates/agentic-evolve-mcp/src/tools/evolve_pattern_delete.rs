//! Tool: evolve_pattern_delete — Delete a pattern by ID.

use std::sync::Arc;
use tokio::sync::Mutex;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::session::SessionManager;
use crate::types::{McpError, McpResult, ToolCallResult, ToolDefinition};

#[derive(Debug, Deserialize)]
struct DeleteParams {
    pattern_id: String,
}

/// Return the tool definition for evolve_pattern_delete.
pub fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "evolve_pattern_delete".to_string(),
        description: Some("Delete a pattern by ID".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "pattern_id": {
                    "type": "string",
                    "description": "The unique pattern identifier to delete"
                }
            },
            "required": ["pattern_id"]
        }),
    }
}

/// Execute the evolve_pattern_delete tool.
pub async fn execute(
    args: Value,
    session: &Arc<Mutex<SessionManager>>,
) -> McpResult<ToolCallResult> {
    let params: DeleteParams =
        serde_json::from_value(args).map_err(|e| McpError::InvalidParams(e.to_string()))?;

    let mut session = session.lock().await;
    let pattern = session
        .delete_pattern(&params.pattern_id)
        .map_err(|e| McpError::AgenticEvolve(e.to_string()))?;

    Ok(ToolCallResult::json(&json!({
        "deleted": true,
        "pattern_id": pattern.id.as_str(),
        "name": pattern.name,
        "domain": pattern.domain
    })))
}
