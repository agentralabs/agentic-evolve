//! Phase 6: Collective learning tests — Usage, Success, Decay, Promotion.

use agentic_evolve_core::collective::decay::{DecayConfig, DecayManager};
use agentic_evolve_core::collective::promotion::{PromotionConfig, PromotionDecision, PromotionEngine};
use agentic_evolve_core::collective::success::SuccessTracker;
use agentic_evolve_core::collective::usage::UsageTracker;
use agentic_evolve_core::types::pattern::{FunctionSignature, Language, Pattern};

fn make_pattern(name: &str, confidence: f64) -> Pattern {
    let sig = FunctionSignature::new(name, Language::Rust);
    Pattern::new(name, "test", Language::Rust, sig, "fn test() {}", vec![], confidence)
}

// ===========================================================================
// UsageTracker
// ===========================================================================

#[test]
fn usage_record_use() {
    let mut tracker = UsageTracker::new();
    tracker.record_use("p1", "web", true);
    let record = tracker.get_usage("p1").unwrap();
    assert_eq!(record.total_uses, 1);
    assert_eq!(record.successful_uses, 1);
    assert_eq!(record.failed_uses, 0);
}

#[test]
fn usage_record_failure() {
    let mut tracker = UsageTracker::new();
    tracker.record_use("p1", "web", false);
    let record = tracker.get_usage("p1").unwrap();
    assert_eq!(record.total_uses, 1);
    assert_eq!(record.successful_uses, 0);
    assert_eq!(record.failed_uses, 1);
}

#[test]
fn usage_get_usage_none() {
    let tracker = UsageTracker::new();
    assert!(tracker.get_usage("nonexistent").is_none());
}

#[test]
fn usage_success_rate() {
    let mut tracker = UsageTracker::new();
    tracker.record_use("p1", "web", true);
    tracker.record_use("p1", "web", true);
    tracker.record_use("p1", "web", false);
    let rate = tracker.success_rate("p1");
    assert!((rate - 2.0 / 3.0).abs() < f64::EPSILON);
}

#[test]
fn usage_success_rate_unknown_pattern() {
    let tracker = UsageTracker::new();
    assert_eq!(tracker.success_rate("unknown"), 0.0);
}

#[test]
fn usage_most_used() {
    let mut tracker = UsageTracker::new();
    tracker.record_use("popular", "web", true);
    tracker.record_use("popular", "web", true);
    tracker.record_use("popular", "web", true);
    tracker.record_use("rare", "web", true);
    let most = tracker.most_used(1);
    assert_eq!(most.len(), 1);
    assert_eq!(most[0].0, "popular");
}

#[test]
fn usage_least_used() {
    let mut tracker = UsageTracker::new();
    tracker.record_use("popular", "web", true);
    tracker.record_use("popular", "web", true);
    tracker.record_use("rare", "web", true);
    let least = tracker.least_used(1);
    assert_eq!(least.len(), 1);
    assert_eq!(least[0].0, "rare");
}

#[test]
fn usage_total_patterns_tracked() {
    let mut tracker = UsageTracker::new();
    assert_eq!(tracker.total_patterns_tracked(), 0);
    tracker.record_use("p1", "web", true);
    tracker.record_use("p2", "cli", true);
    assert_eq!(tracker.total_patterns_tracked(), 2);
}

#[test]
fn usage_clear() {
    let mut tracker = UsageTracker::new();
    tracker.record_use("p1", "web", true);
    tracker.clear();
    assert_eq!(tracker.total_patterns_tracked(), 0);
}

#[test]
fn usage_domain_tracking() {
    let mut tracker = UsageTracker::new();
    tracker.record_use("p1", "web", true);
    tracker.record_use("p1", "web", true);
    tracker.record_use("p1", "cli", true);
    let record = tracker.get_usage("p1").unwrap();
    assert_eq!(record.domains.get("web"), Some(&2));
    assert_eq!(record.domains.get("cli"), Some(&1));
}

// ===========================================================================
// SuccessTracker
// ===========================================================================

#[test]
fn success_record_success() {
    let mut tracker = SuccessTracker::new();
    tracker.record("p1", true);
    let record = tracker.get("p1").unwrap();
    assert_eq!(record.successes, 1);
    assert_eq!(record.failures, 0);
    assert_eq!(record.streak, 1);
}

#[test]
fn success_record_failure() {
    let mut tracker = SuccessTracker::new();
    tracker.record("p1", false);
    let record = tracker.get("p1").unwrap();
    assert_eq!(record.successes, 0);
    assert_eq!(record.failures, 1);
    assert_eq!(record.streak, -1);
}

