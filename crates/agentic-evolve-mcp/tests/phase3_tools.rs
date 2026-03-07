//! Phase 3 MCP: Tool registry tests — listing, naming, descriptions, dispatch.

use std::sync::Arc;
use tokio::sync::Mutex;

use serde_json::json;

use agentic_evolve_mcp::session::SessionManager;
use agentic_evolve_mcp::tools::ToolRegistry;
use agentic_evolve_mcp::types::error::mcp_error_codes;

/// Create a session + keep the tempdir alive by returning both.
fn make_session() -> (tempfile::TempDir, Arc<Mutex<SessionManager>>) {
    let dir = tempfile::tempdir().unwrap();
    let session = SessionManager::new(dir.path().to_str().unwrap()).unwrap();
    (dir, Arc::new(Mutex::new(session)))
}

// ===========================================================================
// Tool listing
// ===========================================================================

#[test]
fn list_tools_returns_14() {
    let tools = ToolRegistry::list_tools();
    assert_eq!(tools.len(), 14, "Expected 14 tools, got {}", tools.len());
}

#[test]
fn tool_names_are_correct() {
    let tools = ToolRegistry::list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
    let expected = vec![
        "evolve_pattern_store",
        "evolve_pattern_get",
        "evolve_pattern_search",
        "evolve_pattern_list",
        "evolve_pattern_delete",
        "evolve_match_signature",
        "evolve_match_context",
        "evolve_crystallize",
        "evolve_get_body",
        "evolve_compose",
        "evolve_coverage",
        "evolve_confidence",
        "evolve_update_usage",
        "evolve_optimize",
    ];
    for name in &expected {
        assert!(names.contains(name), "Missing tool: {name}");
    }
}

#[test]
fn tool_descriptions_start_with_verb() {
    let tools = ToolRegistry::list_tools();
    for tool in &tools {
        if let Some(desc) = &tool.description {
            let first_char = desc.chars().next().unwrap_or(' ');
            assert!(
                first_char.is_uppercase(),
                "Tool {} description should start with uppercase verb: '{}'",
                tool.name,
                desc
            );
        }
    }
}

#[test]
fn tool_descriptions_no_trailing_period() {
    let tools = ToolRegistry::list_tools();
    for tool in &tools {
        if let Some(desc) = &tool.description {
            assert!(
                !desc.ends_with('.'),
                "Tool {} description should not end with period: '{}'",
                tool.name,
                desc
            );
        }
    }
}

#[test]
fn tool_definitions_have_input_schema() {
    let tools = ToolRegistry::list_tools();
    for tool in &tools {
        assert!(
            tool.input_schema.is_object(),
            "Tool {} should have object input_schema",
            tool.name
        );
        assert!(
            tool.input_schema.get("type").is_some(),
            "Tool {} input_schema should have 'type' field",
            tool.name
        );
    }
}

#[test]
fn all_tools_have_descriptions() {
    let tools = ToolRegistry::list_tools();
    for tool in &tools {
        assert!(
            tool.description.is_some(),
            "Tool {} should have a description",
            tool.name
        );
    }
}

// ===========================================================================
// Unknown tool dispatch
// ===========================================================================

#[tokio::test]
async fn call_unknown_tool_returns_tool_not_found() {
    let (_dir, session) = make_session();
    let result = ToolRegistry::call("nonexistent_tool", None, &session).await;
    let err = result.unwrap_err();
    assert_eq!(err.code(), mcp_error_codes::TOOL_NOT_FOUND);
    assert_eq!(err.code(), -32803);
}

// ===========================================================================
// Tool calls with valid params
// ===========================================================================

#[tokio::test]
async fn call_pattern_store_valid() {
    let (_dir, session) = make_session();
    let args = json!({
        "name": "test_pattern",
        "domain": "test",
        "language": "rust",
        "template": "fn test() {}"
    });
    let result = ToolRegistry::call("evolve_pattern_store", Some(args), &session).await;
    assert!(result.is_ok(), "evolve_pattern_store should succeed: {:?}", result.err());
}

