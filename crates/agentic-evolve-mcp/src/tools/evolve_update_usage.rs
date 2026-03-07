//! Tool: evolve_update_usage — Update usage statistics for a pattern.

use std::sync::Arc;
use tokio::sync::Mutex;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::session::SessionManager;
use crate::types::{McpError, McpResult, ToolCallResult, ToolDefinition};

#[derive(Debug, Deserialize)]
struct UsageParams {
    pattern_id: String,
    #[serde(default = "default_domain")]
    domain: String,
    #[serde(default = "default_success")]
    success: bool,
}

fn default_domain() -> String {
    "general".to_string()
}

fn default_success() -> bool {
    true
}

/// Return the tool definition for evolve_update_usage.
pub fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "evolve_update_usage".to_string(),
        description: Some("Update usage statistics for a pattern after it was applied".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "pattern_id": {
                    "type": "string",
                    "description": "The unique pattern identifier"
                },
                "domain": {
                    "type": "string",
                    "default": "general",
                    "description": "Domain where the pattern was used"
                },
                "success": {
                    "type": "boolean",
                    "default": true,
                    "description": "Whether the pattern application was successful"
                }
            },
            "required": ["pattern_id"]
        }),
    }
}

/// Execute the evolve_update_usage tool.
pub async fn execute(
    args: Value,
    session: &Arc<Mutex<SessionManager>>,
) -> McpResult<ToolCallResult> {
    let params: UsageParams =
        serde_json::from_value(args).map_err(|e| McpError::InvalidParams(e.to_string()))?;

    let mut session = session.lock().await;
    session
        .update_usage(&params.pattern_id, &params.domain, params.success)
        .map_err(|e| McpError::AgenticEvolve(e.to_string()))?;

    let pattern = session
        .get_pattern(&params.pattern_id)
        .map_err(|e| McpError::AgenticEvolve(e.to_string()))?;

    Ok(ToolCallResult::json(&json!({
        "pattern_id": params.pattern_id,
        "recorded": true,
        "success": params.success,
        "new_usage_count": pattern.usage_count,
        "new_success_count": pattern.success_count,
        "new_confidence": pattern.confidence,
        "success_rate": pattern.success_rate()
    })))
}