#[test]
fn success_rate() {
    let mut tracker = SuccessTracker::new();
    tracker.record("p1", true);
    tracker.record("p1", true);
    tracker.record("p1", false);
    let rate = tracker.success_rate("p1");
    assert!((rate - 2.0 / 3.0).abs() < f64::EPSILON);
}

#[test]
fn success_rate_unknown() {
    let tracker = SuccessTracker::new();
    assert_eq!(tracker.success_rate("unknown"), 0.0);
}

#[test]
fn success_streaks() {
    let mut tracker = SuccessTracker::new();
    tracker.record("p1", true);
    tracker.record("p1", true);
    tracker.record("p1", true);
    let record = tracker.get("p1").unwrap();
    assert_eq!(record.streak, 3);
    assert_eq!(record.best_streak, 3);

    // Break streak
    tracker.record("p1", false);
    let record = tracker.get("p1").unwrap();
    assert_eq!(record.streak, -1);
    assert_eq!(record.best_streak, 3);
}

#[test]
fn success_top_performers() {
    let mut tracker = SuccessTracker::new();
    // Need 3+ attempts to qualify
    for _ in 0..5 {
        tracker.record("star", true);
    }
    for _ in 0..3 {
        tracker.record("average", true);
    }
    tracker.record("average", false);
    let top = tracker.top_performers(10);
    assert!(!top.is_empty());
    assert_eq!(top[0].0, "star");
}

#[test]
fn success_underperformers() {
    let mut tracker = SuccessTracker::new();
    for _ in 0..5 {
        tracker.record("good", true);
    }
    tracker.record("bad", true);
    tracker.record("bad", false);
    tracker.record("bad", false);
    tracker.record("bad", false);
    let under = tracker.underperformers(0.5);
    assert!(!under.is_empty());
    assert!(under.iter().any(|(id, _)| *id == "bad"));
}

#[test]
fn success_recent_success_rate() {
    let mut tracker = SuccessTracker::new();
    // Record lots of successes then some failures
    for _ in 0..10 {
        tracker.record("p1", true);
    }
    for _ in 0..5 {
        tracker.record("p1", false);
    }
    let record = tracker.get("p1").unwrap();
    let recent = record.recent_success_rate(5);
    assert!((recent - 0.0).abs() < f64::EPSILON, "Last 5 were failures, rate should be 0.0");
}

#[test]
fn success_total_tracked() {
    let mut tracker = SuccessTracker::new();
    tracker.record("a", true);
    tracker.record("b", false);
    assert_eq!(tracker.total_tracked(), 2);
}

#[test]
fn success_clear() {
    let mut tracker = SuccessTracker::new();
    tracker.record("a", true);
    tracker.clear();
    assert_eq!(tracker.total_tracked(), 0);
}

// ===========================================================================
// DecayManager
// ===========================================================================

#[test]
fn decay_apply_decay_reduces_confidence() {
    let config = DecayConfig {
        half_life_days: 30.0,
        min_confidence: 0.1,
        usage_boost: 0.05,
        success_boost: 0.1,
    };
    let manager = DecayManager::new(config);
    let mut p = make_pattern("old", 0.9);
    // Simulate old pattern (set last_used to 60 days ago)
    p.last_used = chrono::Utc::now().timestamp() - 60 * 86400;
    let new_conf = manager.apply_decay(&mut p);
    assert!(
        new_conf < 0.9,
        "Decay should reduce confidence from 0.9, got {new_conf}"
    );
}

#[test]
fn decay_recently_used_no_decay() {
    let manager = DecayManager::default();
    let mut p = make_pattern("recent", 0.9);
    // last_used is already set to now by make_pattern
    let new_conf = manager.apply_decay(&mut p);
    assert!(
        (new_conf - 0.9).abs() < 0.01,
        "Recently used pattern should not decay much, got {new_conf}"
    );
}

#[test]
fn decay_usage_boost() {
    let manager = DecayManager::default();
    let mut p = make_pattern("boosted", 0.5);
    let new_conf = manager.apply_usage_boost(&mut p, true);
    assert!(
        new_conf > 0.5,
        "Usage boost should increase confidence, got {new_conf}"
    );
}

#[test]
fn decay_usage_boost_without_success() {
    let manager = DecayManager::default();
    let mut p = make_pattern("used", 0.5);
    let boost_success = {
        let mut q = p.clone();
        manager.apply_usage_boost(&mut q, true)
    };
    let boost_fail = manager.apply_usage_boost(&mut p, false);
    assert!(
        boost_success > boost_fail,
        "Successful use should boost more: success={boost_success} fail={boost_fail}"
    );
}

#[test]
fn decay_usage_boost_capped_at_one() {
    let manager = DecayManager::default();
    let mut p = make_pattern("near_max", 0.99);
    let new_conf = manager.apply_usage_boost(&mut p, true);
    assert!(new_conf <= 1.0, "Confidence should not exceed 1.0");
}

