//! CacheManager — manages pattern match cache for fast lookups.

use std::collections::HashMap;

/// Cached match result.
#[derive(Debug, Clone)]
pub struct CachedMatch {
    pub pattern_id: String,
    pub score: f64,
    pub timestamp: i64,
    pub hit_count: u64,
}

/// Manages a cache of recent pattern matches.
#[derive(Debug)]
pub struct CacheManager {
    cache: HashMap<String, Vec<CachedMatch>>,
    max_entries: usize,
    ttl_seconds: i64,
}

impl CacheManager {
    pub fn new(max_entries: usize, ttl_seconds: i64) -> Self {
        Self {
            cache: HashMap::new(),
            max_entries,
            ttl_seconds,
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&Vec<CachedMatch>> {
        let now = chrono::Utc::now().timestamp();
        // Check if expired
        if let Some(entries) = self.cache.get(key) {
            if let Some(first) = entries.first() {
                if now - first.timestamp > self.ttl_seconds {
                    self.cache.remove(key);
                    return None;
                }
            }
        }
        self.cache.get(key)
    }

    pub fn put(&mut self, key: &str, matches: Vec<CachedMatch>) {
        if self.cache.len() >= self.max_entries {
            self.evict_oldest();
        }
        self.cache.insert(key.to_string(), matches);
    }

    pub fn invalidate(&mut self, key: &str) {
        self.cache.remove(key);
    }

    pub fn invalidate_pattern(&mut self, pattern_id: &str) {
        self.cache.retain(|_, entries| {
            !entries.iter().any(|e| e.pattern_id == pattern_id)
        });
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn size(&self) -> usize {
        self.cache.len()
    }

    pub fn hit_rate(&self) -> f64 {
        let total_hits: u64 = self.cache.values()
            .flat_map(|entries| entries.iter())
            .map(|e| e.hit_count)
            .sum();
        let total_entries: u64 = self.cache.values()
            .map(|entries| entries.len() as u64)
            .sum();
        if total_entries == 0 { 0.0 } else { total_hits as f64 / total_entries as f64 }
    }

    fn evict_oldest(&mut self) {
        if let Some(oldest_key) = self.cache.iter()
            .min_by_key(|(_, entries)| entries.first().map(|e| e.timestamp).unwrap_or(i64::MAX))
            .map(|(k, _)| k.clone())
        {
            self.cache.remove(&oldest_key);
        }
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new(1000, 3600) // 1000 entries, 1 hour TTL
    }
}
