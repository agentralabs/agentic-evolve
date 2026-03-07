//! Pattern matching — find the best pattern for a given function signature.

pub mod composite;
pub mod context;
pub mod fuzzy;
pub mod semantic;
pub mod signature;

pub use composite::CompositeMatcher;
pub use context::ContextMatcher;
pub use fuzzy::FuzzyMatcher;
pub use semantic::SemanticMatcher;
pub use signature::SignatureMatcher;