#[test]
fn decay_should_prune_old_unused() {
    let config = DecayConfig {
        half_life_days: 30.0,
        min_confidence: 0.1,
        ..Default::default()
    };
    let manager = DecayManager::new(config);
    let mut p = make_pattern("old_unused", 0.05);
    p.last_used = chrono::Utc::now().timestamp() - 100 * 86400; // 100 days ago
    p.usage_count = 1;
    assert!(manager.should_prune(&p));
}

#[test]
fn decay_should_not_prune_popular() {
    let manager = DecayManager::default();
    let mut p = make_pattern("popular", 0.05);
    p.usage_count = 100;
    assert!(!manager.should_prune(&p), "Popular patterns should not be pruned");
}

#[test]
fn decay_report() {
    let manager = DecayManager::default();
    let p1 = make_pattern("healthy", 0.9);
    let p2 = make_pattern("decaying", 0.5);
    let p3 = make_pattern("critical", 0.2);
    let report = manager.decay_report(&[&p1, &p2, &p3]);
    assert_eq!(report.total, 3);
    assert_eq!(report.healthy, 1);
    assert_eq!(report.decaying, 1);
    assert_eq!(report.critical, 1);
}

// ===========================================================================
// PromotionEngine
// ===========================================================================

#[test]
fn promotion_evaluate_promote() {
    let engine = PromotionEngine::default();
    let mut p = make_pattern("star", 0.9);
    p.usage_count = 10;
    p.success_count = 10; // 100% success rate
    assert_eq!(engine.evaluate(&p), PromotionDecision::Promote);
}

#[test]
fn promotion_evaluate_demote() {
    let engine = PromotionEngine::default();
    let mut p = make_pattern("poor", 0.5);
    p.usage_count = 10;
    p.success_count = 1; // 10% success rate
    assert_eq!(engine.evaluate(&p), PromotionDecision::Demote);
}

#[test]
fn promotion_evaluate_maintain() {
    let engine = PromotionEngine::default();
    let mut p = make_pattern("ok", 0.5);
    p.usage_count = 10;
    p.success_count = 5; // 50% success rate
    assert_eq!(engine.evaluate(&p), PromotionDecision::Maintain);
}

#[test]
fn promotion_evaluate_prune() {
    let engine = PromotionEngine::default();
    let mut p = make_pattern("dead", 0.05);
    p.usage_count = 10;
    p.success_count = 1; // 10% success rate, confidence < 0.1
    assert_eq!(engine.evaluate(&p), PromotionDecision::Prune);
}

#[test]
fn promotion_evaluate_insufficient_uses() {
    let engine = PromotionEngine::default();
    let mut p = make_pattern("new", 0.9);
    p.usage_count = 1;
    p.success_count = 1;
    // Not enough uses for promotion
    assert_eq!(engine.evaluate(&p), PromotionDecision::Maintain);
}

#[test]
fn promotion_apply_promotion_increases_confidence() {
    let engine = PromotionEngine::default();
    let mut p = make_pattern("star", 0.85);
    p.usage_count = 10;
    p.success_count = 10;
    let decision = engine.apply_promotion(&mut p);
    assert_eq!(decision, PromotionDecision::Promote);
    assert!(p.confidence > 0.85);
}

#[test]
fn promotion_apply_demotion_decreases_confidence() {
    let engine = PromotionEngine::default();
    let mut p = make_pattern("poor", 0.5);
    p.usage_count = 10;
    p.success_count = 1;
    let decision = engine.apply_promotion(&mut p);
    assert_eq!(decision, PromotionDecision::Demote);
    assert!(p.confidence < 0.5);
}

#[test]
fn promotion_apply_prune_zeros_confidence() {
    let engine = PromotionEngine::default();
    let mut p = make_pattern("dead", 0.05);
    p.usage_count = 10;
    p.success_count = 1;
    let decision = engine.apply_promotion(&mut p);
    assert_eq!(decision, PromotionDecision::Prune);
    assert_eq!(p.confidence, 0.0);
}

#[test]
fn promotion_batch_evaluate() {
    let engine = PromotionEngine::default();
    let mut p1 = make_pattern("star", 0.9);
    p1.usage_count = 10;
    p1.success_count = 10;
    let p2 = make_pattern("new", 0.5);
    let results = engine.batch_evaluate(&[&p1, &p2]);
    assert_eq!(results.len(), 2);
}

#[test]
fn promotion_default_same_as_default_config() {
    let a = PromotionEngine::default();
    let b = PromotionEngine::new(PromotionConfig::default());
    let mut p = make_pattern("test", 0.5);
    p.usage_count = 10;
    p.success_count = 10;
    assert_eq!(a.evaluate(&p), b.evaluate(&p));
}
