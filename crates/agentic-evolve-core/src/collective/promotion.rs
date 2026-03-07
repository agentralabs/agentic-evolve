//! PromotionEngine — promotes high-confidence patterns and demotes poor ones.

use crate::types::pattern::Pattern;

/// Configuration for promotion behavior.
#[derive(Debug, Clone)]
pub struct PromotionConfig {
    pub promote_threshold: f64,
    pub demote_threshold: f64,
    pub min_uses_for_promotion: u64,
    pub min_uses_for_demotion: u64,
}

impl Default for PromotionConfig {
    fn default() -> Self {
        Self {
            promote_threshold: 0.9,
            demote_threshold: 0.3,
            min_uses_for_promotion: 5,
            min_uses_for_demotion: 3,
        }
    }
}

/// Evaluates patterns for promotion or demotion.
#[derive(Debug)]
pub struct PromotionEngine {
    config: PromotionConfig,
}

/// Promotion decision for a pattern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromotionDecision {
    Promote,
    Demote,
    Maintain,
    Prune,
}

impl PromotionEngine {
    pub fn new(config: PromotionConfig) -> Self {
        Self { config }
    }

    pub fn evaluate(&self, pattern: &Pattern) -> PromotionDecision {
        let success_rate = pattern.success_rate();

        if pattern.usage_count >= self.config.min_uses_for_promotion
            && success_rate >= self.config.promote_threshold
        {
            return PromotionDecision::Promote;
        }

        if pattern.usage_count >= self.config.min_uses_for_demotion
            && success_rate < self.config.demote_threshold
        {
            if pattern.confidence < 0.1 {
                return PromotionDecision::Prune;
            }
            return PromotionDecision::Demote;
        }

        PromotionDecision::Maintain
    }

    pub fn apply_promotion(&self, pattern: &mut Pattern) -> PromotionDecision {
        let decision = self.evaluate(pattern);
        match decision {
            PromotionDecision::Promote => {
                pattern.confidence = (pattern.confidence + 0.1).min(1.0);
            }
            PromotionDecision::Demote => {
                pattern.confidence = (pattern.confidence - 0.1).max(0.0);
            }
            PromotionDecision::Prune => {
                pattern.confidence = 0.0;
            }
            PromotionDecision::Maintain => {}
        }
        decision
    }

    pub fn batch_evaluate(&self, patterns: &[&Pattern]) -> Vec<(String, PromotionDecision)> {
        patterns
            .iter()
            .map(|p| (p.id.as_str().to_string(), self.evaluate(p)))
            .collect()
    }
}

impl Default for PromotionEngine {
    fn default() -> Self {
        Self::new(PromotionConfig::default())
    }
}
