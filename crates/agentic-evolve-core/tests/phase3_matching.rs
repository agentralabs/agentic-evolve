//! Phase 3: Matching tests — Signature, Context, Semantic, Fuzzy, Composite matchers.

use agentic_evolve_core::matching::composite::{CompositeMatcher, MatchWeights};
use agentic_evolve_core::matching::context::ContextMatcher;
use agentic_evolve_core::matching::fuzzy::FuzzyMatcher;
use agentic_evolve_core::matching::semantic::SemanticMatcher;
use agentic_evolve_core::matching::signature::SignatureMatcher;
use agentic_evolve_core::types::match_result::MatchContext;
use agentic_evolve_core::types::pattern::{
    FunctionSignature, Language, ParamSignature, Pattern, Visibility,
};

fn make_sig(name: &str, lang: Language) -> FunctionSignature {
    FunctionSignature::new(name, lang)
}

fn make_sig_with_params(name: &str, lang: Language, params: Vec<(&str, &str)>) -> FunctionSignature {
    let mut sig = FunctionSignature::new(name, lang);
    sig.params = params
        .into_iter()
        .map(|(n, t)| ParamSignature {
            name: n.to_string(),
            param_type: t.to_string(),
            is_optional: false,
        })
        .collect();
    sig
}

fn make_pattern(name: &str, domain: &str, lang: Language) -> Pattern {
    let sig = FunctionSignature::new(name, lang.clone());
    Pattern::new(name, domain, lang, sig, "fn body() {}", vec![], 0.8)
}

fn make_pattern_with_sig(name: &str, domain: &str, sig: FunctionSignature) -> Pattern {
    Pattern::new(
        name,
        domain,
        sig.language.clone(),
        sig,
        "fn body() {}",
        vec![],
        0.8,
    )
}

fn make_pattern_with_template(name: &str, domain: &str, template: &str) -> Pattern {
    let sig = FunctionSignature::new(name, Language::Rust);
    Pattern::new(name, domain, Language::Rust, sig, template, vec![], 0.8)
}

// ===========================================================================
// SignatureMatcher
// ===========================================================================

#[test]
fn sig_matcher_exact_match_scores_high() {
    let matcher = SignatureMatcher::new();
    let sig = make_sig("parse_json", Language::Rust);
    let p = make_pattern("parse_json", "data", Language::Rust);
    let score = matcher.score_match(&p, &sig);
    assert!(score > 0.7, "Exact match should score high, got {score}");
}

#[test]
fn sig_matcher_different_language_scores_lower() {
    let matcher = SignatureMatcher::new();
    let sig = make_sig("parse_json", Language::Python);
    let p = make_pattern("parse_json", "data", Language::Rust);
    let same_lang_p = make_pattern("parse_json", "data", Language::Python);
    let score_diff = matcher.score_match(&p, &sig);
    let score_same = matcher.score_match(&same_lang_p, &sig);
    assert!(
        score_same > score_diff,
        "Same language should score higher: same={score_same} diff={score_diff}"
    );
}

#[test]
fn sig_matcher_param_count_affects_score() {
    let matcher = SignatureMatcher::new();
    let sig = make_sig_with_params("f", Language::Rust, vec![("a", "i32"), ("b", "i32")]);
    let p_match = make_pattern_with_sig(
        "f",
        "x",
        make_sig_with_params("f", Language::Rust, vec![("a", "i32"), ("b", "i32")]),
    );
    let p_diff = make_pattern_with_sig(
        "f",
        "x",
        make_sig_with_params("f", Language::Rust, vec![("a", "i32")]),
    );
    let score_match = matcher.score_match(&p_match, &sig);
    let score_diff = matcher.score_match(&p_diff, &sig);
    assert!(
        score_match > score_diff,
        "Matching param count should score higher: match={score_match} diff={score_diff}"
    );
}

#[test]
fn sig_matcher_return_type_match() {
    let matcher = SignatureMatcher::new();
    let mut sig = make_sig("getter", Language::Rust);
    sig.return_type = Some("String".to_string());
    let mut p_sig = make_sig("getter", Language::Rust);
    p_sig.return_type = Some("String".to_string());
    let p = make_pattern_with_sig("getter", "web", p_sig);

    let mut p_sig2 = make_sig("getter", Language::Rust);
    p_sig2.return_type = Some("i32".to_string());
    let p_diff = make_pattern_with_sig("getter", "web", p_sig2);

    let score_same = matcher.score_match(&p, &sig);
    let score_diff = matcher.score_match(&p_diff, &sig);
    assert!(score_same > score_diff);
}

