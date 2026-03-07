//! PatternVersioner — manages pattern version history.

use std::collections::HashMap;

use crate::types::error::{EvolveError, EvolveResult};
use crate::types::pattern::Pattern;

/// Version history entry.
#[derive(Debug, Clone)]
pub struct VersionEntry {
    pub version: u32,
    pub pattern_snapshot: String, // JSON
    pub created_at: i64,
    pub change_description: String,
}

/// Manages versioning of patterns.
#[derive(Debug, Default)]
pub struct PatternVersioner {
    history: HashMap<String, Vec<VersionEntry>>,
}

impl PatternVersioner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_version(&mut self, pattern: &Pattern, description: &str) -> EvolveResult<u32> {
        let snapshot = serde_json::to_string(pattern)
            .map_err(|e| EvolveError::SerializationError(e.to_string()))?;
        let entry = VersionEntry {
            version: pattern.version,
            pattern_snapshot: snapshot,
            created_at: chrono::Utc::now().timestamp(),
            change_description: description.to_string(),
        };
        self.history
            .entry(pattern.id.as_str().to_string())
            .or_default()
            .push(entry);
        Ok(pattern.version)
    }

    pub fn get_history(&self, pattern_id: &str) -> Vec<&VersionEntry> {
        self.history
            .get(pattern_id)
            .map(|entries| entries.iter().collect())
            .unwrap_or_default()
    }

    pub fn get_version(&self, pattern_id: &str, version: u32) -> EvolveResult<&VersionEntry> {
        let entries = self
            .history
            .get(pattern_id)
            .ok_or_else(|| EvolveError::PatternNotFound(pattern_id.to_string()))?;
        entries
            .iter()
            .find(|e| e.version == version)
            .ok_or_else(|| EvolveError::PatternNotFound(format!("{pattern_id}@v{version}")))
    }

    pub fn latest_version(&self, pattern_id: &str) -> Option<u32> {
        self.history
            .get(pattern_id)
            .and_then(|entries| entries.last())
            .map(|e| e.version)
    }

    pub fn total_versions(&self) -> usize {
        self.history.values().map(|v| v.len()).sum()
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }
}
