//! Phase 1 MCP: Type tests — McpError, ToolDefinition, ToolCallResult, JSON-RPC messages.

use agentic_evolve_mcp::types::capabilities::InitializeResult;
use agentic_evolve_mcp::types::error::{error_codes, mcp_error_codes, McpError};
use agentic_evolve_mcp::types::message::{
    JsonRpcError, JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, RequestId, JSONRPC_VERSION,
};
use agentic_evolve_mcp::types::response::{ToolCallResult, ToolDefinition};
use serde_json::json;

// ===========================================================================
// McpError
// ===========================================================================

#[test]
fn mcp_error_parse_error_code() {
    let e = McpError::ParseError("bad json".to_string());
    assert_eq!(e.code(), error_codes::PARSE_ERROR);
}

#[test]
fn mcp_error_invalid_request_code() {
    let e = McpError::InvalidRequest("bad".to_string());
    assert_eq!(e.code(), error_codes::INVALID_REQUEST);
}

#[test]
fn mcp_error_method_not_found_code() {
    let e = McpError::MethodNotFound("unknown".to_string());
    assert_eq!(e.code(), error_codes::METHOD_NOT_FOUND);
}

#[test]
fn mcp_error_invalid_params_code() {
    let e = McpError::InvalidParams("missing field".to_string());
    assert_eq!(e.code(), error_codes::INVALID_PARAMS);
}

#[test]
fn mcp_error_internal_error_code() {
    let e = McpError::InternalError("oops".to_string());
    assert_eq!(e.code(), error_codes::INTERNAL_ERROR);
}

#[test]
fn mcp_error_tool_not_found_code() {
    let e = McpError::ToolNotFound("no_such_tool".to_string());
    assert_eq!(e.code(), mcp_error_codes::TOOL_NOT_FOUND);
    assert_eq!(e.code(), -32803);
}

#[test]
fn mcp_error_pattern_not_found_code() {
    let e = McpError::PatternNotFound("p1".to_string());
    assert_eq!(e.code(), mcp_error_codes::PATTERN_NOT_FOUND);
}

#[test]
fn mcp_error_is_protocol_error_true() {
    assert!(McpError::ParseError("x".to_string()).is_protocol_error());
    assert!(McpError::InvalidRequest("x".to_string()).is_protocol_error());
    assert!(McpError::MethodNotFound("x".to_string()).is_protocol_error());
    assert!(McpError::ToolNotFound("x".to_string()).is_protocol_error());
    assert!(McpError::RequestCancelled.is_protocol_error());
}

#[test]
fn mcp_error_is_protocol_error_false() {
    assert!(!McpError::InternalError("x".to_string()).is_protocol_error());
    assert!(!McpError::InvalidParams("x".to_string()).is_protocol_error());
    assert!(!McpError::PatternNotFound("x".to_string()).is_protocol_error());
    assert!(!McpError::AgenticEvolve("x".to_string()).is_protocol_error());
}

#[test]
fn mcp_error_to_json_rpc_error() {
    let e = McpError::ToolNotFound("unknown_tool".to_string());
    let rpc_err = e.to_json_rpc_error(RequestId::Number(1));
    assert_eq!(rpc_err.error.code, -32803);
    assert_eq!(rpc_err.jsonrpc, JSONRPC_VERSION);
    assert_eq!(rpc_err.id, RequestId::Number(1));
}

// ===========================================================================
// ToolCallResult
// ===========================================================================

#[test]
fn tool_call_result_text() {
    let result = ToolCallResult::text("hello".to_string());
    assert!(result.is_error.is_none());
    assert_eq!(result.content.len(), 1);
}

#[test]
fn tool_call_result_json() {
    let result = ToolCallResult::json(&json!({"key": "value"}));
    assert!(result.is_error.is_none());
    assert_eq!(result.content.len(), 1);
}

#[test]
fn tool_call_result_error() {
    let result = ToolCallResult::error("something went wrong".to_string());
    assert_eq!(result.is_error, Some(true));
    assert_eq!(result.content.len(), 1);
}

// ===========================================================================
// JSON-RPC message types
// ===========================================================================

#[test]
fn json_rpc_request_serialization() {
    let req = JsonRpcRequest {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id: RequestId::Number(1),
        method: "tools/list".to_string(),
        params: None,
    };
    let json = serde_json::to_value(&req).unwrap();
    assert_eq!(json["jsonrpc"], "2.0");
    assert_eq!(json["method"], "tools/list");
}

#[test]
fn json_rpc_response_serialization() {
    let resp = JsonRpcResponse::new(RequestId::Number(1), json!({"tools": []}));
    let json = serde_json::to_value(&resp).unwrap();
    assert_eq!(json["jsonrpc"], "2.0");
    assert_eq!(json["id"], 1);
}

#[test]
fn json_rpc_error_serialization() {
    let err = JsonRpcError::new(
        RequestId::String("abc".to_string()),
        -32600,
        "Invalid".to_string(),
    );
    let json = serde_json::to_value(&err).unwrap();
    assert_eq!(json["error"]["code"], -32600);
}

#[test]
fn json_rpc_notification_creation() {
    let notif = JsonRpcNotification::new("initialized".to_string(), None);
    assert_eq!(notif.jsonrpc, JSONRPC_VERSION);
    assert_eq!(notif.method, "initialized");
    assert!(notif.params.is_none());
}

// ===========================================================================
// RequestId
// ===========================================================================

#[test]
fn request_id_string_variant() {
    let id = RequestId::String("abc-123".to_string());
    assert_eq!(format!("{id}"), "abc-123");
}

#[test]
fn request_id_number_variant() {
    let id = RequestId::Number(42);
    assert_eq!(format!("{id}"), "42");
}

#[test]
fn request_id_null_variant() {
    let id = RequestId::Null;
    assert_eq!(format!("{id}"), "null");
}

// ===========================================================================
// InitializeResult
// ===========================================================================

#[test]
fn initialize_result_default() {
    let result = InitializeResult::default_result();
    assert_eq!(result.protocol_version, "2024-11-05");
    assert!(result.capabilities.tools.is_some());
    assert!(result.server_info.name.contains("evolve"));
}

// ===========================================================================
// ToolDefinition
// ===========================================================================

#[test]
fn tool_definition_structure() {
    let def = ToolDefinition {
        name: "test_tool".to_string(),
        description: Some("Do something useful".to_string()),
        input_schema: json!({"type": "object"}),
    };
    assert_eq!(def.name, "test_tool");
    assert!(def.description.is_some());
}
