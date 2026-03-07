//! Pattern optimization — storage optimization and cache management.

pub mod cache;
pub mod optimizer;

pub use cache::CacheManager;
pub use optimizer::PatternOptimizer;
