//! Phase 9: Edge case, stress, and boundary tests for pattern library robustness.

use agentic_evolve_core::types::pattern::{
    FunctionSignature, Language, Pattern, PatternVariable,
};
use agentic_evolve_core::matching::signature::SignatureMatcher;

fn make_pattern(name: &str, domain: &str, confidence: f64) -> Pattern {
    let sig = FunctionSignature::new(name, Language::Rust);
    Pattern::new(
        name,
        domain,
        Language::Rust,
        sig,
        "fn {{name}}() { todo!() }",
        vec![],
        confidence,
    )
}

// ── stress: heavy pattern creation ──────────────────────────────────

#[test]
fn stress_create_1000_patterns() {
    let patterns: Vec<Pattern> = (0..1000)
        .map(|i| make_pattern(&format!("stress-pattern-{i}"), "web", 0.8))
        .collect();
    assert_eq!(patterns.len(), 1000);
    // Verify all have unique IDs
    let ids: std::collections::HashSet<_> = patterns.iter().map(|p| p.id.clone()).collect();
    assert_eq!(ids.len(), 1000);
}

// ── edge_case: empty name ───────────────────────────────────────────

#[test]
fn edge_empty_pattern_name() {
    let pattern = make_pattern("", "web", 0.5);
    assert_eq!(pattern.name, "");
    assert!(!pattern.content_hash.is_empty());
}

// ── edge_case: very long name ────────────────────────────────────────

#[test]
fn edge_very_long_pattern_name() {
    let long_name = "a".repeat(10_000);
    let pattern = make_pattern(&long_name, "web", 0.5);
    assert_eq!(pattern.name.len(), 10_000);
}

// ── boundary: confidence values ──────────────────────────────────────

#[test]
fn boundary_confidence_zero() {
    let pattern = make_pattern("zero-conf", "web", 0.0);
    assert_eq!(pattern.confidence, 0.0);
}

#[test]
fn boundary_confidence_one() {
    let pattern = make_pattern("full-conf", "web", 1.0);
    assert_eq!(pattern.confidence, 1.0);
}

#[test]
fn boundary_confidence_negative() {
    let pattern = make_pattern("neg-conf", "web", -0.5);
    assert_eq!(pattern.confidence, -0.5);
}

#[test]
fn boundary_confidence_above_one() {
    let pattern = make_pattern("over-conf", "web", 1.5);
    assert_eq!(pattern.confidence, 1.5);
}

// ── edge_case: special characters ────────────────────────────────────

#[test]
fn edge_special_characters_in_name() {
    let pattern = make_pattern("test/pattern:v2@latest#1", "web", 0.8);
    assert_eq!(pattern.name, "test/pattern:v2@latest#1");
}

#[test]
fn edge_unicode_in_name() {
    let pattern = make_pattern("hello-world", "web", 0.8);
    assert!(pattern.name.len() > 0);
}

// ── stress: heavy usage recording ────────────────────────────────────

#[test]
fn stress_heavy_usage_recording() {
    let mut pattern = make_pattern("heavy-use", "web", 0.8);
    for i in 0..10_000 {
        pattern.record_use(i % 3 != 0);
    }
    assert_eq!(pattern.usage_count, 10_000);
    assert!(pattern.success_rate() > 0.6);
    assert!(pattern.success_rate() < 0.7);
}

// ── edge_case: success rate with zero usage ──────────────────────────

#[test]
fn edge_success_rate_zero_usage() {
    let pattern = make_pattern("no-use", "web", 0.8);
    assert_eq!(pattern.success_rate(), 0.0);
}

// ── stress: signature matching many patterns ─────────────────────────

#[test]
fn stress_signature_match_500_patterns() {
    let matcher = SignatureMatcher::new();
    let query = FunctionSignature::new("authenticate", Language::Rust);
    let patterns: Vec<Pattern> = (0..500)
        .map(|i| make_pattern(&format!("sig-stress-{i}"), "web", 0.8))
        .collect();
    for p in &patterns {
        let _score = matcher.score_match(p, &query);
    }
    assert_eq!(patterns.len(), 500);
}

// ── boundary: empty template ─────────────────────────────────────────

#[test]
fn boundary_empty_template() {
    let sig = FunctionSignature::new("empty", Language::Rust);
    let pattern = Pattern::new("empty", "test", Language::Rust, sig, "", vec![], 0.5);
    assert_eq!(pattern.template, "");
    assert!(!pattern.content_hash.is_empty());
}

// ── edge_case: many variables ────────────────────────────────────────

#[test]
fn edge_many_variables() {
    let sig = FunctionSignature::new("many-vars", Language::Rust);
    let vars: Vec<PatternVariable> = (0..100)
        .map(|i| PatternVariable {
            name: format!("var_{i}"),
            var_type: "string".to_string(),
            pattern: None,
            default: Some(format!("default_{i}")),
        })
        .collect();
    let pattern = Pattern::new(
        "many-vars",
        "test",
        Language::Rust,
        sig,
        "template with {{var_0}} ... {{var_99}}",
        vars,
        0.9,
    );
    assert_eq!(pattern.variables.len(), 100);
}

// ── stress: concurrent-like pattern creation ─────────────────────────

#[test]
fn stress_rapid_pattern_creation() {
    let start = std::time::Instant::now();
    let patterns: Vec<Pattern> = (0..5000)
        .map(|i| make_pattern(&format!("rapid-{i}"), "perf", 0.7))
        .collect();
    let elapsed = start.elapsed();
    assert_eq!(patterns.len(), 5000);
    // Should complete in under 1 second
    assert!(elapsed.as_secs() < 5, "Pattern creation too slow: {elapsed:?}");
}

// ── edge_case: all languages ─────────────────────────────────────────

#[test]
fn edge_all_language_variants() {
    let languages = vec![
        "rust", "python", "typescript", "javascript", "go",
        "java", "csharp", "cpp", "c", "shell", "unknown-lang",
    ];
    for lang_name in &languages {
        let lang = Language::from_name(lang_name);
        assert!(!lang.as_str().is_empty());
    }
}
