//! Tool: evolve_pattern_store — Store a new pattern in the library.

use std::sync::Arc;
use tokio::sync::Mutex;

use serde::Deserialize;
use serde_json::{json, Value};

use agentic_evolve_core::types::pattern::{
    FunctionSignature, Language, ParamSignature, PatternVariable, Visibility,
};

use crate::session::SessionManager;
use crate::types::{McpError, McpResult, ToolCallResult, ToolDefinition};

#[derive(Debug, Deserialize)]
struct StoreParams {
    name: String,
    domain: String,
    language: String,
    template: String,
    #[serde(default)]
    function_name: Option<String>,
    #[serde(default)]
    params: Vec<ParamInput>,
    #[serde(default)]
    return_type: Option<String>,
    #[serde(default)]
    is_async: bool,
    #[serde(default)]
    variables: Vec<VariableInput>,
    #[serde(default = "default_confidence")]
    confidence: f64,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ParamInput {
    name: String,
    #[serde(rename = "type", default = "default_type")]
    param_type: String,
    #[serde(default)]
    is_optional: bool,
}

#[derive(Debug, Deserialize)]
struct VariableInput {
    name: String,
    #[serde(rename = "type", default = "default_type")]
    var_type: String,
    #[serde(default)]
    pattern: Option<String>,
    #[serde(default)]
    default: Option<String>,
}

fn default_confidence() -> f64 {
    0.8
}

fn default_type() -> String {
    "Any".to_string()
}

/// Return the tool definition for evolve_pattern_store.
pub fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "evolve_pattern_store".to_string(),
        description: Some("Store a new pattern in the library".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Human-readable pattern name"
                },
                "domain": {
                    "type": "string",
                    "description": "Domain or category (e.g. 'web', 'cli', 'data')"
                },
                "language": {
                    "type": "string",
                    "description": "Programming language (rust, python, typescript, etc.)"
                },
                "template": {
                    "type": "string",
                    "description": "The code template with {{variable}} placeholders"
                },
                "function_name": {
                    "type": "string",
                    "description": "Function name for the pattern signature"
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
                    },
                    "description": "Function parameters"
                },
                "return_type": {
                    "type": "string",
                    "description": "Return type of the function"
                },
                "is_async": {
                    "type": "boolean",
                    "description": "Whether the function is async"
                },
                "variables": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": { "type": "string" },
                            "type": { "type": "string" },
                            "pattern": { "type": "string" },
                            "default": { "type": "string" }
                        },
                        "required": ["name"]
                    },
                    "description": "Template variable definitions"
                },
                "confidence": {
                    "type": "number",
                    "minimum": 0.0,
                    "maximum": 1.0,
                    "default": 0.8,
                    "description": "Initial confidence level (0.0 to 1.0)"
                },
                "tags": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Tags for categorization"
                }
            },
            "required": ["name", "domain", "language", "template"]
        }),
    }
}

/// Execute the evolve_pattern_store tool.
pub async fn execute(
    args: Value,
    session: &Arc<Mutex<SessionManager>>,
) -> McpResult<ToolCallResult> {
    let params: StoreParams =
        serde_json::from_value(args).map_err(|e| McpError::InvalidParams(e.to_string()))?;

    if !(0.0..=1.0).contains(&params.confidence) {
        return Err(McpError::InvalidParams(format!(
            "confidence must be between 0.0 and 1.0, got {}",
            params.confidence
        )));
    }

    let language = Language::from_name(&params.language);
    let fn_name = params.function_name.unwrap_or_else(|| params.name.clone());

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
        name: fn_name,
        params: sig_params,
        return_type: params.return_type,
        language: language.clone(),
        is_async: params.is_async,
        visibility: Visibility::Public,
    };

    let variables: Vec<PatternVariable> = params
        .variables
        .into_iter()
        .map(|v| PatternVariable {
            name: v.name,
            var_type: v.var_type,
            pattern: v.pattern,
            default: v.default,
        })
        .collect();

    let mut session = session.lock().await;
    let pattern = session
        .store_pattern(
            &params.name,
            &params.domain,
            language,
            signature,
            &params.template,
            variables,
            params.confidence,
            params.tags,
        )
        .map_err(|e| McpError::AgenticEvolve(e.to_string()))?;

    Ok(ToolCallResult::json(&json!({
        "pattern_id": pattern.id.as_str(),
        "name": pattern.name,
        "domain": pattern.domain,
        "language": pattern.language.as_str(),
        "version": pattern.version,
        "confidence": pattern.confidence
    })))
}