#[test]
fn sig_matcher_async_match() {
    let matcher = SignatureMatcher::new();
    let mut sig = make_sig("fetch_data", Language::Rust);
    sig.is_async = true;
    let mut p_sig = make_sig("fetch_data", Language::Rust);
    p_sig.is_async = true;
    let p_async = make_pattern_with_sig("fetch_data", "net", p_sig);

    let p_sync = make_pattern("fetch_data", "net", Language::Rust);

    let score_async = matcher.score_match(&p_async, &sig);
    let score_sync = matcher.score_match(&p_sync, &sig);
    assert!(score_async > score_sync);
}

#[test]
fn sig_matcher_find_matches_sorted() {
    let matcher = SignatureMatcher::new();
    let sig = make_sig("parse_json", Language::Rust);
    let ctx = MatchContext::new();
    let p1 = make_pattern("parse_json", "data", Language::Rust);
    let p2 = make_pattern("build_html", "web", Language::Rust);
    let patterns: Vec<&Pattern> = vec![&p1, &p2];
    let results = matcher.find_matches(&sig, &patterns, &ctx, 10).unwrap();
    assert!(!results.is_empty());
    // First result should be the better match
    if results.len() > 1 {
        assert!(results[0].score.combined >= results[1].score.combined);
    }
}

#[test]
fn sig_matcher_respects_limit() {
    let matcher = SignatureMatcher::new();
    let sig = make_sig("test", Language::Rust);
    let ctx = MatchContext::new();
    let p1 = make_pattern("test_a", "x", Language::Rust);
    let p2 = make_pattern("test_b", "y", Language::Rust);
    let p3 = make_pattern("test_c", "z", Language::Rust);
    let patterns: Vec<&Pattern> = vec![&p1, &p2, &p3];
    let results = matcher.find_matches(&sig, &patterns, &ctx, 1).unwrap();
    assert!(results.len() <= 1);
}

#[test]
fn sig_matcher_param_type_similarity() {
    let matcher = SignatureMatcher::new();
    let sig = make_sig_with_params("f", Language::Rust, vec![("a", "String")]);
    let p_exact = make_pattern_with_sig(
        "f",
        "x",
        make_sig_with_params("f", Language::Rust, vec![("a", "String")]),
    );
    let p_diff = make_pattern_with_sig(
        "f",
        "x",
        make_sig_with_params("f", Language::Rust, vec![("a", "i32")]),
    );
    let score_exact = matcher.score_match(&p_exact, &sig);
    let score_diff = matcher.score_match(&p_diff, &sig);
    assert!(score_exact > score_diff);
}

// ===========================================================================
// ContextMatcher
// ===========================================================================

#[test]
fn ctx_matcher_domain_match() {
    let matcher = ContextMatcher::new();
    let ctx = MatchContext::new().with_domain("web");
    let p = make_pattern("handler", "web", Language::Rust);
    let score = matcher.score_context(&p, &ctx);
    assert!(score > 0.5, "Domain match should boost score, got {score}");
}

#[test]
fn ctx_matcher_domain_mismatch() {
    let matcher = ContextMatcher::new();
    let ctx = MatchContext::new().with_domain("cli");
    let p = make_pattern("handler", "web", Language::Rust);
    let score_mismatch = matcher.score_context(&p, &ctx);
    let ctx_match = MatchContext::new().with_domain("web");
    let score_match = matcher.score_context(&p, &ctx_match);
    assert!(score_match > score_mismatch);
}

#[test]
fn ctx_matcher_import_overlap() {
    let matcher = ContextMatcher::new();
    let mut ctx = MatchContext::new();
    ctx.imports = vec!["serde".to_string(), "tokio".to_string()];
    let p = make_pattern_with_template("handler", "web", "use serde;\nfn handler() {}");
    let score = matcher.score_context(&p, &ctx);
    assert!(score > 0.0, "Import overlap should give some score, got {score}");
}

