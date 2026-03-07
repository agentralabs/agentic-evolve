//! Audit log — records every query with token usage for analysis.

use std::sync::Mutex;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::tokens::Layer;
use crate::query::intent::ExtractionIntent;

/// A single audit entry recording one query's token usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// When this query was made.
    pub timestamp: DateTime<Utc>,
    /// The tool/endpoint that was called.
    pub tool: String,
    /// Which layer served the response.
    pub layer: Layer,
    /// Tokens actually used.
    pub tokens_used: u64,
    /// Tokens saved compared to a full retrieval.
    pub tokens_saved: u64,
    /// Whether the cache was hit.
    pub cache_hit: bool,
    /// The extraction intent used.
    pub intent: ExtractionIntent,
    /// Size of the source data (items or bytes).
    pub source_size: u64,
    /// Size of the result data.
    pub result_size: u64,
}

impl AuditEntry {
    /// Create a new audit entry with the current timestamp.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        tool: impl Into<String>,
        layer: Layer,
        tokens_used: u64,
        tokens_saved: u64,
        cache_hit: bool,
        intent: ExtractionIntent,
        source_size: u64,
        result_size: u64,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            tool: tool.into(),
            layer,
            tokens_used,
            tokens_saved,
            cache_hit,
            intent,
            source_size,
            result_size,
        }
    }
}

/// An append-only audit log of query entries.
pub struct AuditLog {
    entries: Mutex<Vec<AuditEntry>>,
}

impl AuditLog {
    /// Create an empty audit log.
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(Vec::new()),
        }
    }

    /// Append an entry.
    pub fn record(&self, entry: AuditEntry) {
        self.entries.lock().unwrap().push(entry);
    }

    /// Number of recorded entries.
    pub fn len(&self) -> usize {
        self.entries.lock().unwrap().len()
    }

    /// Whether the log is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Total tokens used across all entries.
    pub fn total_tokens_used(&self) -> u64 {
        self.entries
            .lock()
            .unwrap()
            .iter()
            .map(|e| e.tokens_used)
            .sum()
    }

    /// Total tokens saved across all entries.
    pub fn total_tokens_saved(&self) -> u64 {
        self.entries
            .lock()
            .unwrap()
            .iter()
            .map(|e| e.tokens_saved)
            .sum()
    }

    /// Cache hit rate across all entries.
    pub fn cache_hit_rate(&self) -> f64 {
        let entries = self.entries.lock().unwrap();
        if entries.is_empty() {
            return 0.0;
        }
        let hits = entries.iter().filter(|e| e.cache_hit).count() as f64;
        hits / entries.len() as f64
    }

    /// Distribution of queries across layers: `(layer, count)`.
    pub fn layer_distribution(&self) -> Vec<(Layer, usize)> {
        let entries = self.entries.lock().unwrap();
        let mut cache = 0usize;
        let mut index = 0usize;
        let mut scoped = 0usize;
        let mut delta = 0usize;
        let mut full = 0usize;

        for e in entries.iter() {
            match e.layer {
                Layer::Cache => cache += 1,
                Layer::Index => index += 1,
                Layer::Scoped => scoped += 1,
                Layer::Delta => delta += 1,
                Layer::Full => full += 1,
            }
        }

        vec![
            (Layer::Cache, cache),
            (Layer::Index, index),
            (Layer::Scoped, scoped),
            (Layer::Delta, delta),
            (Layer::Full, full),
        ]
    }

    /// Get a clone of all entries.
    pub fn entries(&self) -> Vec<AuditEntry> {
        self.entries.lock().unwrap().clone()
    }

    /// Clear the log.
    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
    }
}

impl Default for AuditLog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(tool: &str, layer: Layer, used: u64, saved: u64, hit: bool) -> AuditEntry {
        AuditEntry::new(
            tool,
            layer,
            used,
            saved,
            hit,
            ExtractionIntent::IdsOnly,
            100,
            10,
        )
    }

    #[test]
    fn empty_log() {
        let log = AuditLog::new();
        assert!(log.is_empty());
        assert_eq!(log.total_tokens_used(), 0);
        assert_eq!(log.cache_hit_rate(), 0.0);
    }

    #[test]
    fn record_and_count() {
        let log = AuditLog::new();
        log.record(make_entry("tool_a", Layer::Cache, 0, 100, true));
        log.record(make_entry("tool_b", Layer::Full, 100, 0, false));
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn total_tokens_used() {
        let log = AuditLog::new();
        log.record(make_entry("a", Layer::Cache, 0, 100, true));
        log.record(make_entry("b", Layer::Scoped, 10, 90, false));
        assert_eq!(log.total_tokens_used(), 10);
    }

    #[test]
    fn total_tokens_saved() {
        let log = AuditLog::new();
        log.record(make_entry("a", Layer::Cache, 0, 100, true));
        log.record(make_entry("b", Layer::Full, 100, 0, false));
        assert_eq!(log.total_tokens_saved(), 100);
    }

    #[test]
    fn cache_hit_rate_mixed() {
        let log = AuditLog::new();
        log.record(make_entry("a", Layer::Cache, 0, 100, true));
        log.record(make_entry("b", Layer::Full, 100, 0, false));
        assert!((log.cache_hit_rate() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn layer_distribution_counts() {
        let log = AuditLog::new();
        log.record(make_entry("a", Layer::Cache, 0, 100, true));
        log.record(make_entry("b", Layer::Cache, 0, 50, true));
        log.record(make_entry("c", Layer::Full, 100, 0, false));
        let dist = log.layer_distribution();
        let cache_count = dist.iter().find(|(l, _)| *l == Layer::Cache).unwrap().1;
        let full_count = dist.iter().find(|(l, _)| *l == Layer::Full).unwrap().1;
        assert_eq!(cache_count, 2);
        assert_eq!(full_count, 1);
    }

    #[test]
    fn clear_empties_log() {
        let log = AuditLog::new();
        log.record(make_entry("a", Layer::Cache, 0, 100, true));
        log.clear();
        assert!(log.is_empty());
    }

    #[test]
    fn entries_returns_clone() {
        let log = AuditLog::new();
        log.record(make_entry("a", Layer::Scoped, 10, 90, false));
        let entries = log.entries();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].tool, "a");
    }
}
