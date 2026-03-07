//! Phase 1: Type system tests — IDs, Language, Pattern, Skill, Error, Match types.

use std::collections::HashMap;

use agentic_evolve_core::types::error::EvolveError;
use agentic_evolve_core::types::ids::{EvolveId, PatternId, SkillId};
use agentic_evolve_core::types::match_result::{MatchContext, MatchScore};
use agentic_evolve_core::types::pattern::{FunctionSignature, Language, Pattern, PatternVariable};
use agentic_evolve_core::types::skill::{
    Complexity, CrystallizedSkill, SkillMetadata, SuccessfulExecution, TestResult,
};

// ---------------------------------------------------------------------------
// ID types
// ---------------------------------------------------------------------------

#[test]
fn evolve_id_new_is_unique() {
    let a = EvolveId::new();
    let b = EvolveId::new();
    assert_ne!(a, b);
}

#[test]
fn evolve_id_from_string_roundtrip() {
    let id = EvolveId::from_string("test-id-123");
    assert_eq!(id.as_str(), "test-id-123");
}

#[test]
fn evolve_id_display() {
    let id = EvolveId::from_string("display-test");
    assert_eq!(format!("{id}"), "display-test");
}

#[test]
fn evolve_id_equality() {
    let a = EvolveId::from_string("same");
    let b = EvolveId::from_string("same");
    assert_eq!(a, b);
}

#[test]
fn evolve_id_default_is_uuid() {
    let id = EvolveId::default();
    assert!(!id.as_str().is_empty());
    // UUID v4 format: 8-4-4-4-12
    assert_eq!(id.as_str().len(), 36);
}

#[test]
fn pattern_id_new_is_unique() {
    let a = PatternId::new();
    let b = PatternId::new();
    assert_ne!(a, b);
}

#[test]
fn pattern_id_from_string_roundtrip() {
    let id = PatternId::from_string("pat-42");
    assert_eq!(id.as_str(), "pat-42");
}

#[test]
fn pattern_id_display() {
    let id = PatternId::from_string("pat-display");
    assert_eq!(format!("{id}"), "pat-display");
}

#[test]
fn skill_id_new_is_unique() {
    let a = SkillId::new();
    let b = SkillId::new();
    assert_ne!(a, b);
}

#[test]
fn skill_id_from_string_roundtrip() {
    let id = SkillId::from_string("skill-1");
    assert_eq!(id.as_str(), "skill-1");
}

#[test]
fn skill_id_display() {
    let id = SkillId::from_string("skill-display");
    assert_eq!(format!("{id}"), "skill-display");
}

#[test]
fn skill_id_default_is_uuid() {
    let id = SkillId::default();
    assert_eq!(id.as_str().len(), 36);
}

// ---------------------------------------------------------------------------
// Language enum
// ---------------------------------------------------------------------------

#[test]
fn language_from_name_rust() {
    assert_eq!(Language::from_name("rust"), Language::Rust);
    assert_eq!(Language::from_name("rs"), Language::Rust);
    assert_eq!(Language::from_name("Rust"), Language::Rust);
}

#[test]
fn language_from_name_python() {
    assert_eq!(Language::from_name("python"), Language::Python);
    assert_eq!(Language::from_name("py"), Language::Python);
}

#[test]
fn language_from_name_typescript() {
    assert_eq!(Language::from_name("typescript"), Language::TypeScript);
    assert_eq!(Language::from_name("ts"), Language::TypeScript);
}

#[test]
fn language_from_name_javascript() {
    assert_eq!(Language::from_name("javascript"), Language::JavaScript);
    assert_eq!(Language::from_name("js"), Language::JavaScript);
}

#[test]
fn language_from_name_go() {
    assert_eq!(Language::from_name("go"), Language::Go);
    assert_eq!(Language::from_name("golang"), Language::Go);
}

#[test]
fn language_from_name_java() {
    assert_eq!(Language::from_name("java"), Language::Java);
}

#[test]
fn language_from_name_csharp() {
    assert_eq!(Language::from_name("csharp"), Language::CSharp);
    assert_eq!(Language::from_name("c#"), Language::CSharp);
    assert_eq!(Language::from_name("cs"), Language::CSharp);
}

#[test]
fn language_from_name_cpp() {
    assert_eq!(Language::from_name("cpp"), Language::Cpp);
    assert_eq!(Language::from_name("c++"), Language::Cpp);
}

#[test]
fn language_from_name_c() {
    assert_eq!(Language::from_name("c"), Language::C);
}