#[tokio::test]
async fn call_pattern_list_valid() {
    let (_dir, session) = make_session();
    let result = ToolRegistry::call("evolve_pattern_list", Some(json!({})), &session).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn call_pattern_search_valid() {
    let (_dir, session) = make_session();
    let args = json!({"query": "test"});
    let result = ToolRegistry::call("evolve_pattern_search", Some(args), &session).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn call_pattern_get_not_found() {
    let (_dir, session) = make_session();
    let args = json!({"pattern_id": "nonexistent-id"});
    let result = ToolRegistry::call("evolve_pattern_get", Some(args), &session).await;
    // Should return tool execution error (not protocol error)
    assert!(result.is_ok() || !result.as_ref().unwrap_err().is_protocol_error());
}

#[tokio::test]
async fn call_pattern_delete_not_found() {
    let (_dir, session) = make_session();
    let args = json!({"pattern_id": "nonexistent-id"});
    let result = ToolRegistry::call("evolve_pattern_delete", Some(args), &session).await;
    assert!(result.is_ok() || !result.as_ref().unwrap_err().is_protocol_error());
}

#[tokio::test]
async fn call_match_signature_valid() {
    let (_dir, session) = make_session();
    let args = json!({
        "name": "test_fn",
        "language": "rust"
    });
    let result = ToolRegistry::call("evolve_match_signature", Some(args), &session).await;
    assert!(result.is_ok(), "evolve_match_signature should succeed: {:?}", result.err());
}

#[tokio::test]
async fn call_match_context_valid() {
    let (_dir, session) = make_session();
    let args = json!({
        "name": "handler",
        "language": "rust",
        "domain": "web"
    });
    let result = ToolRegistry::call("evolve_match_context", Some(args), &session).await;
    assert!(result.is_ok(), "evolve_match_context should succeed: {:?}", result.err());
}

#[tokio::test]
async fn call_crystallize_valid() {
    let (_dir, session) = make_session();
    let args = json!({
        "code": "pub fn add(a: i32, b: i32) -> i32 {\n    a + b\n}",
        "language": "rust",
        "domain": "math",
        "test_results": [{"name": "test_add", "passed": true, "duration_ms": 5}],
        "execution_time_ms": 10
    });
    let result = ToolRegistry::call("evolve_crystallize", Some(args), &session).await;
    assert!(result.is_ok(), "crystallize should succeed: {:?}", result.err());
}

#[tokio::test]
async fn call_get_body_valid() {
    let (_dir, session) = make_session();
    let args = json!({
        "name": "test",
        "language": "rust"
    });
    let result = ToolRegistry::call("evolve_get_body", Some(args), &session).await;
    assert!(result.is_ok(), "evolve_get_body should succeed: {:?}", result.err());
}

#[tokio::test]
async fn call_optimize_valid() {
    let (_dir, session) = make_session();
    let result = ToolRegistry::call("evolve_optimize", Some(json!({})), &session).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn call_coverage_valid() {
    let (_dir, session) = make_session();
    let args = json!({
        "signatures": [
            {"name": "handler", "language": "rust"}
        ]
    });
    let result = ToolRegistry::call("evolve_coverage", Some(args), &session).await;
    assert!(result.is_ok(), "evolve_coverage should succeed: {:?}", result.err());
}

// ===========================================================================
// Tool calls with invalid params
// ===========================================================================

#[tokio::test]
async fn call_pattern_store_missing_required() {
    let (_dir, session) = make_session();
    let args = json!({"name": "test"}); // missing domain, language, template
    let result = ToolRegistry::call("evolve_pattern_store", Some(args), &session).await;
    assert!(result.is_err(), "Missing required params should fail");
}

#[tokio::test]
async fn call_pattern_store_invalid_confidence() {
    let (_dir, session) = make_session();
    let args = json!({
        "name": "test",
        "domain": "test",
        "language": "rust",
        "template": "fn test() {}",
        "confidence": 2.0
    });
    let result = ToolRegistry::call("evolve_pattern_store", Some(args), &session).await;
    assert!(result.is_err(), "Confidence > 1.0 should fail");
}

// ===========================================================================
// Store + Get roundtrip
// ===========================================================================

#[tokio::test]
async fn store_and_get_pattern_roundtrip() {
    let (_dir, session) = make_session();
    // Store a pattern
    let store_args = json!({
        "name": "roundtrip_test",
        "domain": "test",
        "language": "rust",
        "template": "fn roundtrip() { 42 }",
        "tags": ["test"]
    });
    let store_result = ToolRegistry::call("evolve_pattern_store", Some(store_args), &session)
        .await
        .unwrap();

    // Extract pattern_id from result
    let content_text = match &store_result.content[0] {
        agentic_evolve_mcp::types::response::ToolContent::Text { text } => text.clone(),
        _ => panic!("Expected text content"),
    };
    let stored: serde_json::Value = serde_json::from_str(&content_text).unwrap();
    let pattern_id = stored["pattern_id"].as_str().unwrap();

    // Get the pattern back
    let get_args = json!({"pattern_id": pattern_id});
    let get_result = ToolRegistry::call("evolve_pattern_get", Some(get_args), &session)
        .await
        .unwrap();
    assert!(get_result.is_error.is_none());
}

// ===========================================================================
// Store + Delete roundtrip
// ===========================================================================

#[tokio::test]
async fn store_and_delete_pattern() {
    let (_dir, session) = make_session();
    let store_args = json!({
        "name": "deletable",
        "domain": "test",
        "language": "rust",
        "template": "fn delete_me() {}"
    });
    let store_result = ToolRegistry::call("evolve_pattern_store", Some(store_args), &session)
        .await
        .unwrap();

    let content_text = match &store_result.content[0] {
        agentic_evolve_mcp::types::response::ToolContent::Text { text } => text.clone(),
        _ => panic!("Expected text content"),
    };
    let stored: serde_json::Value = serde_json::from_str(&content_text).unwrap();
    let pattern_id = stored["pattern_id"].as_str().unwrap();

    let delete_args = json!({"pattern_id": pattern_id});
    let delete_result = ToolRegistry::call("evolve_pattern_delete", Some(delete_args), &session)
        .await
        .unwrap();
    assert!(delete_result.is_error.is_none());
}
