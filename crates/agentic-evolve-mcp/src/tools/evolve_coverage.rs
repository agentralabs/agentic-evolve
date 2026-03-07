//! Tool: evolve_coverage — Get pattern coverage for a set of signatures.

use std::sync::Arc;
use tokio::sync::Mutex;

use serde::Deserialize;
use serde_json::{json, Value};

use agentic_evolve_core::types::pattern::{
    FunctionSignature, Language, ParamSignature, Visibility,
};

use crate::session::SessionManager;
use crate::types::{McpError, McpResult, ToolCallResult, ToolDefinition};

#[derive(Debug, Deserialize)]
struct CoverageParams {
    signatures: Vec<SignatureInput>,
    #[serde(default = "default_threshold")]
    threshold: f64,
}

#[derive(Debug, Deserialize)]
struct SignatureInput {
    name: String,
    language: String,
    #[serde(default)]
    params: Vec<ParamInput>,
    #[serde(default)]
    return_type: Option<String>,
    #[serde(default)]
    is_async: bool,
}

#[derive(Debug, Deserialize)]
struct ParamInput {
    name: String,
    #[serde(rename = "type", default = "default_type")]
    param_type: String,
    #[serde(default)]
    is_optional: bool,
}

fn default_threshold() -> f64 {
    0.5
}

fn default_type() -> String {
    "Any".to_string()
}

/// Return the tool definition for evolve_coverage.
pub fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "evolve_coverage".to_string(),
        description: Some("Get pattern coverage for a set of function signatures".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "signatures": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": { "type": "string" },
                            "language": { "type": "string" },
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
                            "is_async": { "type": "boolean" }
                        },
                        "required": ["name", "language"]
                    },
                    "description": "Function signatures to check coverage for"
                },
                "threshold": {
                    "type": "number",
                    "default": 0.5,
                    "description": "Minimum match score to consider covered (0.0 to 1.0)"
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
            "required": ["signatures"]
        }),
    }
}

/// Execute the evolve_coverage tool.
pub async fn execute(
    args: Value,
    session: &Arc<Mutex<SessionManager>>,
) -> McpResult<ToolCallResult> {
    let params: CoverageParams =
        serde_json::from_value(args).map_err(|e| McpError::InvalidParams(e.to_string()))?;

    let signatures: Vec<FunctionSignature> = params
        .signatures
        .into_iter()
        .map(|s| {
            let language = Language::from_name(&s.language);
            let sig_params: Vec<ParamSignature> = s
                .params
                .into_iter()
                .map(|p| ParamSignature {
                    name: p.name,
                    param_type: p.param_type,
                    is_optional: p.is_optional,
                })
                .collect();
            FunctionSignature {
                name: s.name,
                params: sig_params,
                return_type: s.return_type,
                language,
                is_async: s.is_async,
                visibility: Visibility::Public,
            }
        })
        .collect();

    let session = session.lock().await;
    let report = session.coverage(&signatures, params.threshold);

    Ok(ToolCallResult::json(&report))
}
