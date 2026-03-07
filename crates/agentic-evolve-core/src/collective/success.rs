//! SuccessTracker — tracks pattern success rates over time.

use std::collections::HashMap;

/// Success record with history.
#[derive(Debug, Clone, Default)]
pub struct SuccessRecord {
    pub total_attempts: u64,
    pub successes: u64,
    pub failures: u64,
    pub streak: i32, // positive = success streak, negative = failure streak
    pub best_streak: u32,
    pub recent_results: Vec<bool>, // Last N results
}

impl SuccessRecord {
    pub fn success_rate(&self) -> f64 {
        if self.total_attempts == 0 {
            0.0
        } else {
            self.successes as f64 / self.total_attempts as f64
        }
    }

    pub fn recent_success_rate(&self, window: usize) -> f64 {
        let recent: Vec<_> = self.recent_results.iter().rev().take(window).collect();
        if recent.is_empty() {
            return 0.0;
        }
        let successes = recent.iter().filter(|&&r| *r).count();
        successes as f64 / recent.len() as f64
    }
}

/// Tracks success/failure of pattern applications.
#[derive(Debug, Default)]
pub struct SuccessTracker {
    records: HashMap<String, SuccessRecord>,
    max_recent_results: usize,
}

impl SuccessTracker {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
            max_recent_results: 100,
        }
    }

    pub fn record(&mut self, pattern_id: &str, success: bool) {
        let record = self.records.entry(pattern_id.to_string()).or_default();
        record.total_attempts += 1;
        if success {
            record.successes += 1;
            record.streak = if record.streak >= 0 {
                record.streak + 1
            } else {
                1
            };
        } else {
            record.failures += 1;
            record.streak = if record.streak <= 0 {
                record.streak - 1
            } else {
                -1
            };
        }
        if record.streak > 0 {
            record.best_streak = record.best_streak.max(record.streak as u32);
        }
        record.recent_results.push(success);
        if record.recent_results.len() > self.max_recent_results {
            record.recent_results.remove(0);
        }
    }

    pub fn get(&self, pattern_id: &str) -> Option<&SuccessRecord> {
        self.records.get(pattern_id)
    }

    pub fn success_rate(&self, pattern_id: &str) -> f64 {
        self.records
            .get(pattern_id)
            .map_or(0.0, |r| r.success_rate())
    }

    pub fn top_performers(&self, limit: usize) -> Vec<(&str, f64)> {
        let mut entries: Vec<_> = self
            .records
            .iter()
            .filter(|(_, r)| r.total_attempts >= 3)
            .map(|(k, r)| (k.as_str(), r.success_rate()))
            .collect();
        entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        entries.truncate(limit);
        entries
    }

    pub fn underperformers(&self, threshold: f64) -> Vec<(&str, f64)> {
        self.records
            .iter()
            .filter(|(_, r)| r.total_attempts >= 3 && r.success_rate() < threshold)
            .map(|(k, r)| (k.as_str(), r.success_rate()))
            .collect()
    }

    pub fn total_tracked(&self) -> usize {
        self.records.len()
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }
}
