//! CompositeMatcher — combines all matchers for best results.

use crate::types::error::EvolveResult;
use crate::types::match_result::{MatchContext, MatchResult, MatchScore};
use crate::types::pattern::{FunctionSignature, Pattern};

use super::{ContextMatcher, FuzzyMatcher, SemanticMatcher, SignatureMatcher};

/// Combines all matchers with configurable weights.
#[derive(Debug)]
pub struct CompositeMatcher {
    signature_matcher: SignatureMatcher,
    context_matcher: ContextMatcher,
    #[allow(dead_code)]
    semantic_matcher: SemanticMatcher,
    #[allow(dead_code)]
    fuzzy_matcher: FuzzyMatcher,
    weights: MatchWeights,
}

/// Weights for combining matcher scores.
#[derive(Debug, Clone)]
pub struct MatchWeights {
    pub signature: f64,
    pub context: f64,
    pub semantic: f64,
    pub fuzzy: f64,
}

impl Default for MatchWeights {
    fn default() -> Self {
        Self {
            signature: 0.4,
            context: 0.2,
            semantic: 0.25,
            fuzzy: 0.15,
        }
    }
}

impl CompositeMatcher {
    pub fn new() -> Self {
        Self {
            signature_matcher: SignatureMatcher::new(),
            context_matcher: ContextMatcher::new(),
            semantic_matcher: SemanticMatcher::new(),
            fuzzy_matcher: FuzzyMatcher::default(),
            weights: MatchWeights::default(),
        }
    }

    pub fn with_weights(weights: MatchWeights) -> Self {
        Self {
            signature_matcher: SignatureMatcher::new(),
            context_matcher: ContextMatcher::new(),
            semantic_matcher: SemanticMatcher::new(),
            fuzzy_matcher: FuzzyMatcher::default(),
            weights,
        }
    }

    pub fn find_matches(
        &self,
        signature: &FunctionSignature,
        patterns: &[&Pattern],
        context: &MatchContext,
        limit: usize,
    ) -> EvolveResult<Vec<MatchResult>> {
        let mut combined_scores: std::collections::HashMap<String, (f64, f64, f64, f64, &Pattern)> =
            std::collections::HashMap::new();

        // Score each pattern with each matcher
        for pattern in patterns {
            let sig_score = self.signature_matcher.score_match(pattern, signature);
            let ctx_score = self.context_matcher.score_context(pattern, context);
            let sem_score = {
                let tokens = tokenize_camel_snake(&signature.name);
                let pat_tokens = tokenize_camel_snake(&pattern.name);
                token_overlap(&tokens, &pat_tokens)
            };
            let confidence_factor = pattern.confidence;

            combined_scores.insert(
                pattern.id.as_str().to_string(),
                (sig_score, ctx_score, sem_score, confidence_factor, pattern),
            );
        }

        let mut results: Vec<MatchResult> = combined_scores
            .into_iter()
            .map(|(_id, (sig, ctx, sem, conf, pattern))| {
                let combined = sig * self.weights.signature
                    + ctx * self.weights.context
                    + sem * self.weights.semantic
                    + conf * self.weights.fuzzy;
                MatchResult {
                    pattern_id: pattern.id.clone(),
                    pattern: pattern.clone(),
                    score: MatchScore::new(sig, ctx, sem, combined),
                    suggested_bindings: std::collections::HashMap::new(),
                }
            })
            .filter(|r| r.score.combined > 0.1)
            .collect();

        results.sort_by(|a, b| {
            b.score
                .combined
                .partial_cmp(&a.score.combined)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results.truncate(limit);
        Ok(results)
    }
}

impl Default for CompositeMatcher {
    fn default() -> Self {
        Self::new()
    }
}

fn tokenize_camel_snake(name: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    for ch in name.chars() {
        if ch == '_' || ch == '-' || ch == ' ' {
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

fn token_overlap(a: &[String], b: &[String]) -> f64 {
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    let max_len = a.len().max(b.len());
    if max_len == 0 {
        return 0.0;
    }
    let matches = a.iter().filter(|t| b.contains(t)).count();
    matches as f64 / max_len as f64
}