#[test]
fn ctx_matcher_surrounding_code() {
    let matcher = ContextMatcher::new();
    let ctx = MatchContext::new()
        .with_surrounding_code("fn parse() { let data = serde_json::from_str(input); }");
    let p = make_pattern_with_template("parser", "data", "let data = serde_json::from_str(input);");
    let score = matcher.score_context(&p, &ctx);
    assert!(score > 0.0);
}

#[test]
fn ctx_matcher_no_context_gives_default() {
    let matcher = ContextMatcher::new();
    let ctx = MatchContext::new(); // no domain, no imports, no surrounding code
    let p = make_pattern("any", "any", Language::Rust);
    let score = matcher.score_context(&p, &ctx);
    assert!((score - 0.3).abs() < 1e-9, "Default context score should be 0.3, got {score}");
}

#[test]
fn ctx_matcher_find_matches_returns_results() {
    let matcher = ContextMatcher::new();
    let sig = make_sig("handler", Language::Rust);
    let ctx = MatchContext::new().with_domain("web");
    let p = make_pattern("handler", "web", Language::Rust);
    let patterns: Vec<&Pattern> = vec![&p];
    let results = matcher.find_matches(&sig, &patterns, &ctx, 10).unwrap();
    assert!(!results.is_empty());
}

#[test]
fn ctx_matcher_partial_domain_match() {
    let matcher = ContextMatcher::new();
    let ctx = MatchContext::new().with_domain("web");
    let p = make_pattern("handler", "web-api", Language::Rust);
    let score = matcher.score_context(&p, &ctx);
    assert!(score > 0.0, "Partial domain match should give score");
}

// ===========================================================================
// SemanticMatcher
// ===========================================================================

#[test]
fn sem_matcher_exact_token_match() {
    let matcher = SemanticMatcher::new();
    let sig = make_sig("parse_json", Language::Rust);
    let p = make_pattern("parse_json", "data", Language::Rust);
    let ctx = MatchContext::new();
    let results = matcher.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    assert!(!results.is_empty());
    assert!(results[0].score.combined > 0.5);
}

#[test]
fn sem_matcher_synonym_match() {
    let matcher = SemanticMatcher::new();
    // "get" and "fetch" are synonyms in the semantic matcher
    let sig = make_sig("get_data", Language::Rust);
    let p = make_pattern("fetch_data", "data", Language::Rust);
    let ctx = MatchContext::new();
    let results = matcher.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    assert!(!results.is_empty(), "Synonyms should match");
    assert!(results[0].score.combined > 0.0);
}

#[test]
fn sem_matcher_camel_case_tokenization() {
    let matcher = SemanticMatcher::new();
    let sig = make_sig("getData", Language::TypeScript);
    let p = make_pattern("fetchData", "api", Language::TypeScript);
    let ctx = MatchContext::new();
    let results = matcher.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    // Both "Data" token should match, plus get/fetch are synonyms
    assert!(!results.is_empty());
}

#[test]
fn sem_matcher_snake_case_tokenization() {
    let matcher = SemanticMatcher::new();
    let sig = make_sig("get_user_data", Language::Rust);
    let p = make_pattern("fetch_user_data", "api", Language::Rust);
    let ctx = MatchContext::new();
    let results = matcher.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    assert!(!results.is_empty());
    // "user" and "data" tokens match, plus get/fetch are synonyms
    assert!(results[0].score.combined > 0.3);
}

#[test]
fn sem_matcher_no_match_different_tokens() {
    let matcher = SemanticMatcher::new();
    let sig = make_sig("render_html", Language::Rust);
    let p = make_pattern("parse_json", "data", Language::Rust);
    let ctx = MatchContext::new();
    let results = matcher.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    // May or may not have results, but score should be low
    if !results.is_empty() {
        assert!(results[0].score.combined < 0.5);
    }
}

#[test]
fn sem_matcher_empty_patterns() {
    let matcher = SemanticMatcher::new();
    let sig = make_sig("anything", Language::Rust);
    let ctx = MatchContext::new();
    let results = matcher.find_matches(&sig, &[], &ctx, 10).unwrap();
    assert!(results.is_empty());
}

