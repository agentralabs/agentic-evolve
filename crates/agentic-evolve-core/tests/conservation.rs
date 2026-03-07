//! Token Conservation Architecture integration tests.

use std::time::Duration;

use agentic_evolve_core::cache::invalidation::CacheInvalidator;
use agentic_evolve_core::cache::lru::LruCache;
use agentic_evolve_core::metrics::audit::{AuditEntry, AuditLog};
use agentic_evolve_core::metrics::conservation::generate_report;
use agentic_evolve_core::metrics::tokens::{Layer, TokenMetrics};
use agentic_evolve_core::query::budget::TokenBudget;
use agentic_evolve_core::query::delta::VersionedState;
use agentic_evolve_core::query::intent::ExtractionIntent;

// ---------------------------------------------------------------------------
// 1. Second query is cheaper (cache hit = 0 tokens)
// ---------------------------------------------------------------------------

#[test]
fn test_second_query_cheaper() {
    let cache = LruCache::new(100, Duration::from_secs(60));
    let metrics = TokenMetrics::new();

    // First query: cache miss, full retrieval.
    assert_eq!(cache.get(&"patterns"), None);
    metrics.record(Layer::Full, 100, 100);

    // Populate cache.
    cache.insert("patterns", "cached-result");

    // Second query: cache hit, zero tokens.
    assert_eq!(cache.get(&"patterns"), Some("cached-result"));
    metrics.record(Layer::Cache, 0, 100);

    // The second query cost zero tokens.
    assert_eq!(metrics.layer_tokens(Layer::Cache), 0);
    assert_eq!(metrics.total_saved(), 100);
}

// ---------------------------------------------------------------------------
// 2. Unchanged state is free (empty delta)
// ---------------------------------------------------------------------------

#[test]
fn test_unchanged_state_free() {
    let state = VersionedState::from_data(vec!["a", "b", "c"]);
    let current_version = state.version();

    // Query at the current version should return Unchanged.
    let result = state.changes_since_version(current_version);
    match result {
        agentic_evolve_core::query::delta::DeltaResult::Unchanged { version } => {
            assert_eq!(version, current_version);
        }
        _ => panic!("Expected Unchanged delta for current version"),
    }

    // No tokens should be needed for an unchanged response.
    let metrics = TokenMetrics::new();
    metrics.record(Layer::Delta, 0, 100);
    assert_eq!(metrics.total_saved(), 100);
}

// ---------------------------------------------------------------------------
// 3. Scoped query is 10x cheaper (IdsOnly vs Full)
// ---------------------------------------------------------------------------

#[test]
fn test_scoped_query_10x_cheaper() {
    let ids_cost = ExtractionIntent::IdsOnly.estimated_tokens();
    let full_cost = ExtractionIntent::Full.estimated_tokens();

    // IdsOnly should be at least 10x cheaper than Full.
    assert!(
        full_cost >= ids_cost * 10,
        "Full ({full_cost}) should be >= 10x IdsOnly ({ids_cost})"
    );
}

// ---------------------------------------------------------------------------
// 4. Delta is proportional to changes
// ---------------------------------------------------------------------------

