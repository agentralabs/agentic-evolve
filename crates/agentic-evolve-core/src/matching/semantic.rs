//! SemanticMatcher — matches based on semantic meaning.

use crate::types::error::EvolveResult;
use crate::types::match_result::{MatchContext, MatchResult, MatchScore};
use crate::types::pattern::{FunctionSignature, Pattern};

/// Matches patterns based on semantic meaning of names and types.
#[derive(Debug, Default)]
pub struct SemanticMatcher;

impl SemanticMatcher {
    pub fn new() -> Self {
        Self
    }

    pub fn find_matches(
        &self,
        signature: &FunctionSignature,
        patterns: &[&Pattern],
        _context: &MatchContext,
        limit: usize,
    ) -> EvolveResult<Vec<MatchResult>> {
        let sig_tokens = tokenize_name(&signature.name);

        let mut results: Vec<MatchResult> = patterns
            .iter()
            .map(|p| {
                let score = self.score_semantic(p, &sig_tokens);
                MatchResult {
                    pattern_id: p.id.clone(),
                    pattern: (*p).clone(),
                    score: MatchScore::from_single(score),
                    suggested_bindings: std::collections::HashMap::new(),
                }
            })
            .filter(|r| r.score.combined > 0.0)
            .collect();

        results.sort_by(|a, b| b.score.combined.partial_cmp(&a.score.combined).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);
        Ok(results)
    }

    fn score_semantic(&self, pattern: &Pattern, query_tokens: &[String]) -> f64 {
        let pattern_tokens = tokenize_name(&pattern.name);
        if query_tokens.is_empty() || pattern_tokens.is_empty() {
            return 0.0;
        }
        let matches = query_tokens.iter()
            .filter(|qt| {
                pattern_tokens.iter().any(|pt| {
                    pt == *qt || is_semantic_match(pt, qt)
                })
            })
            .count();
        let max_len = query_tokens.len().max(pattern_tokens.len());
        matches as f64 / max_len as f64
    }
}

fn tokenize_name(name: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for ch in name.chars() {
        if ch == '_' || ch == '-' || ch == '.' || ch == ' ' {
            if !current.is_empty() {
                tokens.push(current.to_lowercase());
                current.clear();
            }
        } else if ch.is_uppercase() && !current.is_empty() {
            tokens.push(current.to_lowercase());
            current.clear();
            current.push(ch);
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        tokens.push(current.to_lowercase());
    }
    tokens
}

fn is_semantic_match(a: &str, b: &str) -> bool {
    const SYNONYMS: &[&[&str]] = &[
        &["get", "fetch", "retrieve", "read", "load", "find", "query"],
        &["set", "put", "write", "store", "save", "update"],
        &["delete", "remove", "drop", "destroy", "clear"],
        &["create", "new", "build", "make", "init", "construct"],
        &["list", "all", "enumerate", "iter"],
        &["check", "validate", "verify", "test", "is", "has"],
        &["parse", "decode", "deserialize", "from"],
        &["format", "encode", "serialize", "to"],
        &["send", "emit", "dispatch", "publish", "notify"],
        &["receive", "handle", "process", "consume", "subscribe"],
    ];

    for group in SYNONYMS {
        if group.contains(&a) && group.contains(&b) {
            return true;
        }
    }
    false
}
