//! ContextMatcher — matches based on surrounding code context.

use crate::types::error::EvolveResult;
use crate::types::match_result::{MatchContext, MatchResult, MatchScore};
use crate::types::pattern::{FunctionSignature, Pattern};

/// Matches patterns based on surrounding code context.
#[derive(Debug, Default)]
pub struct ContextMatcher;

impl ContextMatcher {
    pub fn new() -> Self {
        Self
    }

    pub fn find_matches(
        &self,
        _signature: &FunctionSignature,
        patterns: &[&Pattern],
        context: &MatchContext,
        limit: usize,
    ) -> EvolveResult<Vec<MatchResult>> {
        let mut results: Vec<MatchResult> = patterns
            .iter()
            .map(|p| {
                let score = self.score_context(p, context);
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

    pub fn score_context(&self, pattern: &Pattern, context: &MatchContext) -> f64 {
        let mut score = 0.0;
        let mut factors = 0;

        // Domain match
        if let Some(domain) = &context.domain {
            if pattern.domain.to_lowercase() == domain.to_lowercase() {
                score += 1.0;
            } else if pattern
                .domain
                .to_lowercase()
                .contains(&domain.to_lowercase())
            {
                score += 0.5;
            }
            factors += 1;
        }

        // Import overlap
        if !context.imports.is_empty() {
            let template_lower = pattern.template.to_lowercase();
            let import_matches = context
                .imports
                .iter()
                .filter(|imp| template_lower.contains(&imp.to_lowercase()))
                .count();
            if !context.imports.is_empty() {
                score += import_matches as f64 / context.imports.len() as f64;
            }
            factors += 1;
        }

        // Surrounding code similarity
        if let Some(surrounding) = &context.surrounding_code {
            let words: std::collections::HashSet<&str> = surrounding.split_whitespace().collect();
            let template_words: std::collections::HashSet<&str> =
                pattern.template.split_whitespace().collect();
            let overlap = words.intersection(&template_words).count();
            let total = words.len().max(1);
            score += overlap as f64 / total as f64;
            factors += 1;
        }

        if factors > 0 {
            score / factors as f64
        } else {
            0.3 // Default context score when no context provided
        }
    }
}
