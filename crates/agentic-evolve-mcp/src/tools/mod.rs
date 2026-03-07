//! MCP tool implementations — the primary way LLMs interact with the pattern library.

pub mod evolve_compose;
pub mod evolve_confidence;
pub mod evolve_coverage;
pub mod evolve_crystallize;
pub mod evolve_get_body;
pub mod evolve_match_context;
pub mod evolve_match_signature;
pub mod evolve_optimize;
pub mod evolve_pattern_delete;
pub mod evolve_pattern_get;
pub mod evolve_pattern_list;
pub mod evolve_pattern_search;
pub mod evolve_pattern_store;
pub mod evolve_update_usage;
pub mod registry;

pub use registry::ToolRegistry;
