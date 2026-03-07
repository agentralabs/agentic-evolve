//! Cache-level metrics tracking via atomic counters.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use serde::{Deserialize, Serialize};

/// Thread-safe cache performance metrics.
pub struct CacheMetrics {
    hit_count: AtomicU64,
    miss_count: AtomicU64,
    eviction_count: AtomicU64,
    current_size: AtomicUsize,
}

/// Serializable snapshot of cache metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetricsSnapshot {
    pub hit_count: u64,
    pub miss_count: u64,
    pub eviction_count: u64,
    pub current_size: usize,
    pub hit_rate: f64,
}

impl CacheMetrics {
    /// Create zeroed metrics.
    pub fn new() -> Self {
        Self {
            hit_count: AtomicU64::new(0),
            miss_count: AtomicU64::new(0),
            eviction_count: AtomicU64::new(0),
            current_size: AtomicUsize::new(0),
        }
    }

    /// Record a cache hit.
    pub fn record_hit(&self) {
        self.hit_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a cache miss.
    pub fn record_miss(&self) {
        self.miss_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Record an eviction.
    pub fn record_eviction(&self) {
        self.eviction_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Set the current cache size.
    pub fn set_size(&self, size: usize) {
        self.current_size.store(size, Ordering::Relaxed);
    }

    /// Get the hit count.
    pub fn hit_count(&self) -> u64 {
        self.hit_count.load(Ordering::Relaxed)
    }

    /// Get the miss count.
    pub fn miss_count(&self) -> u64 {
        self.miss_count.load(Ordering::Relaxed)
    }

    /// Get the eviction count.
    pub fn eviction_count(&self) -> u64 {
        self.eviction_count.load(Ordering::Relaxed)
    }

    /// Get the current size.
    pub fn current_size(&self) -> usize {
        self.current_size.load(Ordering::Relaxed)
    }

    /// Compute the hit rate as a fraction in `[0.0, 1.0]`.
    ///
    /// Returns `0.0` if no requests have been made.
    pub fn hit_rate(&self) -> f64 {
        let hits = self.hit_count() as f64;
        let total = hits + self.miss_count() as f64;
        if total == 0.0 {
            0.0
        } else {
            hits / total
        }
    }

    /// Take a serializable snapshot of current metrics.
    pub fn snapshot(&self) -> CacheMetricsSnapshot {
        CacheMetricsSnapshot {
            hit_count: self.hit_count(),
            miss_count: self.miss_count(),
            eviction_count: self.eviction_count(),
            current_size: self.current_size(),
            hit_rate: self.hit_rate(),
        }
    }
}

impl Default for CacheMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_metrics_are_zero() {
        let m = CacheMetrics::new();
        assert_eq!(m.hit_count(), 0);
        assert_eq!(m.miss_count(), 0);
        assert_eq!(m.eviction_count(), 0);
        assert_eq!(m.current_size(), 0);
    }

    #[test]
    fn hit_rate_empty_is_zero() {
        let m = CacheMetrics::new();
        assert_eq!(m.hit_rate(), 0.0);
    }

    #[test]
    fn hit_rate_all_hits() {
        let m = CacheMetrics::new();
        m.record_hit();
        m.record_hit();
        assert_eq!(m.hit_rate(), 1.0);
    }

    #[test]
    fn hit_rate_mixed() {
        let m = CacheMetrics::new();
        m.record_hit();
        m.record_miss();
        assert!((m.hit_rate() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn eviction_count_tracks() {
        let m = CacheMetrics::new();
        m.record_eviction();
        m.record_eviction();
        assert_eq!(m.eviction_count(), 2);
    }

    #[test]
    fn set_size_works() {
        let m = CacheMetrics::new();
        m.set_size(42);
        assert_eq!(m.current_size(), 42);
    }

    #[test]
    fn snapshot_captures_state() {
        let m = CacheMetrics::new();
        m.record_hit();
        m.record_miss();
        m.set_size(10);
        let snap = m.snapshot();
        assert_eq!(snap.hit_count, 1);
        assert_eq!(snap.miss_count, 1);
        assert_eq!(snap.current_size, 10);
        assert!((snap.hit_rate - 0.5).abs() < f64::EPSILON);
    }
}
