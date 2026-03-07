//! Generic LRU cache with time-to-live expiration.

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::RwLock;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use super::metrics::CacheMetrics;

/// An entry in the LRU cache, tracking value, insertion time, and access order.
struct CacheEntry<V> {
    value: V,
    inserted_at: Instant,
    last_accessed: Instant,
}

/// A generic LRU cache with configurable max size and TTL.
///
/// Thread-safe via internal `RwLock`. Expired entries are lazily evicted on access.
pub struct LruCache<K, V> {
    entries: RwLock<HashMap<K, CacheEntry<V>>>,
    max_size: usize,
    ttl: Duration,
    metrics: CacheMetrics,
}

/// Serializable configuration for constructing an `LruCache`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LruCacheConfig {
    pub max_size: usize,
    pub ttl_secs: u64,
}

impl Default for LruCacheConfig {
    fn default() -> Self {
        Self {
            max_size: 1024,
            ttl_secs: 300,
        }
    }
}

impl<K, V> LruCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Create a new LRU cache with the given capacity and TTL.
    pub fn new(max_size: usize, ttl: Duration) -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            max_size,
            ttl,
            metrics: CacheMetrics::new(),
        }
    }

    /// Create from a serializable config.
    pub fn from_config(config: &LruCacheConfig) -> Self {
        Self::new(config.max_size, Duration::from_secs(config.ttl_secs))
    }

    /// Get a value by key. Returns `None` if missing or expired.
    pub fn get(&self, key: &K) -> Option<V> {
        let mut map = self.entries.write().unwrap();
        match map.get_mut(key) {
            Some(entry) if entry.inserted_at.elapsed() < self.ttl => {
                entry.last_accessed = Instant::now();
                self.metrics.record_hit();
                Some(entry.value.clone())
            }
            Some(_) => {
                // Expired — remove it.
                map.remove(key);
                self.metrics.record_eviction();
                self.metrics.record_miss();
                self.metrics.set_size(map.len());
                None
            }
            None => {
                self.metrics.record_miss();
                None
            }
        }
    }

    /// Insert a key-value pair. Evicts the LRU entry if at capacity.
    pub fn insert(&self, key: K, value: V) {
        let mut map = self.entries.write().unwrap();

        // Evict expired entries first.
        self.evict_expired(&mut map);

        // If still at capacity, evict the least-recently-accessed entry.
        if map.len() >= self.max_size && !map.contains_key(&key) {
            self.evict_lru(&mut map);
        }

        let now = Instant::now();
        map.insert(
            key,
            CacheEntry {
                value,
                inserted_at: now,
                last_accessed: now,
            },
        );
        self.metrics.set_size(map.len());
    }

    /// Invalidate (remove) a specific key.
    pub fn invalidate(&self, key: &K) -> bool {
        let mut map = self.entries.write().unwrap();
        let removed = map.remove(key).is_some();
        if removed {
            self.metrics.record_eviction();
        }
        self.metrics.set_size(map.len());
        removed
    }

    /// Clear the entire cache.
    pub fn clear(&self) {
        let mut map = self.entries.write().unwrap();
        let count = map.len();
        map.clear();
        for _ in 0..count {
            self.metrics.record_eviction();
        }
        self.metrics.set_size(0);
    }

    /// Check if a key exists and is not expired.
    pub fn contains(&self, key: &K) -> bool {
        let map = self.entries.read().unwrap();
        match map.get(key) {
            Some(entry) => entry.inserted_at.elapsed() < self.ttl,
            None => false,
        }
    }

    /// Return the number of entries (including possibly-expired ones).
    pub fn len(&self) -> usize {
        self.entries.read().unwrap().len()
    }

    /// Check if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Access the cache metrics.
    pub fn metrics(&self) -> &CacheMetrics {
        &self.metrics
    }

    // -- internal helpers --

    fn evict_expired(&self, map: &mut HashMap<K, CacheEntry<V>>) {
        let expired: Vec<K> = map
            .iter()
            .filter(|(_, e)| e.inserted_at.elapsed() >= self.ttl)
            .map(|(k, _)| k.clone())
            .collect();
        for k in expired {
            map.remove(&k);
            self.metrics.record_eviction();
        }
    }

    fn evict_lru(&self, map: &mut HashMap<K, CacheEntry<V>>) {
        if let Some(lru_key) = map
            .iter()
            .min_by_key(|(_, e)| e.last_accessed)
            .map(|(k, _)| k.clone())
        {
            map.remove(&lru_key);
            self.metrics.record_eviction();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn insert_and_get() {
        let cache = LruCache::new(10, Duration::from_secs(60));
        cache.insert("a", 1);
        assert_eq!(cache.get(&"a"), Some(1));
    }

    #[test]
    fn missing_key_returns_none() {
        let cache: LruCache<&str, i32> = LruCache::new(10, Duration::from_secs(60));
        assert_eq!(cache.get(&"missing"), None);
    }

    #[test]
    fn ttl_expiration() {
        let cache = LruCache::new(10, Duration::from_millis(50));
        cache.insert("x", 42);
        assert_eq!(cache.get(&"x"), Some(42));
        thread::sleep(Duration::from_millis(60));
        assert_eq!(cache.get(&"x"), None);
    }

    #[test]
    fn evict_lru_on_full() {
        let cache = LruCache::new(2, Duration::from_secs(60));
        cache.insert("a", 1);
        cache.insert("b", 2);
        // Access "a" to make "b" the LRU.
        let _ = cache.get(&"a");
        cache.insert("c", 3);
        // "b" should have been evicted.
        assert_eq!(cache.get(&"b"), None);
        assert_eq!(cache.get(&"a"), Some(1));
        assert_eq!(cache.get(&"c"), Some(3));
    }

    #[test]
    fn invalidate_removes_key() {
        let cache = LruCache::new(10, Duration::from_secs(60));
        cache.insert("k", 99);
        assert!(cache.invalidate(&"k"));
        assert_eq!(cache.get(&"k"), None);
    }

    #[test]
    fn clear_empties_cache() {
        let cache = LruCache::new(10, Duration::from_secs(60));
        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn contains_respects_ttl() {
        let cache = LruCache::new(10, Duration::from_millis(50));
        cache.insert("k", 1);
        assert!(cache.contains(&"k"));
        thread::sleep(Duration::from_millis(60));
        assert!(!cache.contains(&"k"));
    }

    #[test]
    fn metrics_track_hits_and_misses() {
        let cache = LruCache::new(10, Duration::from_secs(60));
        cache.insert("a", 1);
        let _ = cache.get(&"a"); // hit
        let _ = cache.get(&"b"); // miss
        assert_eq!(cache.metrics().hit_count(), 1);
        assert_eq!(cache.metrics().miss_count(), 1);
    }

    #[test]
    fn from_config_works() {
        let config = LruCacheConfig {
            max_size: 5,
            ttl_secs: 120,
        };
        let cache: LruCache<String, String> = LruCache::from_config(&config);
        assert_eq!(cache.max_size, 5);
    }

    #[test]
    fn len_tracks_insertions() {
        let cache = LruCache::new(10, Duration::from_secs(60));
        assert_eq!(cache.len(), 0);
        cache.insert("a", 1);
        assert_eq!(cache.len(), 1);
        cache.insert("b", 2);
        assert_eq!(cache.len(), 2);
    }
}