#[test]
fn sem_matcher_respects_limit() {
    let matcher = SemanticMatcher::new();
    let sig = make_sig("get_data", Language::Rust);
    let ctx = MatchContext::new();
    let p1 = make_pattern("get_info", "a", Language::Rust);
    let p2 = make_pattern("fetch_data", "b", Language::Rust);
    let p3 = make_pattern("load_data", "c", Language::Rust);
    let results = matcher.find_matches(&sig, &[&p1, &p2, &p3], &ctx, 1).unwrap();
    assert!(results.len() <= 1);
}

#[test]
fn sem_matcher_delete_remove_synonyms() {
    let matcher = SemanticMatcher::new();
    let sig = make_sig("delete_user", Language::Rust);
    let p = make_pattern("remove_user", "auth", Language::Rust);
    let ctx = MatchContext::new();
    let results = matcher.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    assert!(!results.is_empty());
}

#[test]
fn sem_matcher_create_build_synonyms() {
    let matcher = SemanticMatcher::new();
    let sig = make_sig("create_widget", Language::Rust);
    let p = make_pattern("build_widget", "ui", Language::Rust);
    let ctx = MatchContext::new();
    let results = matcher.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    assert!(!results.is_empty());
}

// ===========================================================================
// FuzzyMatcher
// ===========================================================================

#[test]
fn fuzzy_exact_match_above_threshold() {
    let matcher = FuzzyMatcher::new(0.3);
    let sig = make_sig("parse_json", Language::Rust);
    let p = make_pattern("parse_json", "data", Language::Rust);
    let ctx = MatchContext::new();
    let results = matcher.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    assert!(!results.is_empty());
    assert!(results[0].score.combined >= 0.3);
}

#[test]
fn fuzzy_near_match() {
    let matcher = FuzzyMatcher::new(0.3);
    let sig = make_sig("parse_json", Language::Rust);
    let p = make_pattern("parse_jsn", "data", Language::Rust);
    let ctx = MatchContext::new();
    let results = matcher.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    assert!(!results.is_empty(), "Near match should be found");
}

#[test]
fn fuzzy_below_threshold_filtered() {
    let matcher = FuzzyMatcher::new(0.99);
    let sig = make_sig("parse_json", Language::Rust);
    let p = make_pattern("completely_different", "data", Language::Rust);
    let ctx = MatchContext::new();
    let results = matcher.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    assert!(results.is_empty(), "Very different names with high threshold should filter");
}

#[test]
fn fuzzy_default_threshold() {
    let matcher = FuzzyMatcher::default();
    let sig = make_sig("test", Language::Rust);
    let p = make_pattern("test", "x", Language::Rust);
    let ctx = MatchContext::new();
    let results = matcher.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    assert!(!results.is_empty());
}

#[test]
fn fuzzy_respects_limit() {
    let matcher = FuzzyMatcher::new(0.1);
    let sig = make_sig("test", Language::Rust);
    let ctx = MatchContext::new();
    let p1 = make_pattern("test_a", "x", Language::Rust);
    let p2 = make_pattern("test_b", "x", Language::Rust);
    let p3 = make_pattern("test_c", "x", Language::Rust);
    let results = matcher.find_matches(&sig, &[&p1, &p2, &p3], &ctx, 1).unwrap();
    assert!(results.len() <= 1);
}

#[test]
fn fuzzy_language_affects_score() {
    let matcher = FuzzyMatcher::new(0.1);
    let sig = make_sig("handler", Language::Rust);
    let p_same = make_pattern("handler", "x", Language::Rust);
    let p_diff = make_pattern("handler", "x", Language::Python);
    let ctx = MatchContext::new();
    let r_same = matcher.find_matches(&sig, &[&p_same], &ctx, 10).unwrap();
    let r_diff = matcher.find_matches(&sig, &[&p_diff], &ctx, 10).unwrap();
    assert!(!r_same.is_empty() && !r_diff.is_empty());
    assert!(r_same[0].score.combined >= r_diff[0].score.combined);
}

#[test]
fn fuzzy_param_type_similarity() {
    let matcher = FuzzyMatcher::new(0.1);
    let sig = make_sig_with_params("f", Language::Rust, vec![("a", "String")]);
    let p = make_pattern_with_sig(
        "f",
        "x",
        make_sig_with_params("f", Language::Rust, vec![("a", "String")]),
    );
    let ctx = MatchContext::new();
    let results = matcher.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    assert!(!results.is_empty());
}

