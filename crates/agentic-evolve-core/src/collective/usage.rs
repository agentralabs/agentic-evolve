//! UsageTracker — tracks pattern usage statistics.

use std::collections::HashMap;

/// Usage record for a pattern.
#[derive(Debug, Clone, Default)]
pub struct UsageRecord {
    pub total_uses: u64,
    pub successful_uses: u64,
    pub failed_uses: u64,
    pub last_used: i64,
    pub first_used: i64,
    pub domains: HashMap<String, u64>,
}

/// Tracks how often patterns are used.
#[derive(Debug, Default)]
pub struct UsageTracker {
    records: HashMap<String, UsageRecord>,
}

impl UsageTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_use(&mut self, pattern_id: &str, domain: &str, success: bool) {
        let now = chrono::Utc::now().timestamp();
        let record = self.records.entry(pattern_id.to_string()).or_insert_with(|| UsageRecord {
            first_used: now,
            ..Default::default()
        });
        record.total_uses += 1;
        if success {
            record.successful_uses += 1;
        } else {
            record.failed_uses += 1;
        }
        record.last_used = now;
        *record.domains.entry(domain.to_string()).or_insert(0) += 1;
    }

    pub fn get_usage(&self, pattern_id: &str) -> Option<&UsageRecord> {
        self.records.get(pattern_id)
    }

    pub fn success_rate(&self, pattern_id: &str) -> f64 {
        self.records.get(pattern_id).map_or(0.0, |r| {
            if r.total_uses == 0 { 0.0 } else { r.successful_uses as f64 / r.total_uses as f64 }
        })
    }

    pub fn most_used(&self, limit: usize) -> Vec<(&str, &UsageRecord)> {
        let mut entries: Vec<_> = self.records.iter().map(|(k, v)| (k.as_str(), v)).collect();
        entries.sort_by(|a, b| b.1.total_uses.cmp(&a.1.total_uses));
        entries.truncate(limit);
        entries
    }

    pub fn least_used(&self, limit: usize) -> Vec<(&str, &UsageRecord)> {
        let mut entries: Vec<_> = self.records.iter().map(|(k, v)| (k.as_str(), v)).collect();
        entries.sort_by(|a, b| a.1.total_uses.cmp(&b.1.total_uses));
        entries.truncate(limit);
        entries
    }

    pub fn total_patterns_tracked(&self) -> usize {
        self.records.len()
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }
}