#[test]
fn language_from_name_shell() {
    assert_eq!(Language::from_name("shell"), Language::Shell);
    assert_eq!(Language::from_name("bash"), Language::Shell);
    assert_eq!(Language::from_name("sh"), Language::Shell);
    assert_eq!(Language::from_name("zsh"), Language::Shell);
}

#[test]
fn language_from_name_other() {
    let lang = Language::from_name("haskell");
    assert_eq!(lang, Language::Other("haskell".to_string()));
}

#[test]
fn language_as_str_roundtrip() {
    assert_eq!(Language::Rust.as_str(), "rust");
    assert_eq!(Language::Python.as_str(), "python");
    assert_eq!(Language::TypeScript.as_str(), "typescript");
    assert_eq!(Language::JavaScript.as_str(), "javascript");
    assert_eq!(Language::Go.as_str(), "go");
    assert_eq!(Language::Java.as_str(), "java");
    assert_eq!(Language::CSharp.as_str(), "csharp");
    assert_eq!(Language::Cpp.as_str(), "cpp");
    assert_eq!(Language::C.as_str(), "c");
    assert_eq!(Language::Shell.as_str(), "shell");
}

#[test]
fn language_display() {
    assert_eq!(format!("{}", Language::Rust), "rust");
    assert_eq!(format!("{}", Language::Python), "python");
}

// ---------------------------------------------------------------------------
// FunctionSignature
// ---------------------------------------------------------------------------

#[test]
fn function_signature_creation() {
    let sig = FunctionSignature::new("do_thing", Language::Rust);
    assert_eq!(sig.name, "do_thing");
    assert_eq!(sig.language, Language::Rust);
    assert!(sig.params.is_empty());
    assert!(sig.return_type.is_none());
    assert!(!sig.is_async);
}

// ---------------------------------------------------------------------------
// Pattern
// ---------------------------------------------------------------------------

fn make_test_pattern(name: &str, domain: &str) -> Pattern {
    let sig = FunctionSignature::new(name, Language::Rust);
    Pattern::new(
        name,
        domain,
        Language::Rust,
        sig,
        "fn {{NAME}}() {}",
        vec![],
        0.9,
    )
}

#[test]
fn pattern_creation() {
    let p = make_test_pattern("my_fn", "web");
    assert_eq!(p.name, "my_fn");
    assert_eq!(p.domain, "web");
    assert_eq!(p.confidence, 0.9);
    assert_eq!(p.usage_count, 0);
    assert_eq!(p.success_count, 0);
    assert_eq!(p.version, 1);
    assert!(!p.content_hash.is_empty());
}

#[test]
fn pattern_success_rate_zero_uses() {
    let p = make_test_pattern("unused", "misc");
    assert_eq!(p.success_rate(), 0.0);
}

#[test]
fn pattern_success_rate_after_uses() {
    let mut p = make_test_pattern("used", "web");
    p.record_use(true);
    p.record_use(true);
    p.record_use(false);
    assert!((p.success_rate() - 2.0 / 3.0).abs() < f64::EPSILON);
}

#[test]
fn pattern_record_use_increments_counts() {
    let mut p = make_test_pattern("counter", "web");
    p.record_use(true);
    assert_eq!(p.usage_count, 1);
    assert_eq!(p.success_count, 1);
    p.record_use(false);
    assert_eq!(p.usage_count, 2);
    assert_eq!(p.success_count, 1);
}

// ---------------------------------------------------------------------------
// PatternVariable
// ---------------------------------------------------------------------------

#[test]
fn pattern_variable_creation() {
    let var = PatternVariable {
        name: "DB_NAME".to_string(),
        var_type: "string".to_string(),
        pattern: Some(r"\w+".to_string()),
        default: Some("mydb".to_string()),
    };
    assert_eq!(var.name, "DB_NAME");
    assert_eq!(var.var_type, "string");
    assert_eq!(var.default.unwrap(), "mydb");
}

// ---------------------------------------------------------------------------
// CrystallizedSkill
// ---------------------------------------------------------------------------

#[test]
fn crystallized_skill_creation() {
    let pid = PatternId::from_string("pat-1");
    let meta = SkillMetadata {
        domain: "web".to_string(),
        language: Language::Rust,
        complexity: Complexity::Simple,
        source: "test".to_string(),
    };
    let skill = CrystallizedSkill::new(pid.clone(), "fn main() {}", HashMap::new(), meta);
    assert_eq!(skill.pattern_id, pid);
    assert_eq!(skill.code, "fn main() {}");
    assert_eq!(skill.verified_count, 1);
}

