//! Tool: evolve_compose — Compose multiple patterns together.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::session::SessionManager;
use crate::types::{McpError, McpResult, ToolCallResult, ToolDefinition};

#[derive(Debug, Deserialize)]
struct ComposeParams {
    pattern_ids: Vec<String>,
    #[serde(default)]
    bindings: HashMap<String, String>,
}

/// Return the tool definition for evolve_compose.
pub fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "evolve_compose".to_string(),
        description: Some("Compose multiple patterns together with variable bindings".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "pattern_ids": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "List of pattern IDs to compose together"
                },
                "bindings": {
                    "type": "object",
                    "additionalProperties": { "type": "string" },
                    "description": "Variable bindings (variable_name -> value)"
                }
            },
            "required": ["pattern_ids"]
        }),
    }
}

/// Execute the evolve_compose tool.
pub async fn execute(
    args: Value,
    session: &Arc<Mutex<SessionManager>>,
) -> McpResult<ToolCallResult> {
    let params: ComposeParams =
        serde_json::from_value(args).map_err(|e| McpError::InvalidParams(e.to_string()))?;

    if params.pattern_ids.is_empty() {
        return Err(McpError::InvalidParams(
            "pattern_ids must contain at least one pattern ID".to_string(),
        ));
    }

    let session = session.lock().await;
    let result = session
        .compose(&params.pattern_ids, &params.bindings)
        .map_err(|e| McpError::AgenticEvolve(e.to_string()))?;

    Ok(ToolCallResult::json(&json!({
        "code": result.code,
        "patterns_used": result.patterns_used,
        "coverage": result.coverage,
        "gaps": result.gaps
    })))
}
