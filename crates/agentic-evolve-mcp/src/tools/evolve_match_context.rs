//! Tool: evolve_match_context — Match patterns with surrounding context.

use std::sync::Arc;
use tokio::sync::Mutex;

use serde::Deserialize;
use serde_json::{json, Value};

use agentic_evolve_core::types::match_result::MatchContext;
use agentic_evolve_core::types::pattern::{FunctionSignature, Language, ParamSignature, Visibility};

use crate::session::SessionManager;
use crate::types::{McpError, McpResult, ToolCallResult, ToolDefinition};

#[derive(Debug, Deserialize)]
struct ContextMatchParams {
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
    #[serde(default)]
    surrounding_code: Option<String>,
    #[serde(default)]
    imports: Vec<String>,
    #[serde(default)]
    project_type: Option<String>,
    #[serde(default = "default_limit")]
    limit: usize,
}

#[derive(Debug, Deserialize)]
struct ParamInput {
    name: String,
    #[serde(rename = "type", default = "default_type")]
    param_type: String,
    #[serde(default)]
    is_optional: bool,
}

fn default_limit() -> usize {
    5
}

fn default_type() -> String {
    "Any".to_string()
}

/// Return the tool definition for evolve_match_context.
pub fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "evolve_match_context".to_string(),
        description: Some("Match patterns using surrounding code context for better results".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Function name to match"
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
                    "description": "Domain or category for context matching"
                },
                "surrounding_code": {
                    "type": "string",
                    "description": "Code surrounding the function for context analysis"
                },
                "imports": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Import statements in the file"
                },
                "project_type": {
                    "type": "string",
                    "description": "Project type (e.g. 'web-api', 'cli', 'library')"
                },
                "limit": {
                    "type": "integer",
                    "default": 5,
                    "description": "Maximum number of matches"
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

/// Execute the evolve_match_context tool.
pub async fn execute(
    args: Value,
    session: &Arc<Mutex<SessionManager>>,
) -> McpResult<ToolCallResult> {
    let params: ContextMatchParams =
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

    let mut context = MatchContext {
        domain: params.domain,
        surrounding_code: params.surrounding_code,
        imports: params.imports,
        project_type: params.project_type,
        max_results: params.limit,
    };
    let _ = &mut context; // suppress unused warning

    let session = session.lock().await;
    let results = session
        .match_context(&signature, &context, params.limit)
        .map_err(|e| McpError::AgenticEvolve(e.to_string()))?;

    let matches: Vec<Value> = results
        .iter()
        .map(|r| {
            json!({
                "pattern_id": r.pattern_id.as_str(),
                "name": r.pattern.name,
                "score": {
                    "combined": r.score.combined,
                    "signature": r.score.signature_score,
                    "context": r.score.context_score,
                    "semantic": r.score.semantic_score
                },
                "template": r.pattern.template,
                "domain": r.pattern.domain,
                "confidence": r.pattern.confidence,
                "suggested_bindings": r.suggested_bindings
            })
        })
        .collect();

    Ok(ToolCallResult::json(&json!({
        "match_count": matches.len(),
        "matches": matches
    })))
}
