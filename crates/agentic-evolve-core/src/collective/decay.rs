//! DecayManager — manages confidence decay of unused patterns.

use crate::types::pattern::Pattern;

/// Configuration for decay behavior.
#[derive(Debug, Clone)]
pub struct DecayConfig {
    pub half_life_days: f64,
    pub min_confidence: f64,
    pub usage_boost: f64,
    pub success_boost: f64,
}

impl Default for DecayConfig {
    fn default() -> Self {
        Self {
            half_life_days: 30.0,
            min_confidence: 0.1,
            usage_boost: 0.05,
            success_boost: 0.1,
        }
    }
}

/// Manages confidence decay of patterns over time.
#[derive(Debug)]
pub struct DecayManager {
    config: DecayConfig,
}

impl DecayManager {
    pub fn new(config: DecayConfig) -> Self {
        Self { config }
    }

    pub fn apply_decay(&self, pattern: &mut Pattern) -> f64 {
        let now = chrono::Utc::now().timestamp();
        let days_since_use = (now - pattern.last_used) as f64 / 86400.0;

        if days_since_use <= 0.0 {
            return pattern.confidence;
        }

        let decay_factor = 0.5_f64.powf(days_since_use / self.config.half_life_days);
        let new_confidence = (pattern.confidence * decay_factor).max(self.config.min_confidence);
        pattern.confidence = new_confidence;
        new_confidence
    }

    pub fn apply_usage_boost(&self, pattern: &mut Pattern, success: bool) -> f64 {
        let boost = if success {
            self.config.usage_boost + self.config.success_boost
        } else {
            self.config.usage_boost
        };
        pattern.confidence = (pattern.confidence + boost).min(1.0);
        pattern.confidence
    }

    pub fn should_prune(&self, pattern: &Pattern) -> bool {
        let now = chrono::Utc::now().timestamp();
        let days_since_use = (now - pattern.last_used) as f64 / 86400.0;
        pattern.confidence <= self.config.min_confidence
            && days_since_use > self.config.half_life_days * 3.0
            && pattern.usage_count < 3
    }

    pub fn decay_report(&self, patterns: &[&Pattern]) -> DecayReport {
        let mut healthy = 0;
        let mut decaying = 0;
        let mut critical = 0;
        let mut prunable = 0;

        for pattern in patterns {
            if pattern.confidence > 0.7 {
                healthy += 1;
            } else if pattern.confidence > 0.3 {
                decaying += 1;
            } else {
                critical += 1;
            }
            if self.should_prune(pattern) {
                prunable += 1;
            }
        }

        DecayReport {
            total: patterns.len(),
            healthy,
            decaying,
            critical,
            prunable,
        }
    }
}

impl Default for DecayManager {
    fn default() -> Self {
        Self::new(DecayConfig::default())
    }
}

/// Report on pattern decay status.
#[derive(Debug, Clone)]
pub struct DecayReport {
    pub total: usize,
    pub healthy: usize,
    pub decaying: usize,
    pub critical: usize,
    pub prunable: usize,
}
