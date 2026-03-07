//! AgenticEvolve — pattern library engine for AI agents.
//!
//! Crystallizes verified code patterns for reuse, providing ~80% of function
//! bodies from patterns on subsequent builds.

pub mod bridges;
pub mod cache;
pub mod collective;
pub mod composition;
pub mod crystallization;
pub mod matching;
pub mod metrics;
pub mod optimization;
pub mod query;
pub mod storage;
pub mod types;

pub use types::error::{EvolveError, EvolveResult};
pub use types::ids::{EvolveId, PatternId, SkillId};
pub use types::match_result::{MatchContext, MatchResult, MatchScore};
pub use types::pattern::{FunctionSignature, Language, Pattern, PatternVariable};
pub use types::skill::{CrystallizedSkill, SkillMetadata, SuccessfulExecution};
