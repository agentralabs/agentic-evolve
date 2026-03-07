//! Phase 5: Composition tests — Composer, GapFiller, AdapterGenerator, IntegrationWeaver.

use std::collections::HashMap;

use agentic_evolve_core::composition::adapter::AdapterGenerator;
use agentic_evolve_core::composition::composer::PatternComposer;
use agentic_evolve_core::composition::gap_filler::{GapDescription, GapFiller, GapType};
use agentic_evolve_core::composition::weaver::IntegrationWeaver;
use agentic_evolve_core::types::pattern::{FunctionSignature, Language, ParamSignature, Pattern};

fn make_pattern(name: &str, template: &str) -> Pattern {
    let sig = FunctionSignature::new(name, Language::Rust);
    Pattern::new(name, "test", Language::Rust, sig, template, vec![], 0.8)
}

fn make_pattern_with_sig(name: &str, template: &str, sig: FunctionSignature) -> Pattern {
    Pattern::new(
        name,
        "test",
        sig.language.clone(),
        sig,
        template,
        vec![],
        0.8,
    )
}

// ===========================================================================
// PatternComposer
// ===========================================================================

#[test]
fn composer_compose_single() {
    let composer = PatternComposer::new();
    let p = make_pattern("greet", "fn greet() { println!(\"hello\"); }");
    let bindings = HashMap::new();
    let result = composer.compose(&[&p], &bindings, None).unwrap();
    assert!(result.code.contains("greet"));
    assert_eq!(result.patterns_used.len(), 1);
}

#[test]
fn composer_compose_multiple() {
    let composer = PatternComposer::new();
    let p1 = make_pattern("init", "fn init() {}");
    let p2 = make_pattern("run", "fn run() {}");
    let bindings = HashMap::new();
    let result = composer.compose(&[&p1, &p2], &bindings, None).unwrap();
    assert!(result.code.contains("init"));
    assert!(result.code.contains("run"));
    assert_eq!(result.patterns_used.len(), 2);
}

#[test]
fn composer_compose_with_bindings() {
    let composer = PatternComposer::new();
    let p = make_pattern("server", "let port = {{PORT}};");
    let mut bindings = HashMap::new();
    bindings.insert("PORT".to_string(), "3000".to_string());
    let result = composer.compose(&[&p], &bindings, None).unwrap();
    assert!(result.code.contains("3000"));
    assert!(!result.code.contains("{{PORT}}"));
}

#[test]
fn composer_empty_patterns_error() {
    let composer = PatternComposer::new();
    let bindings = HashMap::new();
    let result = composer.compose(&[], &bindings, None);
    assert!(result.is_err());
}

#[test]
fn composer_custom_order() {
    let composer = PatternComposer::new();
    let p1 = make_pattern("first", "// first");
    let p2 = make_pattern("second", "// second");
    let bindings = HashMap::new();
    // Reverse order
    let result = composer
        .compose(&[&p1, &p2], &bindings, Some(&[1, 0]))
        .unwrap();
    let first_pos = result.code.find("second").unwrap_or(usize::MAX);
    let second_pos = result.code.find("first").unwrap_or(usize::MAX);
    assert!(
        first_pos < second_pos,
        "Custom order should put second before first"
    );
}

#[test]
fn composer_unbound_placeholders_in_gaps() {
    let composer = PatternComposer::new();
    let p = make_pattern("tmpl", "fn {{NAME}}({{PARAM}}: {{TYPE}}) {}");
    let bindings = HashMap::new();
    let result = composer.compose(&[&p], &bindings, None).unwrap();
    assert!(
        !result.gaps.is_empty(),
        "Unbound placeholders should appear as gaps"
    );
}

#[test]
fn composer_coverage_calculation() {
    let composer = PatternComposer::new();
    let p = make_pattern("tmpl", "let x = {{VALUE}};");
    let mut bindings = HashMap::new();
    bindings.insert("VALUE".to_string(), "42".to_string());
    let result = composer.compose(&[&p], &bindings, None).unwrap();
    assert!(
        result.coverage > 0.0,
        "Coverage should be positive when bindings applied"
    );
}

#[test]
fn composer_full_coverage() {
    let composer = PatternComposer::new();
    let p = make_pattern("simple", "fn main() {}");
    let bindings = HashMap::new();
    let result = composer.compose(&[&p], &bindings, None).unwrap();
    assert!(
        (result.coverage - 1.0).abs() < f64::EPSILON,
        "No placeholders should give 100% coverage"
    );
}

