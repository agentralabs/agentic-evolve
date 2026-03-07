//! Pattern matching result types.

use serde::{Deserialize, Serialize};

use super::ids::PatternId;
use super::pattern::Pattern;

/// Score from a pattern match attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchScore {
    pub signature_score: f64,
    pub context_score: f64,
    pub semantic_score: f64,
    pub confidence_score: f64,
    pub combined: f64,
}

impl MatchScore {
    pub fn new(signature: f64, context: f64, semantic: f64, confidence: f64) -> Self {
        let combined = signature * 0.4 + context * 0.2 + semantic * 0.2 + confidence * 0.2;
        Self {
            signature_score: signature,
            context_score: context,
            semantic_score: semantic,
            confidence_score: confidence,
            combined,
        }
    }

    pub fn from_single(score: f64) -> Self {
        Self {
            signature_score: score,
            context_score: score,
            semantic_score: score,
            confidence_score: score,
            combined: score,
        }
    }
}

/// Context provided for pattern matching.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MatchContext {
    pub domain: Option<String>,
    pub surrounding_code: Option<String>,
    pub imports: Vec<String>,
    pub project_type: Option<String>,
    pub max_results: usize,
}

impl MatchContext {
    pub fn new() -> Self {
        Self {
            max_results: 10,
            ..Default::default()
        }
    }

    pub fn with_domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_string());
        self
    }

    pub fn with_surrounding_code(mut self, code: &str) -> Self {
        self.surrounding_code = Some(code.to_string());
        self
    }

    pub fn with_max_results(mut self, limit: usize) -> Self {
        self.max_results = limit;
        self
    }
}

/// Result of a pattern match.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub pattern_id: PatternId,
    pub pattern: Pattern,
    pub score: MatchScore,
    pub suggested_bindings: std::collections::HashMap<String, String>,
}
