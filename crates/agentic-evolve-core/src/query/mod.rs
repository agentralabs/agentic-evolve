//! Query layer — intent-based extraction, delta queries, token budgets, pagination.

pub mod budget;
pub mod delta;
pub mod intent;
pub mod pagination;

pub use budget::TokenBudget;
pub use delta::{ChangeType, DeltaResult, VersionedState};
pub use intent::{ExtractionIntent, ScopedResult, Scopeable};
pub use pagination::CursorPage;
