//! Tool: evolve_pattern_list — List all stored patterns.

use std::sync::Arc;
use tokio::sync::Mutex;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::session::SessionManager;
use crate::types::{McpError, McpResult, ToolCallResult, ToolDefinition};

#[derive(Debug, Deserialize)]
struct ListParams {
    #[serde(default)]
    domain: Option<String>,
    #[serde(default)]
    language: Option<String>,
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_limit() -> usize {
    50
}

/// Return the tool definition for evolve_pattern_list.
pub fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "evolve_pattern_list".to_string(),
        description: Some("List all stored patterns with optional filtering".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "domain": {
                    "type": "string",
                    "description": "Filter by domain"
                },
                "language": {
                    "type": "string",
                    "description": "Filter by programming language"
                },
                "limit": {
                    "type": "integer",
                    "default": 50,
                    "description": "Maximum number of results"
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
            }
        }),
    }
}

/// Execute the evolve_pattern_list tool.
pub async fn execute(
    args: Value,
    session: &Arc<Mutex<SessionManager>>,
) -> McpResult<ToolCallResult> {
    let params: ListParams =
        serde_json::from_value(args).map_err(|e| McpError::InvalidParams(e.to_string()))?;

    let session = session.lock().await;
    let all = session.list_patterns();

    let filtered: Vec<_> = all
        .into_iter()
        .filter(|p| {
            if let Some(domain) = &params.domain {
                if p.domain.to_lowercase() != domain.to_lowercase() {
                    return false;
                }
            }
            if let Some(language) = &params.language {
                if p.language.as_str() != language.to_lowercase() {
                    return false;
                }
            }
            true
        })
        .take(params.limit)
        .collect();

    let patterns: Vec<Value> = filtered
        .iter()
        .map(|p| {
            json!({
                "pattern_id": p.id.as_str(),
                "name": p.name,
                "domain": p.domain,
                "language": p.language.as_str(),
                "confidence": p.confidence,
                "usage_count": p.usage_count,
                "tags": p.tags
            })
        })
        .collect();

    let total = session.pattern_count();

    Ok(ToolCallResult::json(&json!({
        "total_in_library": total,
        "returned": patterns.len(),
        "patterns": patterns
    })))
}
