//! Phase 8: Optimization tests — PatternOptimizer, CacheManager.

use agentic_evolve_core::optimization::cache::{CacheManager, CachedMatch};
use agentic_evolve_core::optimization::optimizer::PatternOptimizer;
use agentic_evolve_core::types::pattern::{FunctionSignature, Language, Pattern};

fn make_pattern(name: &str, template: &str, confidence: f64) -> Pattern {
    let sig = FunctionSignature::new(name, Language::Rust);
    Pattern::new(
        name,
        "test",
        Language::Rust,
        sig,
        template,
        vec![],
        confidence,
    )
}

fn make_cached_match(pattern_id: &str, score: f64) -> CachedMatch {
    CachedMatch {
        pattern_id: pattern_id.to_string(),
        score,
        timestamp: chrono::Utc::now().timestamp(),
        hit_count: 1,
    }
}

// ===========================================================================
// PatternOptimizer
// ===========================================================================

#[test]
fn optimizer_find_duplicates() {
    let opt = PatternOptimizer::new();
    // Same template = same content_hash
    let p1 = make_pattern("a", "fn same() {}", 0.8);
    let p2 = make_pattern("b", "fn same() {}", 0.8);
    let dups = opt.find_duplicates(&[&p1, &p2]);
    assert_eq!(dups.len(), 1);
}

#[test]
fn optimizer_no_duplicates() {
    let opt = PatternOptimizer::new();
    let p1 = make_pattern("a", "fn one() {}", 0.8);
    let p2 = make_pattern("b", "fn two() {}", 0.8);
    let dups = opt.find_duplicates(&[&p1, &p2]);
    assert!(dups.is_empty());
}

#[test]
fn optimizer_find_similar() {
    let opt = PatternOptimizer::new();
    let p1 = make_pattern("a", "fn test() {\n  let x = 1;\n  x\n}", 0.8);
    let p2 = make_pattern("b", "fn test() {\n  let x = 2;\n  x\n}", 0.8);
    let similar = opt.find_similar(&[&p1, &p2], 0.3);
    assert!(
        !similar.is_empty(),
        "Very similar templates should be detected"
    );
}

#[test]
fn optimizer_find_similar_below_threshold() {
    let opt = PatternOptimizer::new();
    let p1 = make_pattern("a", "completely different code", 0.8);
    let p2 = make_pattern("b", "fn entirely_other() { 42 }", 0.8);
    let similar = opt.find_similar(&[&p1, &p2], 0.9);
    assert!(similar.is_empty());
}

#[test]
fn optimizer_suggest_pruning() {
    let opt = PatternOptimizer::new();
    let mut p1 = make_pattern("low_conf", "fn a() {}", 0.1);
    p1.usage_count = 0;
    let mut p2 = make_pattern("high_conf", "fn b() {}", 0.9);
    p2.usage_count = 10;
    let prunable = opt.suggest_pruning(&[&p1, &p2], 0.2, 2);
    assert_eq!(prunable.len(), 1);
    assert_eq!(prunable[0], p1.id.as_str());
}

#[test]
fn optimizer_suggest_pruning_none() {
    let opt = PatternOptimizer::new();
    let mut p = make_pattern("good", "fn a() {}", 0.9);
    p.usage_count = 100;
    let prunable = opt.suggest_pruning(&[&p], 0.2, 2);
    assert!(prunable.is_empty());
}

#[test]
fn optimizer_optimize_report() {
    let opt = PatternOptimizer::new();
    let p1 = make_pattern("a", "fn same() {}", 0.8);
    let p2 = make_pattern("b", "fn same() {}", 0.8);
    let mut p3 = make_pattern("c", "fn other() {}", 0.05);
    p3.usage_count = 0;
    let report = opt.optimize_report(&[&p1, &p2, &p3]);
    assert_eq!(report.patterns_before, 3);
    assert!(report.duplicates_removed > 0);
    assert!(report.patterns_after < report.patterns_before);
}

