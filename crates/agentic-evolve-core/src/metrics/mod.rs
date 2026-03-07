//! Metrics layer — token tracking, audit logging, conservation scoring.

pub mod audit;
pub mod conservation;
pub mod tokens;

pub use audit::{AuditEntry, AuditLog};
pub use conservation::{ConservationReport, ConservationVerdict};
pub use tokens::{Layer, ResponseMetrics, TokenMetrics};