#[test]
fn crystallized_skill_record_verification() {
    let pid = PatternId::from_string("pat-2");
    let meta = SkillMetadata {
        domain: "cli".to_string(),
        language: Language::Python,
        complexity: Complexity::Medium,
        source: "test".to_string(),
    };
    let mut skill = CrystallizedSkill::new(pid, "def main(): pass", HashMap::new(), meta);
    assert_eq!(skill.verified_count, 1);
    skill.record_verification();
    assert_eq!(skill.verified_count, 2);
}

// ---------------------------------------------------------------------------
// SuccessfulExecution / TestResult
// ---------------------------------------------------------------------------

#[test]
fn successful_execution_creation() {
    let exec = SuccessfulExecution {
        code: "fn hello() {}".to_string(),
        language: Language::Rust,
        domain: "greeting".to_string(),
        test_results: vec![TestResult {
            name: "test_hello".to_string(),
            passed: true,
            duration_ms: 5,
        }],
        execution_time_ms: 10,
    };
    assert_eq!(exec.test_results.len(), 1);
    assert!(exec.test_results[0].passed);
}

// ---------------------------------------------------------------------------
// MatchScore
// ---------------------------------------------------------------------------

#[test]
fn match_score_new_combined_weight() {
    let score = MatchScore::new(1.0, 0.5, 0.5, 0.5);
    // combined = 1.0*0.4 + 0.5*0.2 + 0.5*0.2 + 0.5*0.2 = 0.4 + 0.1 + 0.1 + 0.1 = 0.7
    assert!((score.combined - 0.7).abs() < 1e-9);
}

#[test]
fn match_score_from_single() {
    let score = MatchScore::from_single(0.8);
    assert!((score.signature_score - 0.8).abs() < f64::EPSILON);
    assert!((score.context_score - 0.8).abs() < f64::EPSILON);
    assert!((score.semantic_score - 0.8).abs() < f64::EPSILON);
    assert!((score.confidence_score - 0.8).abs() < f64::EPSILON);
    assert!((score.combined - 0.8).abs() < f64::EPSILON);
}

// ---------------------------------------------------------------------------
// MatchContext builder
// ---------------------------------------------------------------------------

#[test]
fn match_context_default() {
    let ctx = MatchContext::new();
    assert!(ctx.domain.is_none());
    assert!(ctx.surrounding_code.is_none());
    assert!(ctx.imports.is_empty());
    assert_eq!(ctx.max_results, 10);
}

#[test]
fn match_context_builder_chain() {
    let ctx = MatchContext::new()
        .with_domain("web")
        .with_surrounding_code("use serde;")
        .with_max_results(5);
    assert_eq!(ctx.domain.unwrap(), "web");
    assert_eq!(ctx.surrounding_code.unwrap(), "use serde;");
    assert_eq!(ctx.max_results, 5);
}

// ---------------------------------------------------------------------------
// EvolveError variants
// ---------------------------------------------------------------------------

#[test]
fn error_pattern_not_found_display() {
    let e = EvolveError::PatternNotFound("abc".to_string());
    assert!(e.to_string().contains("abc"));
}

#[test]
fn error_skill_not_found_display() {
    let e = EvolveError::SkillNotFound("xyz".to_string());
    assert!(e.to_string().contains("xyz"));
}

#[test]
fn error_invalid_pattern_display() {
    let e = EvolveError::InvalidPattern("bad data".to_string());
    assert!(e.to_string().contains("bad data"));
}

#[test]
fn error_storage_display() {
    let e = EvolveError::StorageError("disk full".to_string());
    assert!(e.to_string().contains("disk full"));
}

#[test]
fn error_serialization_display() {
    let e = EvolveError::SerializationError("bad json".to_string());
    assert!(e.to_string().contains("bad json"));
}

#[test]
fn error_matching_display() {
    let e = EvolveError::MatchingError("no patterns".to_string());
    assert!(e.to_string().contains("no patterns"));
}

#[test]
fn error_crystallization_display() {
    let e = EvolveError::CrystallizationError("parse fail".to_string());
    assert!(e.to_string().contains("parse fail"));
}

#[test]
fn error_composition_display() {
    let e = EvolveError::CompositionError("empty".to_string());
    assert!(e.to_string().contains("empty"));
}

#[test]
fn error_template_display() {
    let e = EvolveError::TemplateError("missing var".to_string());
    assert!(e.to_string().contains("missing var"));
}

// ---------------------------------------------------------------------------
// Complexity enum
// ---------------------------------------------------------------------------

#[test]
fn complexity_variants_exist() {
    let _ = Complexity::Simple;
    let _ = Complexity::Medium;
    let _ = Complexity::Complex;
}

#[test]
fn complexity_equality() {
    assert_eq!(Complexity::Simple, Complexity::Simple);
    assert_ne!(Complexity::Simple, Complexity::Complex);
}
