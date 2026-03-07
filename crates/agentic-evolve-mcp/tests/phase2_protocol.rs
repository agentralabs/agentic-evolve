//! Phase 2 MCP: Protocol tests — message parsing, validation, negotiation.

use agentic_evolve_mcp::protocol::negotiation::NegotiatedCapabilities;
use agentic_evolve_mcp::protocol::validator::validate_request;
use agentic_evolve_mcp::types::capabilities::{
    ClientCapabilities, Implementation, InitializeParams, MCP_VERSION,
};
use agentic_evolve_mcp::types::message::{
    JsonRpcMessage, JsonRpcRequest, RequestId, JSONRPC_VERSION,
};

// ===========================================================================
// Message parsing
// ===========================================================================

#[test]
fn parse_valid_json_rpc_request() {
    let json = r#"{
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list"
    }"#;
    let msg: JsonRpcMessage = serde_json::from_str(json).unwrap();
    match msg {
        JsonRpcMessage::Request(req) => {
            assert_eq!(req.method, "tools/list");
            assert_eq!(req.id, RequestId::Number(1));
        }
        _ => panic!("Expected Request variant"),
    }
}

#[test]
fn parse_json_rpc_request_with_params() {
    let json = r#"{
        "jsonrpc": "2.0",
        "id": "abc",
        "method": "tools/call",
        "params": {"name": "evolve_pattern_list"}
    }"#;
    let msg: JsonRpcMessage = serde_json::from_str(json).unwrap();
    match msg {
        JsonRpcMessage::Request(req) => {
            assert_eq!(req.method, "tools/call");
            assert!(req.params.is_some());
        }
        _ => panic!("Expected Request variant"),
    }
}

#[test]
fn parse_invalid_json() {
    let result: Result<JsonRpcMessage, _> = serde_json::from_str("not json at all");
    assert!(result.is_err());
}

#[test]
fn parse_empty_string() {
    let result: Result<JsonRpcMessage, _> = serde_json::from_str("");
    assert!(result.is_err());
}

#[test]
fn parse_notification() {
    let json = r#"{
        "jsonrpc": "2.0",
        "method": "initialized"
    }"#;
    let msg: JsonRpcMessage = serde_json::from_str(json).unwrap();
    match msg {
        JsonRpcMessage::Notification(n) => {
            assert_eq!(n.method, "initialized");
        }
        _ => panic!("Expected Notification variant"),
    }
}

// ===========================================================================
// Request validation
// ===========================================================================

#[test]
fn validate_valid_request() {
    let req = JsonRpcRequest {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id: RequestId::Number(1),
        method: "tools/list".to_string(),
        params: None,
    };
    assert!(validate_request(&req).is_ok());
}

#[test]
fn validate_wrong_jsonrpc_version() {
    let req = JsonRpcRequest {
        jsonrpc: "1.0".to_string(),
        id: RequestId::Number(1),
        method: "tools/list".to_string(),
        params: None,
    };
    let result = validate_request(&req);
    assert!(result.is_err());
}

#[test]
fn validate_empty_method() {
    let req = JsonRpcRequest {
        jsonrpc: JSONRPC_VERSION.to_string(),
        id: RequestId::Number(1),
        method: "".to_string(),
        params: None,
    };
    let result = validate_request(&req);
    assert!(result.is_err());
}

// ===========================================================================
// Capability negotiation
// ===========================================================================

#[test]
fn negotiated_capabilities_default() {
    let caps = NegotiatedCapabilities::default();
    assert!(!caps.initialized);
}

#[test]
fn negotiated_capabilities_negotiate() {
    let mut caps = NegotiatedCapabilities::default();
    let params = InitializeParams {
        protocol_version: MCP_VERSION.to_string(),
        capabilities: ClientCapabilities::default(),
        client_info: Implementation {
            name: "test-client".to_string(),
            version: "1.0".to_string(),
        },
    };
    let result = caps.negotiate(params).unwrap();
    assert_eq!(result.protocol_version, MCP_VERSION);
    assert!(result.capabilities.tools.is_some());
}

#[test]
fn negotiated_capabilities_mark_initialized() {
    let mut caps = NegotiatedCapabilities::default();
    assert!(!caps.initialized);
    caps.mark_initialized().unwrap();
    assert!(caps.initialized);
}

#[test]
fn negotiated_capabilities_version_mismatch_still_works() {
    let mut caps = NegotiatedCapabilities::default();
    let params = InitializeParams {
        protocol_version: "2023-01-01".to_string(), // different version
        capabilities: ClientCapabilities::default(),
        client_info: Implementation {
            name: "old-client".to_string(),
            version: "0.1".to_string(),
        },
    };
    // Should still succeed (with server version)
    let result = caps.negotiate(params).unwrap();
    assert_eq!(result.protocol_version, MCP_VERSION);
}

#[test]
fn negotiated_capabilities_ensure_initialized_fails_before() {
    let caps = NegotiatedCapabilities::default();
    let result = caps.ensure_initialized();
    assert!(result.is_err());
}

#[test]
fn negotiated_capabilities_ensure_initialized_succeeds_after() {
    let mut caps = NegotiatedCapabilities::default();
    caps.mark_initialized().unwrap();
    assert!(caps.ensure_initialized().is_ok());
}