#[test]
fn test_delta_proportional_to_changes() {
    let mut state = VersionedState::from_data(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let baseline = state.version();

    // Make 2 changes out of 10 items.
    state.add(11);
    state.add(12);

    let result = state.changes_since_version(baseline);
    match result {
        agentic_evolve_core::query::delta::DeltaResult::Changed { items, .. } => {
            // Only 2 items returned, not the full 12.
            assert_eq!(items.len(), 2);
            assert!(items.contains(&11));
            assert!(items.contains(&12));
        }
        _ => panic!("Expected Changed delta"),
    }
}

// ---------------------------------------------------------------------------
// 5. Conservation score improves with cache warmup
// ---------------------------------------------------------------------------

#[test]
fn test_conservation_score_improves_with_warmup() {
    let metrics = TokenMetrics::new();

    // Cold start: full retrieval.
    metrics.record(Layer::Full, 100, 100);
    let cold_score = metrics.conservation_score();

    // Warm: cache hits.
    metrics.record(Layer::Cache, 0, 100);
    let warm_score_1 = metrics.conservation_score();

    metrics.record(Layer::Cache, 0, 100);
    let warm_score_2 = metrics.conservation_score();

    // Score should improve as more queries hit the cache.
    assert!(
        warm_score_1 > cold_score,
        "Score should improve after first cache hit"
    );
    assert!(warm_score_2 > warm_score_1, "Score should improve further");
}

// ---------------------------------------------------------------------------
// 6. Conservation target of 0.7
// ---------------------------------------------------------------------------

#[test]
fn test_conservation_target_07() {
    let metrics = TokenMetrics::new();
    let audit = AuditLog::new();

    // Simulate a workload: 1 cold, 3 warm (cache hits).
    metrics.record(Layer::Full, 100, 100);
    metrics.record(Layer::Cache, 0, 100);
    metrics.record(Layer::Cache, 0, 100);
    metrics.record(Layer::Cache, 0, 100);

    let report = generate_report(&metrics, &audit);

    // saved=300, used=100, potential=400 => score=0.75
    assert!(
        report.conservation_score >= 0.7,
        "Score {} should meet 0.7 target",
        report.conservation_score
    );
    assert!(report.target_met);
    assert!(report.verdict.meets_target());
}

// ---------------------------------------------------------------------------
// 7. Token budget is enforced
// ---------------------------------------------------------------------------

#[test]
fn test_token_budget_enforced() {
    let mut budget = TokenBudget::new(100);

    assert!(budget.can_afford(50));
    assert!(budget.spend(50));
    assert_eq!(budget.remaining(), 50);

    assert!(budget.can_afford(50));
    assert!(!budget.can_afford(51));

    assert!(budget.spend(50));
    assert!(budget.is_exhausted());
    assert_eq!(budget.remaining(), 0);

    // Overspending is tracked but returns false.
    assert!(!budget.spend(1));
    assert!(budget.used_tokens() > budget.max_tokens());
}

// ---------------------------------------------------------------------------
// 8. Default intent is minimal
// ---------------------------------------------------------------------------

#[test]
fn test_default_intent_is_minimal() {
    let intent = ExtractionIntent::default();
    assert_eq!(intent, ExtractionIntent::IdsOnly);
    assert!(intent.is_minimal());
    assert!(!intent.is_full());

    // IdsOnly cost should be much less than Full.
    assert!(intent.estimated_tokens() < ExtractionIntent::Full.estimated_tokens());
}

// ---------------------------------------------------------------------------
// 9. Cache invalidation on mutation
// ---------------------------------------------------------------------------

#[test]
fn test_cache_invalidation_on_mutation() {
    let cache = LruCache::new(100, Duration::from_secs(60));
    let invalidator = CacheInvalidator::new();

    // Set up: "pattern_list" depends on "pattern:1".
    invalidator.add_dependency("pattern:1".to_string(), "pattern_list".to_string());

    // Populate both cache entries.
    cache.insert("pattern:1".to_string(), "data-v1".to_string());
    cache.insert("pattern_list".to_string(), "list-v1".to_string());

    assert!(cache.contains(&"pattern:1".to_string()));
    assert!(cache.contains(&"pattern_list".to_string()));

    // Simulate mutation: invalidate "pattern:1" and cascade.
    let cascade = invalidator.cascade(&"pattern:1".to_string());
    for key in &cascade.invalidated {
        cache.invalidate(key);
    }

    // Both should be gone.
    assert!(!cache.contains(&"pattern:1".to_string()));
    assert!(!cache.contains(&"pattern_list".to_string()));
}

// ---------------------------------------------------------------------------
// 10. End-to-end conservation flow
// ---------------------------------------------------------------------------

#[test]
fn test_end_to_end_conservation_flow() {
    let cache = LruCache::new(100, Duration::from_secs(60));
    let metrics = TokenMetrics::new();
    let audit = AuditLog::new();
    let mut budget = TokenBudget::new(500);

    // Step 1: Cold query — full retrieval.
    let full_cost = 100u64;
    assert_eq!(cache.get(&"query_1"), None);
    metrics.record(Layer::Full, full_cost, full_cost);
    budget.spend(full_cost);
    audit.record(AuditEntry::new(
        "pattern_search",
        Layer::Full,
        full_cost,
        0,
        false,
        ExtractionIntent::Full,
        1000,
        100,
    ));
    cache.insert("query_1", "result_1");

    // Step 2: Warm query — cache hit.
    assert!(cache.contains(&"query_1"));
    metrics.record(Layer::Cache, 0, full_cost);
    audit.record(AuditEntry::new(
        "pattern_search",
        Layer::Cache,
        0,
        full_cost,
        true,
        ExtractionIntent::IdsOnly,
        1000,
        10,
    ));

    // Step 3: Scoped query — reduced cost.
    let scoped_cost = 10u64;
    metrics.record(Layer::Scoped, scoped_cost, full_cost);
    budget.spend(scoped_cost);
    audit.record(AuditEntry::new(
        "pattern_list",
        Layer::Scoped,
        scoped_cost,
        full_cost - scoped_cost,
        false,
        ExtractionIntent::Summary,
        1000,
        50,
    ));

    // Step 4: Another cache hit.
    metrics.record(Layer::Cache, 0, full_cost);
    audit.record(AuditEntry::new(
        "pattern_get",
        Layer::Cache,
        0,
        full_cost,
        true,
        ExtractionIntent::IdsOnly,
        100,
        5,
    ));

    // Verify conservation.
    let report = generate_report(&metrics, &audit);

    // Total used: 100 + 0 + 10 + 0 = 110
    // Total saved: 0 + 100 + 90 + 100 = 290
    // Potential: 400
    // Score: 290/400 = 0.725
    assert!(
        report.conservation_score >= 0.7,
        "End-to-end score {} should meet target",
        report.conservation_score
    );
    assert!(report.target_met);
    assert_eq!(report.query_count, 4);
    assert!((report.cache_hit_rate - 0.5).abs() < f64::EPSILON);

    // Budget should still have room.
    assert!(!budget.is_exhausted());
    assert_eq!(budget.remaining(), 390);
}
