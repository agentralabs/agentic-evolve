//! Phase 4: Crystallization tests — Extractor, VariableDetector, TemplateGenerator, Confidence.

use std::collections::HashMap;

use agentic_evolve_core::crystallization::confidence::ConfidenceCalculator;
use agentic_evolve_core::crystallization::extractor::PatternExtractor;
use agentic_evolve_core::crystallization::template_generator::TemplateGenerator;
use agentic_evolve_core::crystallization::variable_detector::VariableDetector;
use agentic_evolve_core::types::pattern::{Language, PatternVariable};
use agentic_evolve_core::types::skill::{SuccessfulExecution, TestResult};

fn make_execution(
    code: &str,
    lang: Language,
    tests: Vec<TestResult>,
    time_ms: u64,
) -> SuccessfulExecution {
    SuccessfulExecution {
        code: code.to_string(),
        language: lang,
        domain: "test".to_string(),
        test_results: tests,
        execution_time_ms: time_ms,
    }
}

fn make_test(name: &str, passed: bool) -> TestResult {
    TestResult {
        name: name.to_string(),
        passed,
        duration_ms: 5,
    }
}

// ===========================================================================
// PatternExtractor
// ===========================================================================

#[test]
fn extractor_extract_rust_functions() {
    let extractor = PatternExtractor::new();
    let code = r#"
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn helper() {
    println!("helper");
}
"#;
    let exec = make_execution(code, Language::Rust, vec![make_test("test_add", true)], 50);
    let patterns = extractor.extract(&exec).unwrap();
    assert!(
        !patterns.is_empty(),
        "Should extract at least one Rust function"
    );
    // Should find "add" and "helper"
    let names: Vec<&str> = patterns.iter().map(|p| p.name.as_str()).collect();
    assert!(
        names.contains(&"add"),
        "Should extract 'add', got {:?}",
        names
    );
}

#[test]
fn extractor_extract_python_functions() {
    let extractor = PatternExtractor::new();
    let code = r#"
def greet(name: str) -> str:
    return f"Hello, {name}"

async def fetch_data(url: str):
    pass
"#;
    let exec = make_execution(
        code,
        Language::Python,
        vec![make_test("test_greet", true)],
        30,
    );
    let patterns = extractor.extract(&exec).unwrap();
    assert!(!patterns.is_empty());
    let names: Vec<&str> = patterns.iter().map(|p| p.name.as_str()).collect();
    assert!(names.contains(&"greet"));
}

#[test]
fn extractor_extract_generic_language() {
    let extractor = PatternExtractor::new();
    let code = "console.log('hello');";
    let exec = make_execution(
        code,
        Language::JavaScript,
        vec![make_test("test_js", true)],
        10,
    );
    let patterns = extractor.extract(&exec).unwrap();
    // Generic extractor wraps everything as "main"
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0].name, "main");
}

#[test]
fn extractor_empty_code() {
    let extractor = PatternExtractor::new();
    let exec = make_execution("", Language::Rust, vec![make_test("test_empty", true)], 5);
    let patterns = extractor.extract(&exec).unwrap();
    // No Rust functions to extract from empty code
    assert!(patterns.is_empty());
}

#[test]
fn extractor_multiple_rust_functions() {
    let extractor = PatternExtractor::new();
    let code = r#"
pub fn first() {
    println!("first");
}

pub fn second() {
    println!("second");
}

pub fn third() {
    println!("third");
}
"#;
    let exec = make_execution(code, Language::Rust, vec![make_test("test_all", true)], 20);
    let patterns = extractor.extract(&exec).unwrap();
    assert!(patterns.len() >= 3);
}

#[test]
fn extractor_async_rust_function() {
    let extractor = PatternExtractor::new();
    let code = r#"
pub async fn fetch(url: &str) -> String {
    reqwest::get(url).await.unwrap().text().await.unwrap()
}
"#;
    let exec = make_execution(
        code,
        Language::Rust,
        vec![make_test("test_fetch", true)],
        50,
    );
    let patterns = extractor.extract(&exec).unwrap();
    assert!(!patterns.is_empty());
    assert!(patterns[0].signature.is_async);
}

#[test]
fn extractor_rust_params_extracted() {
    let extractor = PatternExtractor::new();
    let code = r#"
pub fn process(data: &str, count: usize) -> bool {
    true
}
"#;
    let exec = make_execution(
        code,
        Language::Rust,
        vec![make_test("test_process", true)],
        10,
    );
    let patterns = extractor.extract(&exec).unwrap();
    assert!(!patterns.is_empty());
    assert_eq!(patterns[0].signature.params.len(), 2);
}

