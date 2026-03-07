//! Sister integration bridge traits for AgenticEvolve.

/// Bridge to agentic-memory for linking patterns to memory nodes.
pub trait MemoryBridge: Send + Sync {
    fn link_pattern_to_memory(&self, pattern_id: &str, node_id: u64) -> Result<(), String> {
        let _ = (pattern_id, node_id);
        Err("Memory bridge not connected".to_string())
    }

    fn query_related_memories(&self, pattern_name: &str, max_results: usize) -> Vec<String> {
        let _ = (pattern_name, max_results);
        Vec::new()
    }
}

/// Bridge to agentic-codebase for code-aware pattern operations.
pub trait CodebaseBridge: Send + Sync {
    fn find_similar_code(&self, signature: &str, max_results: usize) -> Vec<String> {
        let _ = (signature, max_results);
        Vec::new()
    }

    fn validate_pattern_against_codebase(&self, pattern_template: &str) -> Result<bool, String> {
        let _ = pattern_template;
        Ok(true)
    }
}

/// Bridge to agentic-identity for signing patterns.
pub trait IdentityBridge: Send + Sync {
    fn sign_pattern(&self, pattern_id: &str, content_hash: &str) -> Result<String, String> {
        let _ = (pattern_id, content_hash);
        Err("Identity bridge not connected".to_string())
    }

    fn verify_pattern_signature(&self, pattern_id: &str, signature: &str) -> bool {
        let _ = (pattern_id, signature);
        true
    }
}

/// Bridge to agentic-contract for pattern usage policies.
pub trait ContractBridge: Send + Sync {
    fn check_pattern_policy(&self, pattern_id: &str, operation: &str) -> Result<bool, String> {
        let _ = (pattern_id, operation);
        Ok(true)
    }
}

/// Bridge to agentic-cognition for reasoning about patterns.
pub trait CognitionBridge: Send + Sync {
    fn reason_about_match(&self, pattern_id: &str, context: &str) -> Option<f64> {
        let _ = (pattern_id, context);
        None
    }
}

/// No-op implementation of all bridges for standalone use.
#[derive(Debug, Clone, Default)]
pub struct NoOpBridges;

impl MemoryBridge for NoOpBridges {}
impl CodebaseBridge for NoOpBridges {}
impl IdentityBridge for NoOpBridges {}
impl ContractBridge for NoOpBridges {}
impl CognitionBridge for NoOpBridges {}

/// Configuration for which bridges are active.
#[derive(Debug, Clone, Default)]
pub struct BridgeConfig {
    pub memory_enabled: bool,
    pub codebase_enabled: bool,
    pub identity_enabled: bool,
    pub contract_enabled: bool,
    pub cognition_enabled: bool,
}

/// Hydra adapter trait for orchestrator integration.
pub trait HydraAdapter: Send + Sync {
    fn adapter_id(&self) -> &str;
    fn capabilities(&self) -> Vec<String>;
    fn handle_request(&self, method: &str, params: &str) -> Result<String, String>;
}
