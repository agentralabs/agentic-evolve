//! Collective learning — track usage, success, decay, and promotion.

pub mod decay;
pub mod promotion;
pub mod success;
pub mod usage;

pub use decay::DecayManager;
pub use promotion::PromotionEngine;
pub use success::SuccessTracker;
pub use usage::UsageTracker;
