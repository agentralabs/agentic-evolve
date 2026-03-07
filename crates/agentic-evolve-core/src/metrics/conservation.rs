//! Conservation report — aggregates metrics into a verdict on token efficiency.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::audit::AuditLog;
use super::tokens::TokenMetrics;

/// Overall verdict on conservation effectiveness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConservationVerdict {
    /// Conservation score >= 0.9 — outstanding efficiency.
    Excellent,
    /// Conservation score >= 0.7 — target met.
    Good,
    /// Conservation score >= 0.5 — acceptable but improvable.
    Fair,
    /// Conservation score >= 0.3 — significant room for improvement.
    Poor,
    /// Conservation score < 0.3 — conservation is not working.
    Wasteful,
}

impl ConservationVerdict {
    /// Derive a verdict from a conservation score in `[0.0, 1.0]`.
    pub fn from_score(score: f64) -> Self {
        if score >= 0.9 {
            Self::Excellent
        } else if score >= 0.7 {
            Self::Good
        } else if score >= 0.5 {
            Self::Fair
        } else if score >= 0.3 {
            Self::Poor
        } else {
            Self::Wasteful
        }
    }

    /// Whether this verdict meets the 0.7 conservation target.
    pub fn meets_target(&self) -> bool {
        matches!(self, Self::Excellent | Self::Good)
    }
}

/// A comprehensive conservation report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConservationReport {
    /// When this report was generated.
    pub generated_at: DateTime<Utc>,
    /// Overall conservation score `[0.0, 1.0]`.
    pub conservation_score: f64,
    /// The verdict.
    pub verdict: ConservationVerdict,
    /// Total tokens used.
    pub total_tokens_used: u64,
    /// Total tokens saved.
    pub total_tokens_saved: u64,
    /// Cache hit rate.
    pub cache_hit_rate: f64,
    /// Number of queries analyzed.
    pub query_count: usize,
    /// Whether the 0.7 target is met.
    pub target_met: bool,
}

/// Generate a conservation report from the current metrics and audit log.
pub fn generate_report(metrics: &TokenMetrics, audit: &AuditLog) -> ConservationReport {
    let score = metrics.conservation_score();
    let verdict = ConservationVerdict::from_score(score);

    ConservationReport {
        generated_at: Utc::now(),
        conservation_score: score,
        verdict,
        total_tokens_used: metrics.total_used(),
        total_tokens_saved: metrics.total_saved(),
        cache_hit_rate: audit.cache_hit_rate(),
        query_count: audit.len(),
        target_met: verdict.meets_target(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::tokens::Layer;

    #[test]
    fn verdict_from_score_excellent() {
        assert_eq!(ConservationVerdict::from_score(0.95), ConservationVerdict::Excellent);
    }

    #[test]
    fn verdict_from_score_good() {
        assert_eq!(ConservationVerdict::from_score(0.75), ConservationVerdict::Good);
    }

    #[test]
    fn verdict_from_score_fair() {
        assert_eq!(ConservationVerdict::from_score(0.55), ConservationVerdict::Fair);
    }

    #[test]
    fn verdict_from_score_poor() {
        assert_eq!(ConservationVerdict::from_score(0.35), ConservationVerdict::Poor);
    }

    #[test]
    fn verdict_from_score_wasteful() {
        assert_eq!(ConservationVerdict::from_score(0.1), ConservationVerdict::Wasteful);
    }

    #[test]
    fn meets_target_only_good_and_excellent() {
        assert!(ConservationVerdict::Excellent.meets_target());
        assert!(ConservationVerdict::Good.meets_target());
        assert!(!ConservationVerdict::Fair.meets_target());
        assert!(!ConservationVerdict::Poor.meets_target());
        assert!(!ConservationVerdict::Wasteful.meets_target());
    }

    #[test]
    fn generate_report_empty() {
        let metrics = TokenMetrics::new();
        let audit = AuditLog::new();
        let report = generate_report(&metrics, &audit);
        assert_eq!(report.conservation_score, 0.0);
        assert_eq!(report.verdict, ConservationVerdict::Wasteful);
        assert_eq!(report.query_count, 0);
        assert!(!report.target_met);
    }

    #[test]
    fn generate_report_with_savings() {
        let metrics = TokenMetrics::new();
        // 3 cache hits saving 100 each, 1 full costing 100
        metrics.record(Layer::Cache, 0, 100);
        metrics.record(Layer::Cache, 0, 100);
        metrics.record(Layer::Cache, 0, 100);
        metrics.record(Layer::Full, 100, 100);

        let audit = AuditLog::new();
        let report = generate_report(&metrics, &audit);
        // saved 300, used 100, potential 400 => score = 0.75
        assert!((report.conservation_score - 0.75).abs() < f64::EPSILON);
        assert_eq!(report.verdict, ConservationVerdict::Good);
        assert!(report.target_met);
    }
}