// ===========================================================================
// GapFiller
// ===========================================================================

#[test]
fn gap_filler_fill_gaps() {
    let filler = GapFiller::new();
    let code = "fn main() {\n  /* GAP: initialization */\n  run();\n}";
    let gaps = vec![GapDescription {
        index: 0,
        description: "initialization".to_string(),
        gap_type: GapType::Initialization,
        context_before: String::new(),
        context_after: String::new(),
    }];
    let result = filler.fill_gaps(code, &gaps).unwrap();
    assert!(result.contains("Initialize initialization"));
    assert!(!result.contains("/* GAP:"));
}

#[test]
fn gap_filler_identify_gaps() {
    let filler = GapFiller::new();
    let code = "/* GAP: auth */ and /* GAP: logging */";
    let gaps = filler.identify_gaps(code);
    assert_eq!(gaps.len(), 2);
    assert_eq!(gaps[0].description, "auth");
    assert_eq!(gaps[1].description, "logging");
}

#[test]
fn gap_filler_type_conversion() {
    let filler = GapFiller::new();
    let code = "/* GAP: type mismatch */";
    let gaps = vec![GapDescription {
        index: 0,
        description: "type mismatch".to_string(),
        gap_type: GapType::TypeConversion,
        context_before: String::new(),
        context_after: String::new(),
    }];
    let result = filler.fill_gaps(code, &gaps).unwrap();
    assert!(result.contains("Convert type"));
}

#[test]
fn gap_filler_error_handling() {
    let filler = GapFiller::new();
    let code = "/* GAP: error case */";
    let gaps = vec![GapDescription {
        index: 0,
        description: "error case".to_string(),
        gap_type: GapType::ErrorHandling,
        context_before: String::new(),
        context_after: String::new(),
    }];
    let result = filler.fill_gaps(code, &gaps).unwrap();
    assert!(result.contains("Error handling"));
}

#[test]
fn gap_filler_missing() {
    let filler = GapFiller::new();
    let code = "/* GAP: something missing */";
    let gaps = vec![GapDescription {
        index: 0,
        description: "something missing".to_string(),
        gap_type: GapType::Missing,
        context_before: String::new(),
        context_after: String::new(),
    }];
    let result = filler.fill_gaps(code, &gaps).unwrap();
    assert!(result.contains("TODO: Implement"));
}

#[test]
fn gap_filler_no_gaps() {
    let filler = GapFiller::new();
    let code = "fn main() {}";
    let gaps = filler.identify_gaps(code);
    assert!(gaps.is_empty());
}

// ===========================================================================
// AdapterGenerator
// ===========================================================================

#[test]
fn adapter_no_adapter_needed() {
    let gen = AdapterGenerator::new();
    let mut sig1 = FunctionSignature::new("source", Language::Rust);
    sig1.return_type = Some("String".to_string());
    sig1.is_async = false;
    let mut sig2 = FunctionSignature::new("target", Language::Rust);
    sig2.params = vec![ParamSignature {
        name: "input".to_string(),
        param_type: "String".to_string(),
        is_optional: false,
    }];
    sig2.is_async = false;
    let p1 = make_pattern_with_sig("source", "source body", sig1);
    let p2 = make_pattern_with_sig("target", "target body", sig2);
    let adapter = gen.generate_adapter(&p1, &p2).unwrap();
    assert!(adapter.code.contains("no adapter needed"));
    assert!(!adapter.needs_async_bridge);
}

#[test]
fn adapter_type_conversion() {
    let gen = AdapterGenerator::new();
    let mut sig1 = FunctionSignature::new("source", Language::Rust);
    sig1.return_type = Some("i32".to_string());
    let mut sig2 = FunctionSignature::new("target", Language::Rust);
    sig2.params = vec![ParamSignature {
        name: "input".to_string(),
        param_type: "String".to_string(),
        is_optional: false,
    }];
    let p1 = make_pattern_with_sig("source", "code", sig1);
    let p2 = make_pattern_with_sig("target", "code", sig2);
    let adapter = gen.generate_adapter(&p1, &p2).unwrap();
    assert!(adapter.needs_type_conversion);
    assert!(adapter.code.contains("Type conversion"));
}

