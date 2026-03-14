//! Main request dispatcher — receives JSON-RPC messages, routes to handlers.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;

use serde_json::Value;

use crate::session::SessionManager;
use crate::tools::ToolRegistry;
use crate::types::*;

use super::compact;
use super::negotiation::NegotiatedCapabilities;
use super::validator::validate_request;

/// The main protocol handler that dispatches incoming JSON-RPC messages.
pub struct ProtocolHandler {
    session: Arc<Mutex<SessionManager>>,
    capabilities: Arc<Mutex<NegotiatedCapabilities>>,
    shutdown_requested: Arc<AtomicBool>,
}

impl ProtocolHandler {
    /// Create a new protocol handler with the given session manager.
    pub fn new(session: Arc<Mutex<SessionManager>>) -> Self {
        Self {
            session,
            capabilities: Arc::new(Mutex::new(NegotiatedCapabilities::default())),
            shutdown_requested: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Returns true once a shutdown request has been handled.
    pub fn shutdown_requested(&self) -> bool {
        self.shutdown_requested.load(Ordering::Relaxed)
    }

    /// Handle an incoming JSON-RPC message and optionally return a response.
    pub async fn handle_message(&self, msg: JsonRpcMessage) -> Option<Value> {
        match msg {
            JsonRpcMessage::Request(req) => Some(self.handle_request(req).await),
            JsonRpcMessage::Notification(notif) => {
                self.handle_notification(notif).await;
                None
            }
            _ => {
                // Responses and errors from the client are unexpected
                tracing::warn!("Received unexpected message type from client");
                None
            }
        }
    }

    async fn handle_request(&self, request: JsonRpcRequest) -> Value {
        // Validate JSON-RPC structure
        if let Err(e) = validate_request(&request) {
            return serde_json::to_value(e.to_json_rpc_error(request.id)).unwrap_or_default();
        }

        let id = request.id.clone();
        let result = self.dispatch_request(&request).await;

        match result {
            Ok(value) => serde_json::to_value(JsonRpcResponse::new(id, value)).unwrap_or_default(),
            Err(e) => serde_json::to_value(e.to_json_rpc_error(id)).unwrap_or_default(),
        }
    }

    async fn dispatch_request(&self, request: &JsonRpcRequest) -> McpResult<Value> {
        match request.method.as_str() {
            // Lifecycle
            "initialize" => self.handle_initialize(request.params.clone()).await,
            "shutdown" => self.handle_shutdown().await,

            // Tools
            "tools/list" => self.handle_tools_list().await,
            "tools/call" => self.handle_tools_call(request.params.clone()).await,

            // Resources (empty — this server only exposes tools)
            "resources/list" => {
                let result = ResourceListResult {
                    resources: Vec::new(),
                    next_cursor: None,
                };
                serde_json::to_value(result).map_err(|e| McpError::InternalError(e.to_string()))
            }
            "resources/templates/list" => {
                let result = serde_json::json!({
                    "resourceTemplates": [],
                });
                Ok(result)
            }
            "resources/subscribe" => Ok(Value::Object(serde_json::Map::new())),
            "resources/unsubscribe" => Ok(Value::Object(serde_json::Map::new())),

            // Prompts (empty)
            "prompts/list" => {
                let result = PromptListResult {
                    prompts: Vec::new(),
                    next_cursor: None,
                };
                serde_json::to_value(result).map_err(|e| McpError::InternalError(e.to_string()))
            }

            // Ping
            "ping" => Ok(Value::Object(serde_json::Map::new())),

            _ => Err(McpError::MethodNotFound(request.method.clone())),
        }
    }

    async fn handle_notification(&self, notification: JsonRpcNotification) {
        match notification.method.as_str() {
            "initialized" | "notifications/initialized" => {
                let mut caps = self.capabilities.lock().await;
                if let Err(e) = caps.mark_initialized() {
                    tracing::error!("Failed to mark initialized: {e}");
                }
            }
            "notifications/cancelled" | "$/cancelRequest" => {
                tracing::info!("Received cancellation notification");
            }
            _ => {
                tracing::debug!("Unknown notification: {}", notification.method);
            }
        }
    }

    async fn handle_initialize(&self, params: Option<Value>) -> McpResult<Value> {
        let init_params: InitializeParams = params
            .map(serde_json::from_value)
            .transpose()
            .map_err(|e| McpError::InvalidParams(e.to_string()))?
            .ok_or_else(|| McpError::InvalidParams("Initialize params required".to_string()))?;

        let mut caps = self.capabilities.lock().await;
        let result = caps.negotiate(init_params)?;

        serde_json::to_value(result).map_err(|e| McpError::InternalError(e.to_string()))
    }

    async fn handle_shutdown(&self) -> McpResult<Value> {
        tracing::info!("Shutdown requested");
        self.shutdown_requested.store(true, Ordering::Relaxed);
        Ok(Value::Object(serde_json::Map::new()))
    }

    async fn handle_tools_list(&self) -> McpResult<Value> {
        let tools = if compact::is_compact_mode() {
            compact::compact_tool_definitions()
        } else {
            ToolRegistry::list_tools()
        };
        let result = ToolListResult {
            tools,
            next_cursor: None,
        };
        serde_json::to_value(result).map_err(|e| McpError::InternalError(e.to_string()))
    }

    async fn handle_tools_call(&self, params: Option<Value>) -> McpResult<Value> {
        let call_params: ToolCallParams = params
            .map(serde_json::from_value)
            .transpose()
            .map_err(|e| McpError::InvalidParams(e.to_string()))?
            .ok_or_else(|| McpError::InvalidParams("Tool call params required".to_string()))?;

        // Normalize compact facade calls to underlying tool names.
        let (tool_name, arguments) = if compact::is_compact_facade(&call_params.name) {
            match compact::normalize_compact_call(&call_params.name, &call_params.arguments) {
                Some((real_name, real_args)) => (real_name, real_args),
                None => {
                    return Err(McpError::InvalidParams(
                        "Invalid operation for compact facade".to_string(),
                    ));
                }
            }
        } else {
            (call_params.name, call_params.arguments)
        };

        // Classify errors: protocol errors (ToolNotFound etc.) become JSON-RPC errors;
        // tool execution errors become isError: true.
        let result = match ToolRegistry::call(&tool_name, arguments, &self.session).await {
            Ok(r) => r,
            Err(e) if e.is_protocol_error() => return Err(e),
            Err(e) => ToolCallResult::error(e.to_string()),
        };

        serde_json::to_value(result).map_err(|e| McpError::InternalError(e.to_string()))
    }
}
