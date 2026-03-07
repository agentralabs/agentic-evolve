//! Tool: evolve_pattern_get — Get a pattern by ID.

use std::sync::Arc;
use tokio::sync::Mutex;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::session::SessionManager;
use crate::types::{McpError, McpResult, ToolCallResult, ToolDefinition};

#[derive(Debug, Deserialize)]
struct GetParams {
    pattern_id: String,
}

/// Return the tool definition for evolve_pattern_get.
pub fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "evolve_pattern_get".to_string(),
        description: Some("Get a pattern by ID".to_string()),
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

/// Execute the evolve_pattern_get tool.
pub async fn execute(
    args: Value,
    session: &Arc<Mutex<SessionManager>>,
) -> McpResult<ToolCallResult> {
    let params: GetParams =
        serde_json::from_value(args).map_err(|e| McpError::InvalidParams(e.to_string()))?;

    let session = session.lock().await;
    let pattern = session
        .get_pattern(&params.pattern_id)
        .map_err(|e| McpError::AgenticEvolve(e.to_string()))?;

    Ok(ToolCallResult::json(&json!({
        "pattern_id": pattern.id.as_str(),
        "name": pattern.name,
        "domain": pattern.domain,
        "language": pattern.language.as_str(),
        "template": pattern.template,
        "variables": pattern.variables,
        "confidence": pattern.confidence,
        "usage_count": pattern.usage_count,
        "success_count": pattern.success_count,
        "success_rate": pattern.success_rate(),
        "version": pattern.version,
        "tags": pattern.tags,
        "created_at": pattern.created_at,
        "updated_at": pattern.updated_at,
        "last_used": pattern.last_used,
        "signature": {
            "name": pattern.signature.name,
            "params": pattern.signature.params,
            "return_type": pattern.signature.return_type,
            "language": pattern.signature.language.as_str(),
            "is_async": pattern.signature.is_async
        }
    })))
}
