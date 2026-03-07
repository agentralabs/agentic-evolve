//! SignatureMatcher — matches function signatures to patterns.

use crate::types::error::EvolveResult;
use crate::types::match_result::{MatchContext, MatchResult, MatchScore};
use crate::types::pattern::{FunctionSignature, Pattern};

/// Matches patterns based on function signature similarity.
#[derive(Debug, Default)]
pub struct SignatureMatcher;

impl SignatureMatcher {
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
        let mut results: Vec<MatchResult> = patterns
            .iter()
            .map(|p| {
                let score = self.score_match(p, signature);
                MatchResult {
                    pattern_id: p.id.clone(),
                    pattern: (*p).clone(),
                    score: MatchScore::from_single(score),
                    suggested_bindings: std::collections::HashMap::new(),
                }
            })
            .filter(|r| r.score.combined > 0.0)
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

    pub fn score_match(&self, pattern: &Pattern, signature: &FunctionSignature) -> f64 {
        let mut score = 0.0;
        let mut factors = 0;

        // Name similarity
        let name_score = string_similarity(&pattern.signature.name, &signature.name);
        score += name_score;
        factors += 1;

        // Language match
        if pattern.signature.language == signature.language {
            score += 1.0;
        }
        factors += 1;

        // Parameter count similarity
        let param_diff =
            (pattern.signature.params.len() as i32 - signature.params.len() as i32).unsigned_abs();
        let param_score = if param_diff == 0 {
            1.0
        } else {
            1.0 / (1.0 + param_diff as f64)
        };
        score += param_score;
        factors += 1;

        // Return type match
        if pattern.signature.return_type == signature.return_type {
            score += 1.0;
        } else if pattern.signature.return_type.is_some() && signature.return_type.is_some() {
            score += string_similarity(
                pattern.signature.return_type.as_deref().unwrap_or(""),
                signature.return_type.as_deref().unwrap_or(""),
            );
        }
        factors += 1;

        // Async match
        if pattern.signature.is_async == signature.is_async {
            score += 0.5;
        }
        factors += 1;

        // Parameter type similarity
        let type_score = param_type_similarity(&pattern.signature.params, &signature.params);
        score += type_score;
        factors += 1;

        if factors > 0 {
            score / factors as f64
        } else {
            0.0
        }
    }
}

fn string_similarity(a: &str, b: &str) -> f64 {
    if a == b {
        return 1.0;
    }
    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();
    if a_lower == b_lower {
        return 0.95;
    }
    // Simple substring match
    if a_lower.contains(&b_lower) || b_lower.contains(&a_lower) {
        return 0.7;
    }
    // Levenshtein-based similarity
    let max_len = a.len().max(b.len());
    if max_len == 0 {
        return 1.0;
    }
    let dist = levenshtein_distance(&a_lower, &b_lower);
    1.0 - (dist as f64 / max_len as f64)
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for (i, row) in dp.iter_mut().enumerate().take(m + 1) {
        row[0] = i;
    }
    for j in 0..=n {
        dp[0][j] = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] {
                0
            } else {
                1
            };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }
    dp[m][n]
}

fn param_type_similarity(
    a_params: &[crate::types::pattern::ParamSignature],
    b_params: &[crate::types::pattern::ParamSignature],
) -> f64 {
    if a_params.is_empty() && b_params.is_empty() {
        return 1.0;
    }
    let max_len = a_params.len().max(b_params.len());
    if max_len == 0 {
        return 1.0;
    }
    let matches: f64 = a_params
        .iter()
        .zip(b_params.iter())
        .map(|(a, b)| string_similarity(&a.param_type, &b.param_type))
        .sum();
    matches / max_len as f64
}
