//! Phase 7: Bridge tests — NoOp bridges, BridgeConfig, trait object safety.

use agentic_evolve_core::bridges::{
    BridgeConfig, CodebaseBridge, CognitionBridge, ContractBridge, IdentityBridge,
    MemoryBridge, NoOpBridges,
};

// ===========================================================================
// NoOpBridges implements all traits
// ===========================================================================

#[test]
fn noop_implements_memory_bridge() {
    let bridge = NoOpBridges;
    let result = bridge.link_pattern_to_memory("pat-1", 42);
    assert!(result.is_err(), "NoOp memory bridge should return error");
}

#[test]
fn noop_memory_bridge_query_returns_empty() {
    let bridge = NoOpBridges;
    let results = bridge.query_related_memories("test", 10);
    assert!(results.is_empty());
}

#[test]
fn noop_implements_codebase_bridge() {
    let bridge = NoOpBridges;
    let results = bridge.find_similar_code("fn test()", 10);
    assert!(results.is_empty());
}

#[test]
fn noop_codebase_bridge_validate_returns_ok() {
    let bridge = NoOpBridges;
    let result = bridge.validate_pattern_against_codebase("fn test() {}");
    assert!(result.unwrap());
}

#[test]
fn noop_implements_identity_bridge() {
    let bridge = NoOpBridges;
    let result = bridge.sign_pattern("pat-1", "hash-abc");
    assert!(result.is_err(), "NoOp identity bridge should return error");
}

#[test]
fn noop_identity_bridge_verify_returns_true() {
    let bridge = NoOpBridges;
    assert!(bridge.verify_pattern_signature("pat-1", "sig-abc"));
}

#[test]
fn noop_implements_contract_bridge() {
    let bridge = NoOpBridges;
    let result = bridge.check_pattern_policy("pat-1", "read");
    assert!(result.unwrap());
}

#[test]
fn noop_implements_cognition_bridge() {
    let bridge = NoOpBridges;
    let result = bridge.reason_about_match("pat-1", "some context");
    assert!(result.is_none());
}

// ===========================================================================
// BridgeConfig
// ===========================================================================

#[test]
fn bridge_config_default_all_false() {
    let config = BridgeConfig::default();
    assert!(!config.memory_enabled);
    assert!(!config.codebase_enabled);
    assert!(!config.identity_enabled);
    assert!(!config.contract_enabled);
    assert!(!config.cognition_enabled);
}

#[test]
fn bridge_config_set_fields() {
    let config = BridgeConfig {
        memory_enabled: true,
        codebase_enabled: true,
        identity_enabled: false,
        contract_enabled: false,
        cognition_enabled: true,
    };
    assert!(config.memory_enabled);
    assert!(config.codebase_enabled);
    assert!(!config.identity_enabled);
    assert!(config.cognition_enabled);
}

// ===========================================================================
// Trait object safety
// ===========================================================================

#[test]
fn memory_bridge_is_object_safe() {
    let _: Box<dyn MemoryBridge> = Box::new(NoOpBridges);
}

#[test]
fn codebase_bridge_is_object_safe() {
    let _: Box<dyn CodebaseBridge> = Box::new(NoOpBridges);
}

#[test]
fn identity_bridge_is_object_safe() {
    let _: Box<dyn IdentityBridge> = Box::new(NoOpBridges);
}

#[test]
fn contract_bridge_is_object_safe() {
    let _: Box<dyn ContractBridge> = Box::new(NoOpBridges);
}

#[test]
fn cognition_bridge_is_object_safe() {
    let _: Box<dyn CognitionBridge> = Box::new(NoOpBridges);
}

#[test]
fn hydra_adapter_trait_exists() {
    // Just confirm the trait is defined and importable
    fn _assert_object_safe(_: &dyn agentic_evolve_core::bridges::HydraAdapter) {}
}

#[test]
fn noop_bridges_is_send_sync() {
    fn _assert_send_sync<T: Send + Sync>() {}
    _assert_send_sync::<NoOpBridges>();
}

#[test]
fn noop_bridges_clone() {
    let a = NoOpBridges;
    let _b = a.clone();
}

#[test]
fn noop_bridges_debug() {
    let bridge = NoOpBridges;
    let debug_str = format!("{:?}", bridge);
    assert!(debug_str.contains("NoOpBridges"));
}

#[test]
fn noop_bridges_default() {
    let _bridge = NoOpBridges::default();
}
