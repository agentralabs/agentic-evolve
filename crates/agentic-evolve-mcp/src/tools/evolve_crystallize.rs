//! Tool: evolve_crystallize — Crystallize successful code into a pattern.

use std::sync::Arc;
use tokio::sync::Mutex;

use serde::Deserialize;
use serde_json::{json, Value};

use agentic_evolve_core::types::pattern::Language;
use agentic_evolve_core::types::skill::{SuccessfulExecution, TestResult};

use crate::session::SessionManager;
use crate::types::{McpError, McpResult, ToolCallResult, ToolDefinition};

#[derive(Debug, Deserialize)]
struct CrystallizeParams {
    code: String,
    language: String,
    domain: String,
    #[serde(default)]
    test_results: Vec<TestInput>,
    #[serde(default)]
    execution_time_ms: u64,
}

#[derive(Debug, Deserialize)]
struct TestInput {
    name: String,
    #[serde(default = "default_passed")]
    passed: bool,
    #[serde(default)]
    duration_ms: u64,
}

fn default_passed() -> bool {
    true
}

/// Return the tool definition for evolve_crystallize.
pub fn definition() -> ToolDefinition {
    ToolDefinition {
        name: "evolve_crystallize".to_string(),
        description: Some("Crystallize successful code into reusable patterns".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "code": {
                    "type": "string",
                    "description": "The successful code to crystallize into patterns"
                },
                "language": {
                    "type": "string",
                    "description": "Programming language of the code"
                },
                "domain": {
                    "type": "string",
                    "description": "Domain or category for the patterns"
                },
                "test_results": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": { "type": "string" },
                            "passed": { "type": "boolean" },
                            "duration_ms": { "type": "integer" }
                        },
                        "required": ["name"]
                    },
                    "description": "Test results that verified the code"
                },
                "execution_time_ms": {
                    "type": "integer",
                    "description": "Total execution time in milliseconds"
                }
            },
            "required": ["code", "language", "domain"]
        }),
    }
}

/// Execute the evolve_crystallize tool.
pub async fn execute(
    args: Value,
    session: &Arc<Mutex<SessionManager>>,
) -> McpResult<ToolCallResult> {
    let params: CrystallizeParams =
        serde_json::from_value(args).map_err(|e| McpError::InvalidParams(e.to_string()))?;

    let test_results: Vec<TestResult> = params
        .test_results
        .into_iter()
        .map(|t| TestResult {
            name: t.name,
            passed: t.passed,
            duration_ms: t.duration_ms,
        })
        .collect();

    let execution = SuccessfulExecution {
        code: params.code,
        language: Language::from_name(&params.language),
        domain: params.domain,
        test_results,
        execution_time_ms: params.execution_time_ms,
    };

    let mut session = session.lock().await;
    let patterns = session
        .crystallize(&execution)
        .map_err(|e| McpError::AgenticEvolve(e.to_string()))?;

    let pattern_summaries: Vec<Value> = patterns
        .iter()
        .map(|p| {
            json!({
                "pattern_id": p.id.as_str(),
                "name": p.name,
                "domain": p.domain,
                "confidence": p.confidence,
                "template_lines": p.template.lines().count(),
                "variables": p.variables.len()
            })
        })
        .collect();

    Ok(ToolCallResult::json(&json!({
        "patterns_created": pattern_summaries.len(),
        "patterns": pattern_summaries
    })))
}
