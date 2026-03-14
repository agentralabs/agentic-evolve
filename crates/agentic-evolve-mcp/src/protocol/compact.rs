//! Compact facade layer — groups 14 tools into 3 compact facades.
//!
//! Env var: `AEVOLVE_MCP_TOOL_SURFACE` (fallback `MCP_TOOL_SURFACE`).
//! Set to "compact" to enable facade mode.

use serde_json::Value;

use crate::types::ToolDefinition;

/// A facade grouping that maps one compact tool to multiple underlying operations.
struct FacadeGroup {
    name: &'static str,
    description: &'static str,
    operations: &'static [&'static str],
}

const FACADES: &[FacadeGroup] = &[
    FacadeGroup {
        name: "evolve_patterns",
        description: "Store, retrieve, delete, search, and list evolution patterns",
        operations: &[
            "pattern_store",
            "pattern_get",
            "pattern_delete",
            "pattern_search",
            "pattern_list",
        ],
    },
    FacadeGroup {
        name: "evolve_matching",
        description: "Match patterns by signature or context, crystallize, compose, and retrieve bodies",
        operations: &[
            "match_signature",
            "match_context",
            "crystallize",
            "compose",
            "get_body",
        ],
    },
    FacadeGroup {
        name: "evolve_analytics",
        description: "Check coverage and confidence, update usage stats, and optimize the pattern library",
        operations: &[
            "coverage",
            "confidence",
            "update_usage",
            "optimize",
        ],
    },
];

/// Check whether compact tool mode is enabled via environment variable.
pub fn is_compact_mode() -> bool {
    let val = std::env::var("AEVOLVE_MCP_TOOL_SURFACE")
        .or_else(|_| std::env::var("MCP_TOOL_SURFACE"))
        .unwrap_or_default();
    val.eq_ignore_ascii_case("compact")
}

/// Build the compact facade tool definitions for tools/list.
pub fn compact_tool_definitions() -> Vec<ToolDefinition> {
    FACADES
        .iter()
        .map(|f| {
            let ops_enum: Vec<Value> = f
                .operations
                .iter()
                .map(|o| Value::String(o.to_string()))
                .collect();

            ToolDefinition {
                name: f.name.to_string(),
                description: Some(f.description.to_string()),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "operation": {
                            "type": "string",
                            "enum": ops_enum,
                            "description": "Operation to perform"
                        },
                        "params": {
                            "type": "object",
                            "description": "Parameters for the operation"
                        }
                    },
                    "required": ["operation"]
                }),
            }
        })
        .collect()
}

/// Normalize a compact facade call into the underlying tool name and arguments.
///
/// Mapping: facade "evolve_patterns" + operation "pattern_store"
///          -> tool "evolve_pattern_store"
pub fn normalize_compact_call(
    facade_name: &str,
    arguments: &Option<Value>,
) -> Option<(String, Option<Value>)> {
    let facade = FACADES.iter().find(|f| f.name == facade_name)?;

    let args = arguments.as_ref().unwrap_or(&Value::Null);
    let operation = args.get("operation").and_then(|v| v.as_str())?;

    if !facade.operations.contains(&operation) {
        return None;
    }

    let real_name = format!("evolve_{}", operation);
    let params = args.get("params").cloned();

    Some((real_name, params))
}

/// Check if a tool name is a compact facade name.
pub fn is_compact_facade(name: &str) -> bool {
    FACADES.iter().any(|f| f.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_definitions_count() {
        let defs = compact_tool_definitions();
        assert_eq!(defs.len(), 3);
        assert_eq!(defs[0].name, "evolve_patterns");
        assert_eq!(defs[1].name, "evolve_matching");
        assert_eq!(defs[2].name, "evolve_analytics");
    }

    #[test]
    fn normalize_patterns_facade() {
        let args = Some(serde_json::json!({
            "operation": "pattern_store",
            "params": { "name": "test", "domain": "web" }
        }));
        let (name, params) = normalize_compact_call("evolve_patterns", &args).unwrap();
        assert_eq!(name, "evolve_pattern_store");
        assert_eq!(
            params.unwrap().get("name").unwrap().as_str().unwrap(),
            "test"
        );
    }

    #[test]
    fn normalize_matching_facade() {
        let args = Some(serde_json::json!({
            "operation": "crystallize",
            "params": { "pattern_id": "p1" }
        }));
        let (name, _) = normalize_compact_call("evolve_matching", &args).unwrap();
        assert_eq!(name, "evolve_crystallize");
    }

    #[test]
    fn normalize_analytics_facade() {
        let args = Some(serde_json::json!({
            "operation": "coverage",
            "params": {}
        }));
        let (name, _) = normalize_compact_call("evolve_analytics", &args).unwrap();
        assert_eq!(name, "evolve_coverage");
    }

    #[test]
    fn normalize_unknown_facade_returns_none() {
        let args = Some(serde_json::json!({ "operation": "pattern_store" }));
        assert!(normalize_compact_call("evolve_unknown", &args).is_none());
    }

    #[test]
    fn normalize_invalid_operation_returns_none() {
        let args = Some(serde_json::json!({ "operation": "nonexistent" }));
        assert!(normalize_compact_call("evolve_patterns", &args).is_none());
    }

    #[test]
    fn is_compact_facade_checks() {
        assert!(is_compact_facade("evolve_patterns"));
        assert!(is_compact_facade("evolve_matching"));
        assert!(is_compact_facade("evolve_analytics"));
        assert!(!is_compact_facade("evolve_pattern_store"));
    }

    #[test]
    fn compact_mode_off_by_default() {
        // Without env var set, should be false
        // (This test is valid as long as the env var isn't set in CI)
        assert!(!is_compact_mode());
    }
}
