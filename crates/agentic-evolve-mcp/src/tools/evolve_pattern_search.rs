//! Tool: evolve_pattern_search — Search patterns by query.

use std::sync::Arc;
use tokio::sync::Mutex;

use serde::Deserialize;
use serde_json::{json, Value};

use crate::session::SessionManager;
use crate::types::{McpError, McpResult, ToolCallResult, ToolDefinition};

#[derive(Debug, Deserialize)]
struct SearchParams {
    query: String,
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_limit() -> usize {
    20
}

/// Return the tool definition for evolve_pattern_search.
pub fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "evolve_pattern_search".to_string(),
        description: Some("Search patterns by query string".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search query (matches name, domain, template, tags)"
                },
                "limit": {
                    "type": "integer",
                    "default": 20,
                    "description": "Maximum number of results to return"
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
            "required": ["query"]
        }),
    }
}

/// Execute the evolve_pattern_search tool.
pub async fn execute(
    args: Value,
    session: &Arc<Mutex<SessionManager>>,
) -> McpResult<ToolCallResult> {
    let params: SearchParams =
        serde_json::from_value(args).map_err(|e| McpError::InvalidParams(e.to_string()))?;

    let session = session.lock().await;
    let results = session.search_patterns(&params.query);

    let limited: Vec<_> = results.into_iter().take(params.limit).collect();
    let patterns: Vec<Value> = limited
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

    Ok(ToolCallResult::json(&json!({
        "count": patterns.len(),
        "patterns": patterns
    })))
}
