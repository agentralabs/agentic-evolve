//! Per-layer token tracking and conservation scoring.

use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};

/// The processing layer that handled a request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Layer {
    /// Served from cache — zero or near-zero token cost.
    Cache,
    /// Served from an in-memory index.
    Index,
    /// Served via scoped extraction (intent-based).
    Scoped,
    /// Served via delta (only changes returned).
    Delta,
    /// Full data retrieval — maximum token cost.
    Full,
}

impl Layer {
    /// Typical token cost multiplier for this layer relative to `Full`.
    pub fn cost_multiplier(&self) -> f64 {
        match self {
            Self::Cache => 0.0,
            Self::Index => 0.05,
            Self::Scoped => 0.1,
            Self::Delta => 0.2,
            Self::Full => 1.0,
        }
    }
}

/// Thread-safe token metrics tracked per layer.
pub struct TokenMetrics {
    cache_tokens: AtomicU64,
    index_tokens: AtomicU64,
    scoped_tokens: AtomicU64,
    delta_tokens: AtomicU64,
    full_tokens: AtomicU64,
    tokens_saved: AtomicU64,
}

impl TokenMetrics {
    /// Create zeroed metrics.
    pub fn new() -> Self {
        Self {
            cache_tokens: AtomicU64::new(0),
            index_tokens: AtomicU64::new(0),
            scoped_tokens: AtomicU64::new(0),
            delta_tokens: AtomicU64::new(0),
            full_tokens: AtomicU64::new(0),
            tokens_saved: AtomicU64::new(0),
        }
    }

    /// Record tokens used at a given layer, along with what the full cost
    /// would have been (to compute savings).
    pub fn record(&self, layer: Layer, tokens_used: u64, full_cost: u64) {
        let counter = match layer {
            Layer::Cache => &self.cache_tokens,
            Layer::Index => &self.index_tokens,
            Layer::Scoped => &self.scoped_tokens,
            Layer::Delta => &self.delta_tokens,
            Layer::Full => &self.full_tokens,
        };
        counter.fetch_add(tokens_used, Ordering::Relaxed);

        if full_cost > tokens_used {
            self.tokens_saved
                .fetch_add(full_cost - tokens_used, Ordering::Relaxed);
        }
    }

    /// Total tokens used across all layers.
    pub fn total_used(&self) -> u64 {
        self.layer_tokens(Layer::Cache)
            + self.layer_tokens(Layer::Index)
            + self.layer_tokens(Layer::Scoped)
            + self.layer_tokens(Layer::Delta)
            + self.layer_tokens(Layer::Full)
    }

    /// Tokens used at a specific layer.
    pub fn layer_tokens(&self, layer: Layer) -> u64 {
        match layer {
            Layer::Cache => self.cache_tokens.load(Ordering::Relaxed),
            Layer::Index => self.index_tokens.load(Ordering::Relaxed),
            Layer::Scoped => self.scoped_tokens.load(Ordering::Relaxed),
            Layer::Delta => self.delta_tokens.load(Ordering::Relaxed),
            Layer::Full => self.full_tokens.load(Ordering::Relaxed),
        }
    }

    /// Total tokens saved by using lower layers instead of Full.
    pub fn total_saved(&self) -> u64 {
        self.tokens_saved.load(Ordering::Relaxed)
    }

    /// Conservation score: ratio of tokens saved to total potential cost.
    ///
    /// Returns a value in `[0.0, 1.0]` where 1.0 means all tokens were saved.
    pub fn conservation_score(&self) -> f64 {
        let used = self.total_used() as f64;
        let saved = self.total_saved() as f64;
        let potential = used + saved;
        if potential == 0.0 {
            0.0
        } else {
            saved / potential
        }
    }
}

impl Default for TokenMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Serializable snapshot of a single response's token metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetrics {
    /// Which layer served this response.
    pub layer: Layer,
    /// Tokens actually used.
    pub tokens_used: u64,
    /// Tokens that would have been used at the Full layer.
    pub full_cost: u64,
    /// Tokens saved.
    pub tokens_saved: u64,
}

impl ResponseMetrics {
    /// Create response metrics for a given layer and cost.
    pub fn new(layer: Layer, tokens_used: u64, full_cost: u64) -> Self {
        Self {
            layer,
            tokens_used,
            full_cost,
            tokens_saved: full_cost.saturating_sub(tokens_used),
        }
    }

    /// Conservation ratio for this single response.
    pub fn conservation_ratio(&self) -> f64 {
        if self.full_cost == 0 {
            0.0
        } else {
            self.tokens_saved as f64 / self.full_cost as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_cost_ordering() {
        assert!(Layer::Cache.cost_multiplier() < Layer::Index.cost_multiplier());
        assert!(Layer::Index.cost_multiplier() < Layer::Scoped.cost_multiplier());
        assert!(Layer::Scoped.cost_multiplier() < Layer::Delta.cost_multiplier());
        assert!(Layer::Delta.cost_multiplier() < Layer::Full.cost_multiplier());
    }

    #[test]
    fn new_metrics_zero() {
        let m = TokenMetrics::new();
        assert_eq!(m.total_used(), 0);
        assert_eq!(m.total_saved(), 0);
        assert_eq!(m.conservation_score(), 0.0);
    }

    #[test]
    fn record_tracks_per_layer() {
        let m = TokenMetrics::new();
        m.record(Layer::Cache, 0, 100);
        m.record(Layer::Full, 100, 100);
        assert_eq!(m.layer_tokens(Layer::Cache), 0);
        assert_eq!(m.layer_tokens(Layer::Full), 100);
        assert_eq!(m.total_saved(), 100);
    }

    #[test]
    fn conservation_score_perfect() {
        let m = TokenMetrics::new();
        m.record(Layer::Cache, 0, 100);
        assert!((m.conservation_score() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn conservation_score_none() {
        let m = TokenMetrics::new();
        m.record(Layer::Full, 100, 100);
        assert_eq!(m.conservation_score(), 0.0);
    }

    #[test]
    fn conservation_score_mixed() {
        let m = TokenMetrics::new();
        m.record(Layer::Cache, 0, 100); // saved 100
        m.record(Layer::Full, 100, 100); // saved 0
        // total used = 100, total saved = 100, potential = 200
        assert!((m.conservation_score() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn response_metrics_conservation_ratio() {
        let rm = ResponseMetrics::new(Layer::Cache, 0, 100);
        assert!((rm.conservation_ratio() - 1.0).abs() < f64::EPSILON);

        let rm2 = ResponseMetrics::new(Layer::Scoped, 10, 100);
        assert!((rm2.conservation_ratio() - 0.9).abs() < f64::EPSILON);
    }

    #[test]
    fn response_metrics_full_layer() {
        let rm = ResponseMetrics::new(Layer::Full, 100, 100);
        assert_eq!(rm.tokens_saved, 0);
        assert_eq!(rm.conservation_ratio(), 0.0);
    }
}