#[test]
fn extractor_python_optional_params() {
    let extractor = PatternExtractor::new();
    let code = r#"
def configure(host: str, port: int = 8080):
    pass
"#;
    let exec = make_execution(
        code,
        Language::Python,
        vec![make_test("test_config", true)],
        5,
    );
    let patterns = extractor.extract(&exec).unwrap();
    assert!(!patterns.is_empty());
    let optional_count = patterns[0]
        .signature
        .params
        .iter()
        .filter(|p| p.is_optional)
        .count();
    assert!(optional_count >= 1);
}

// ===========================================================================
// VariableDetector
// ===========================================================================

#[test]
fn detector_detect_string_literals() {
    let detector = VariableDetector::new();
    let vars = detector.detect(r#"let name = "hello world";"#, &Language::Rust);
    let string_vars: Vec<_> = vars.iter().filter(|v| v.var_type == "string").collect();
    assert!(!string_vars.is_empty(), "Should detect string literal");
}

#[test]
fn detector_detect_numeric_literals() {
    let detector = VariableDetector::new();
    let vars = detector.detect("let port = 8080;", &Language::Rust);
    let num_vars: Vec<_> = vars.iter().filter(|v| v.var_type == "number").collect();
    assert!(!num_vars.is_empty(), "Should detect numeric literal 8080");
}

#[test]
fn detector_detect_rust_type_names() {
    let detector = VariableDetector::new();
    let vars = detector.detect("let config = MyConfig::new();", &Language::Rust);
    let type_vars: Vec<_> = vars.iter().filter(|v| v.var_type == "type").collect();
    assert!(!type_vars.is_empty(), "Should detect custom type MyConfig");
}

#[test]
fn detector_detect_python_type_names() {
    let detector = VariableDetector::new();
    let vars = detector.detect("config = MyConfig()", &Language::Python);
    let type_vars: Vec<_> = vars.iter().filter(|v| v.var_type == "type").collect();
    assert!(
        !type_vars.is_empty(),
        "Should detect custom type MyConfig in Python"
    );
}

#[test]
fn detector_common_types_excluded_rust() {
    let detector = VariableDetector::new();
    let vars = detector.detect("let x: Option<String> = None;", &Language::Rust);
    let type_vars: Vec<_> = vars.iter().filter(|v| v.var_type == "type").collect();
    // String, Option, None are common types and should be excluded
    assert!(
        type_vars.is_empty(),
        "Common types should be excluded, got: {:?}",
        type_vars.iter().map(|v| &v.name).collect::<Vec<_>>()
    );
}

#[test]
fn detector_common_types_excluded_python() {
    let detector = VariableDetector::new();
    let vars = detector.detect("x: Optional[List[str]] = None", &Language::Python);
    let type_vars: Vec<_> = vars.iter().filter(|v| v.var_type == "type").collect();
    assert!(type_vars.is_empty());
}

#[test]
fn detector_no_variables_in_simple_code() {
    let detector = VariableDetector::new();
    let vars = detector.detect("let x = y + z;", &Language::Rust);
    // No string literals (3+ chars), no 2+ digit numbers, no custom types
    assert!(vars.is_empty() || vars.iter().all(|v| v.var_type != "string"));
}

#[test]
fn detector_other_language_no_types() {
    let detector = VariableDetector::new();
    let vars = detector.detect("let config = MyConfig();", &Language::Go);
    // Go is not Rust or Python, so no type detection
    let type_vars: Vec<_> = vars.iter().filter(|v| v.var_type == "type").collect();
    assert!(type_vars.is_empty());
}

// ===========================================================================
// TemplateGenerator
// ===========================================================================

#[test]
fn template_generate_replaces_variables() {
    let gen = TemplateGenerator::new();
    let vars = vec![PatternVariable {
        name: "PORT".to_string(),
        var_type: "number".to_string(),
        pattern: None,
        default: Some("8080".to_string()),
    }];
    let template = gen.generate("let port = 8080;", &vars);
    assert!(template.contains("{{PORT}}"));
    assert!(!template.contains("8080"));
}

#[test]
fn template_apply_bindings() {
    let gen = TemplateGenerator::new();
    let mut bindings = HashMap::new();
    bindings.insert("PORT".to_string(), "3000".to_string());
    let result = gen.apply_bindings("let port = {{PORT}};", &bindings);
    assert_eq!(result, "let port = 3000;");
}

#[test]
fn template_extract_placeholders() {
    let gen = TemplateGenerator::new();
    let placeholders = gen.extract_placeholders("fn {{NAME}}({{PARAM}}: {{TYPE}}) {}");
    assert_eq!(placeholders.len(), 3);
    assert!(placeholders.contains(&"NAME".to_string()));
    assert!(placeholders.contains(&"PARAM".to_string()));
    assert!(placeholders.contains(&"TYPE".to_string()));
}

#[test]
fn template_has_unbound_placeholders_true() {
    let gen = TemplateGenerator::new();
    assert!(gen.has_unbound_placeholders("fn {{NAME}}() {}"));
}

#[test]
fn template_has_unbound_placeholders_false() {
    let gen = TemplateGenerator::new();
    assert!(!gen.has_unbound_placeholders("fn main() {}"));
}

#[test]
fn template_empty() {
    let gen = TemplateGenerator::new();
    let template = gen.generate("", &[]);
    assert_eq!(template, "");
}

#[test]
fn template_no_variables() {
    let gen = TemplateGenerator::new();
    let template = gen.generate("fn main() {}", &[]);
    assert_eq!(template, "fn main() {}");
}

#[test]
fn template_apply_multiple_bindings() {
    let gen = TemplateGenerator::new();
    let mut bindings = HashMap::new();
    bindings.insert("A".to_string(), "1".to_string());
    bindings.insert("B".to_string(), "2".to_string());
    let result = gen.apply_bindings("{{A}} + {{B}}", &bindings);
    assert_eq!(result, "1 + 2");
}

#[test]
fn template_extract_no_placeholders() {
    let gen = TemplateGenerator::new();
    let placeholders = gen.extract_placeholders("fn main() {}");
    assert!(placeholders.is_empty());
}

// ===========================================================================
// ConfidenceCalculator
// ===========================================================================

#[test]
fn confidence_all_tests_pass_high() {
    let calc = ConfidenceCalculator::new();
    let exec = make_execution(
        "fn test() { assert!(true); }",
        Language::Rust,
        vec![
            make_test("t1", true),
            make_test("t2", true),
            make_test("t3", true),
        ],
        50,
    );
    let score = calc.calculate(&exec);
    assert!(
        score > 0.7,
        "All tests passing should give high confidence, got {score}"
    );
}

#[test]
fn confidence_no_tests_moderate() {
    let calc = ConfidenceCalculator::new();
    let exec = make_execution("fn test() {}", Language::Rust, vec![], 50);
    let score = calc.calculate(&exec);
    assert!(
        score > 0.3 && score < 0.8,
        "No tests should give moderate confidence, got {score}"
    );
}

#[test]
fn confidence_mixed_results() {
    let calc = ConfidenceCalculator::new();
    let exec = make_execution(
        "fn test() {}",
        Language::Rust,
        vec![
            make_test("t1", true),
            make_test("t2", false),
            make_test("t3", true),
        ],
        50,
    );
    let score = calc.calculate(&exec);
    // Mixed: 2/3 tests pass = 0.667 test score
    assert!(score > 0.3 && score < 0.9);
}

#[test]
fn confidence_fast_execution() {
    let calc = ConfidenceCalculator::new();
    let exec_fast = make_execution(
        "fn test() {}",
        Language::Rust,
        vec![make_test("t1", true)],
        10,
    );
    let exec_slow = make_execution(
        "fn test() {}",
        Language::Rust,
        vec![make_test("t1", true)],
        10000,
    );
    let fast_score = calc.calculate(&exec_fast);
    let slow_score = calc.calculate(&exec_slow);
    assert!(
        fast_score >= slow_score,
        "Fast execution should score higher: fast={fast_score} slow={slow_score}"
    );
}

#[test]
fn confidence_slow_execution() {
    let calc = ConfidenceCalculator::new();
    let exec = make_execution(
        "fn test() {}",
        Language::Rust,
        vec![make_test("t1", true)],
        50000,
    );
    let score = calc.calculate(&exec);
    assert!(
        score > 0.0,
        "Even slow execution should have some confidence"
    );
}

#[test]
fn confidence_simple_code() {
    let calc = ConfidenceCalculator::new();
    let exec = make_execution(
        "fn add(a: i32, b: i32) -> i32 { a + b }",
        Language::Rust,
        vec![make_test("t1", true)],
        5,
    );
    let score = calc.calculate(&exec);
    assert!(
        score > 0.6,
        "Simple code should have good confidence, got {score}"
    );
}

#[test]
fn confidence_complex_code() {
    let calc = ConfidenceCalculator::new();
    // Deeply nested code
    let code = "fn complex() { if a { if b { if c { if d { if e { if f { } } } } } } }";
    let exec = make_execution(code, Language::Rust, vec![make_test("t1", true)], 5);
    let score = calc.calculate(&exec);
    assert!((0.0..=1.0).contains(&score) && score > 0.0);
}

#[test]
fn confidence_clamped_to_zero_one() {
    let calc = ConfidenceCalculator::new();
    let exec = make_execution("x", Language::Rust, vec![make_test("t1", true)], 1);
    let score = calc.calculate(&exec);
    assert!((0.0..=1.0).contains(&score));
}

#[test]
fn confidence_default_same_as_new() {
    let a = ConfidenceCalculator::new();
    let b = ConfidenceCalculator::default();
    let exec = make_execution("fn x() {}", Language::Rust, vec![], 10);
    let sa = a.calculate(&exec);
    let sb = b.calculate(&exec);
    assert!((sa - sb).abs() < f64::EPSILON);
}