#[test]
fn optimizer_empty_patterns() {
    let opt = PatternOptimizer::new();
    let report = opt.optimize_report(&[]);
    assert_eq!(report.patterns_before, 0);
    assert_eq!(report.duplicates_removed, 0);
}

// ===========================================================================
// CacheManager
// ===========================================================================

#[test]
fn cache_put_and_get() {
    let mut cache = CacheManager::new(100, 3600);
    let matches = vec![make_cached_match("p1", 0.9)];
    cache.put("query1", matches);
    let result = cache.get("query1");
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 1);
}

#[test]
fn cache_get_missing() {
    let mut cache = CacheManager::new(100, 3600);
    assert!(cache.get("nonexistent").is_none());
}

#[test]
fn cache_invalidate() {
    let mut cache = CacheManager::new(100, 3600);
    cache.put("q1", vec![make_cached_match("p1", 0.9)]);
    cache.invalidate("q1");
    assert!(cache.get("q1").is_none());
}

#[test]
fn cache_clear() {
    let mut cache = CacheManager::new(100, 3600);
    cache.put("q1", vec![make_cached_match("p1", 0.9)]);
    cache.put("q2", vec![make_cached_match("p2", 0.8)]);
    cache.clear();
    assert_eq!(cache.size(), 0);
}

#[test]
fn cache_size() {
    let mut cache = CacheManager::new(100, 3600);
    assert_eq!(cache.size(), 0);
    cache.put("q1", vec![make_cached_match("p1", 0.9)]);
    assert_eq!(cache.size(), 1);
    cache.put("q2", vec![make_cached_match("p2", 0.8)]);
    assert_eq!(cache.size(), 2);
}

#[test]
fn cache_hit_rate_no_entries() {
    let cache = CacheManager::new(100, 3600);
    assert_eq!(cache.hit_rate(), 0.0);
}

#[test]
fn cache_hit_rate_with_entries() {
    let mut cache = CacheManager::new(100, 3600);
    cache.put(
        "q1",
        vec![CachedMatch {
            pattern_id: "p1".to_string(),
            score: 0.9,
            timestamp: chrono::Utc::now().timestamp(),
            hit_count: 5,
        }],
    );
    let rate = cache.hit_rate();
    assert!(rate > 0.0);
}

#[test]
fn cache_eviction() {
    let mut cache = CacheManager::new(2, 3600); // max 2 entries
    cache.put("q1", vec![make_cached_match("p1", 0.9)]);
    cache.put("q2", vec![make_cached_match("p2", 0.8)]);
    cache.put("q3", vec![make_cached_match("p3", 0.7)]);
    // Should have evicted one entry
    assert!(cache.size() <= 2);
}

#[test]
fn cache_ttl_expiry() {
    let mut cache = CacheManager::new(100, 1); // 1 second TTL
    let expired_match = CachedMatch {
        pattern_id: "p1".to_string(),
        score: 0.9,
        timestamp: chrono::Utc::now().timestamp() - 10, // 10 seconds ago
        hit_count: 1,
    };
    cache.put("old_query", vec![expired_match]);
    // Should be expired
    let result = cache.get("old_query");
    assert!(result.is_none(), "Expired entries should not be returned");
}

#[test]
fn cache_invalidate_pattern() {
    let mut cache = CacheManager::new(100, 3600);
    cache.put("q1", vec![make_cached_match("p1", 0.9)]);
    cache.put("q2", vec![make_cached_match("p2", 0.8)]);
    cache.put("q3", vec![make_cached_match("p1", 0.7)]);
    cache.invalidate_pattern("p1");
    // q1 and q3 contained p1 so should be removed
    assert!(cache.size() <= 1);
}

#[test]
fn cache_default() {
    let cache = CacheManager::default();
    assert_eq!(cache.size(), 0);
}
