//! FuzzyMatcher — handles approximate matches with edit distance.

use crate::types::error::EvolveResult;
use crate::types::match_result::{MatchContext, MatchResult, MatchScore};
use crate::types::pattern::{FunctionSignature, Pattern};

/// Matches patterns using fuzzy/approximate matching.
#[derive(Debug)]
pub struct FuzzyMatcher {
    threshold: f64,
}

impl FuzzyMatcher {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    pub fn find_matches(
        &self,
        signature: &FunctionSignature,
        patterns: &[&Pattern],
        _context: &MatchContext,
        limit: usize,
    ) -> EvolveResult<Vec<MatchResult>> {
        let mut results: Vec<MatchResult> = patterns
            .iter()
            .map(|p| {
                let score = self.score_fuzzy(p, signature);
                MatchResult {
                    pattern_id: p.id.clone(),
                    pattern: (*p).clone(),
                    score: MatchScore::from_single(score),
                    suggested_bindings: std::collections::HashMap::new(),
                }
            })
            .filter(|r| r.score.combined >= self.threshold)
            .collect();

        results.sort_by(|a, b| b.score.combined.partial_cmp(&a.score.combined).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);
        Ok(results)
    }

    fn score_fuzzy(&self, pattern: &Pattern, signature: &FunctionSignature) -> f64 {
        let name_sim = fuzzy_similarity(&pattern.signature.name, &signature.name);
        let lang_match = if pattern.signature.language == signature.language { 1.0 } else { 0.5 };

        let param_sim = if pattern.signature.params.is_empty() && signature.params.is_empty() {
            1.0
        } else {
            let max_params = pattern.signature.params.len().max(signature.params.len());
            let matching = pattern.signature.params.iter()
                .zip(signature.params.iter())
                .filter(|(a, b)| fuzzy_similarity(&a.param_type, &b.param_type) > 0.6)
                .count();
            if max_params == 0 { 1.0 } else { matching as f64 / max_params as f64 }
        };

        name_sim * 0.5 + lang_match * 0.2 + param_sim * 0.3
    }
}

impl Default for FuzzyMatcher {
    fn default() -> Self {
        Self::new(0.3)
    }
}

fn fuzzy_similarity(a: &str, b: &str) -> f64 {
    if a == b { return 1.0; }
    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();
    if a_lower == b_lower { return 0.95; }

    // Trigram similarity
    let a_trigrams = trigrams(&a_lower);
    let b_trigrams = trigrams(&b_lower);
    if a_trigrams.is_empty() && b_trigrams.is_empty() { return 1.0; }
    if a_trigrams.is_empty() || b_trigrams.is_empty() { return 0.0; }

    let intersection = a_trigrams.iter().filter(|t| b_trigrams.contains(t)).count();
    let union = a_trigrams.len() + b_trigrams.len() - intersection;
    if union == 0 { return 0.0; }
    intersection as f64 / union as f64
}

fn trigrams(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < 3 {
        return vec![s.to_string()];
    }
    chars.windows(3).map(|w| w.iter().collect()).collect()
}