#[test]
fn adapter_async_bridge() {
    let gen = AdapterGenerator::new();
    let mut sig1 = FunctionSignature::new("async_source", Language::Rust);
    sig1.is_async = true;
    let sig2 = FunctionSignature::new("sync_target", Language::Rust);
    let p1 = make_pattern_with_sig("async_source", "code", sig1);
    let p2 = make_pattern_with_sig("sync_target", "code", sig2);
    let adapter = gen.generate_adapter(&p1, &p2).unwrap();
    assert!(adapter.needs_async_bridge);
    assert!(adapter.code.contains("Async-to-sync"));
}

#[test]
fn adapter_stores_pattern_ids() {
    let gen = AdapterGenerator::new();
    let sig1 = FunctionSignature::new("s", Language::Rust);
    let sig2 = FunctionSignature::new("t", Language::Rust);
    let p1 = make_pattern_with_sig("s", "code", sig1);
    let p2 = make_pattern_with_sig("t", "code", sig2);
    let adapter = gen.generate_adapter(&p1, &p2).unwrap();
    assert_eq!(adapter.source_pattern_id, p1.id.as_str());
    assert_eq!(adapter.target_pattern_id, p2.id.as_str());
}

// ===========================================================================
// IntegrationWeaver
// ===========================================================================

#[test]
fn weaver_weave_patterns() {
    let weaver = IntegrationWeaver::new();
    let p1 = make_pattern("init", "fn init() { /* init code */ }");
    let p2 = make_pattern("run", "fn run() { /* run code */ }");
    let result = weaver.weave(&[&p1, &p2]).unwrap();
    assert!(result.code.contains("init code"));
    assert!(result.code.contains("run code"));
    assert_eq!(result.patterns_used.len(), 2);
}

#[test]
fn weaver_extracts_imports() {
    let weaver = IntegrationWeaver::new();
    let p1 = make_pattern("p1", "use serde::Serialize;\nfn serialize() {}");
    let p2 = make_pattern("p2", "use serde::Deserialize;\nfn deserialize() {}");
    let result = weaver.weave(&[&p1, &p2]).unwrap();
    assert_eq!(result.import_count, 2);
    assert!(result.code.contains("use serde::Serialize;"));
    assert!(result.code.contains("use serde::Deserialize;"));
}

#[test]
fn weaver_strips_duplicate_imports() {
    let weaver = IntegrationWeaver::new();
    let p1 = make_pattern("p1", "use serde::Serialize;\nfn a() {}");
    let p2 = make_pattern("p2", "use serde::Serialize;\nfn b() {}");
    let result = weaver.weave(&[&p1, &p2]).unwrap();
    // HashSet deduplicates imports
    assert_eq!(result.import_count, 1);
    let count = result.code.matches("use serde::Serialize;").count();
    assert_eq!(count, 1, "Import should appear only once");
}

#[test]
fn weaver_single_pattern() {
    let weaver = IntegrationWeaver::new();
    let p = make_pattern("solo", "fn solo() { 42 }");
    let result = weaver.weave(&[&p]).unwrap();
    assert!(result.code.contains("solo"));
    assert_eq!(result.patterns_used.len(), 1);
}

#[test]
fn weaver_empty_patterns() {
    let weaver = IntegrationWeaver::new();
    let result = weaver.weave(&[]).unwrap();
    assert!(result.code.is_empty() || result.code.trim().is_empty());
    assert_eq!(result.patterns_used.len(), 0);
}

#[test]
fn weaver_python_imports() {
    let weaver = IntegrationWeaver::new();
    let p1 = make_pattern("p1", "import os\ndef list_files(): pass");
    let p2 = make_pattern("p2", "from pathlib import Path\ndef get_path(): pass");
    let result = weaver.weave(&[&p1, &p2]).unwrap();
    assert_eq!(result.import_count, 2);
}

#[test]
fn weaver_preserves_order() {
    let weaver = IntegrationWeaver::new();
    let p1 = make_pattern("first", "fn first() {}");
    let p2 = make_pattern("second", "fn second() {}");
    let result = weaver.weave(&[&p1, &p2]).unwrap();
    let pos1 = result.code.find("first").unwrap_or(usize::MAX);
    let pos2 = result.code.find("second").unwrap_or(usize::MAX);
    assert!(pos1 < pos2, "First pattern should appear before second");
}