// ===========================================================================
// CompositeMatcher
// ===========================================================================

#[test]
fn composite_combines_scores() {
    let matcher = CompositeMatcher::new();
    let sig = make_sig("parse_json", Language::Rust);
    let ctx = MatchContext::new().with_domain("data");
    let p = make_pattern("parse_json", "data", Language::Rust);
    let results = matcher.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    assert!(!results.is_empty());
    // Combined score should incorporate multiple matchers
    assert!(results[0].score.combined > 0.1);
}

#[test]
fn composite_empty_patterns() {
    let matcher = CompositeMatcher::new();
    let sig = make_sig("anything", Language::Rust);
    let ctx = MatchContext::new();
    let results = matcher.find_matches(&sig, &[], &ctx, 10).unwrap();
    assert!(results.is_empty());
}

#[test]
fn composite_respects_limit() {
    let matcher = CompositeMatcher::new();
    let sig = make_sig("test", Language::Rust);
    let ctx = MatchContext::new();
    let p1 = make_pattern("test_a", "x", Language::Rust);
    let p2 = make_pattern("test_b", "x", Language::Rust);
    let p3 = make_pattern("test_c", "x", Language::Rust);
    let results = matcher.find_matches(&sig, &[&p1, &p2, &p3], &ctx, 2).unwrap();
    assert!(results.len() <= 2);
}

#[test]
fn composite_custom_weights() {
    let weights = MatchWeights {
        signature: 0.9,
        context: 0.0,
        semantic: 0.1,
        fuzzy: 0.0,
    };
    let matcher = CompositeMatcher::with_weights(weights);
    let sig = make_sig("exact_match", Language::Rust);
    let ctx = MatchContext::new();
    let p = make_pattern("exact_match", "x", Language::Rust);
    let results = matcher.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    assert!(!results.is_empty());
}

#[test]
fn composite_default_is_same_as_new() {
    let a = CompositeMatcher::new();
    let b = CompositeMatcher::default();
    let sig = make_sig("test", Language::Rust);
    let ctx = MatchContext::new();
    let p = make_pattern("test", "x", Language::Rust);
    let ra = a.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    let rb = b.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    assert_eq!(ra.len(), rb.len());
}

#[test]
fn composite_sorted_by_combined_score() {
    let matcher = CompositeMatcher::new();
    let sig = make_sig("parse_data", Language::Rust);
    let ctx = MatchContext::new().with_domain("data");
    let p_good = make_pattern("parse_data", "data", Language::Rust);
    let p_bad = make_pattern("render_html", "web", Language::Python);
    let results = matcher
        .find_matches(&sig, &[&p_good, &p_bad], &ctx, 10)
        .unwrap();
    if results.len() > 1 {
        assert!(results[0].score.combined >= results[1].score.combined);
    }
}

#[test]
fn composite_confidence_factor_used() {
    let matcher = CompositeMatcher::new();
    let sig = make_sig("test_fn", Language::Rust);
    let ctx = MatchContext::new();
    let mut p_high = make_pattern("test_fn", "x", Language::Rust);
    p_high.confidence = 1.0;
    let mut p_low = make_pattern("test_fn", "x", Language::Rust);
    p_low.confidence = 0.1;
    // Both match on name but different confidence
    let r_high = matcher.find_matches(&sig, &[&p_high], &ctx, 10).unwrap();
    let r_low = matcher.find_matches(&sig, &[&p_low], &ctx, 10).unwrap();
    if !r_high.is_empty() && !r_low.is_empty() {
        assert!(r_high[0].score.combined >= r_low[0].score.combined);
    }
}

#[test]
fn composite_with_context_domain() {
    let matcher = CompositeMatcher::new();
    let mut sig = FunctionSignature::new("handler", Language::Rust);
    sig.is_async = true;
    let ctx = MatchContext::new().with_domain("web");
    let mut p_sig = FunctionSignature::new("handler", Language::Rust);
    p_sig.is_async = true;
    let p = make_pattern_with_sig("handler", "web", p_sig);
    let results = matcher.find_matches(&sig, &[&p], &ctx, 10).unwrap();
    assert!(!results.is_empty());
}
