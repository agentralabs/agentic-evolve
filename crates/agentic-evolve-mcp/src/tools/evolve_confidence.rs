//! Tool: evolve_confidence — Get confidence score for a pattern.

use std::sync::Arc;
use tokio::sync::Mutex;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::session::SessionManager;
use crate::types::{McpError, McpResult, ToolCallResult, ToolDefinition};

#[derive(Debug, Deserialize)]
struct ConfidenceParams {
    pattern_id: String,
}

/// Return the tool definition for evolve_confidence.
pub fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "evolve_confidence".to_string(),
        description: Some("Get confidence score and usage analytics for a pattern".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "pattern_id": {
                    "type": "string",
                    "description": "The unique pattern identifier"
                },
                "include_content": {
                    "type": "boolean",
                    "default": false,
                    "description": "Include full template content in response"
                },
                "intent": {
                    "type": "string",
                    "enum": ["exists", "ids", "summary", "full"],
                    "description": "Response detail level"
                },
                "since": {
                    "type": "integer",
                    "description": "Only return data changed after this Unix timestamp"
                },
                "token_budget": {
                    "type": "integer",
                    "description": "Maximum token budget for the response"
                },
                "max_results": {
                    "type": "integer",
                    "default": 10,
                    "description": "Maximum number of results to return"
                },
                "cursor": {
                    "type": "string",
                    "description": "Pagination cursor from a previous response"
                }
            },
            "required": ["pattern_id"]
        }),
    }
}

/// Execute the evolve_confidence tool.
pub async fn execute(
    args: Value,
    session: &Arc<Mutex<SessionManager>>,
) -> McpResult<ToolCallResult> {
    let params: ConfidenceParams =
        serde_json::from_value(args).map_err(|e| McpError::InvalidParams(e.to_string()))?;

    let session = session.lock().await;
    let report = session
        .confidence(&params.pattern_id)
        .map_err(|e| McpError::AgenticEvolve(e.to_string()))?;

    Ok(ToolCallResult::json(&json!({
        "pattern_id": report.pattern_id,
        "base_confidence": report.base_confidence,
        "usage_success_rate": report.usage_success_rate,
        "tracker_success_rate": report.tracker_success_rate,
        "promotion_decision": report.promotion_decision,
        "usage_count": report.usage_count,
        "success_count": report.success_count
    })))
}
