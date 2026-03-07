//! Tool: evolve_get_body — Get function body from best matching pattern.

use std::sync::Arc;
use tokio::sync::Mutex;

use serde::Deserialize;
use serde_json::{json, Value};

use agentic_evolve_core::types::match_result::MatchContext;
use agentic_evolve_core::types::pattern::{
    FunctionSignature, Language, ParamSignature, Visibility,
};

use crate::session::SessionManager;
use crate::types::{McpError, McpResult, ToolCallResult, ToolDefinition};

#[derive(Debug, Deserialize)]
struct GetBodyParams {
    name: String,
    language: String,
    #[serde(default)]
    params: Vec<ParamInput>,
    #[serde(default)]
    return_type: Option<String>,
    #[serde(default)]
    is_async: bool,
    #[serde(default)]
    domain: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ParamInput {
    name: String,
    #[serde(rename = "type", default = "default_type")]
    param_type: String,
    #[serde(default)]
    is_optional: bool,
}

fn default_type() -> String {
    "Any".to_string()
}

/// Return the tool definition for evolve_get_body.
pub fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "evolve_get_body".to_string(),
        description: Some("Get function body from the best matching pattern".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Function name to look up"
                },
                "language": {
                    "type": "string",
                    "description": "Programming language"
                },
                "params": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": { "type": "string" },
                            "type": { "type": "string" },
                            "is_optional": { "type": "boolean" }
                        },
                        "required": ["name"]
                    }
                },
                "return_type": { "type": "string" },
                "is_async": { "type": "boolean" },
                "domain": {
                    "type": "string",
                    "description": "Optional domain hint"
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
            "required": ["name", "language"]
        }),
    }
}

/// Execute the evolve_get_body tool.
pub async fn execute(
    args: Value,
    session: &Arc<Mutex<SessionManager>>,
) -> McpResult<ToolCallResult> {
    let params: GetBodyParams =
        serde_json::from_value(args).map_err(|e| McpError::InvalidParams(e.to_string()))?;

    let language = Language::from_name(&params.language);

    let sig_params: Vec<ParamSignature> = params
        .params
        .into_iter()
        .map(|p| ParamSignature {
            name: p.name,
            param_type: p.param_type,
            is_optional: p.is_optional,
        })
        .collect();

    let signature = FunctionSignature {
        name: params.name,
        params: sig_params,
        return_type: params.return_type,
        language,
        is_async: params.is_async,
        visibility: Visibility::Public,
    };

    let mut context = MatchContext::new();
    if let Some(domain) = params.domain {
        context = context.with_domain(&domain);
    }

    let session = session.lock().await;
    let result = session
        .get_body(&signature, &context)
        .map_err(|e| McpError::AgenticEvolve(e.to_string()))?;

    match result {
        Some((body, pattern_id, score)) => Ok(ToolCallResult::json(&json!({
            "found": true,
            "pattern_id": pattern_id,
            "score": score,
            "body": body
        }))),
        None => Ok(ToolCallResult::json(&json!({
            "found": false,
            "message": "No matching pattern found for this function signature"
        }))),
    }
}
