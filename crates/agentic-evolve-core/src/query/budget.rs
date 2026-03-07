//! Token budget — track and enforce a per-request token spending limit.

use serde::{Deserialize, Serialize};

/// A token budget that tracks spending against a maximum allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBudget {
    max_tokens: u64,
    used_tokens: u64,
}

impl TokenBudget {
    /// Create a new budget with the given maximum.
    pub fn new(max_tokens: u64) -> Self {
        Self {
            max_tokens,
            used_tokens: 0,
        }
    }

    /// Maximum tokens allowed.
    pub fn max_tokens(&self) -> u64 {
        self.max_tokens
    }

    /// Tokens consumed so far.
    pub fn used_tokens(&self) -> u64 {
        self.used_tokens
    }

    /// Spend tokens from the budget. Returns `true` if the budget was
    /// sufficient, `false` if it would exceed the limit (in which case
    /// the spend is still applied to track overruns).
    pub fn spend(&mut self, tokens: u64) -> bool {
        self.used_tokens += tokens;
        self.used_tokens <= self.max_tokens
    }

    /// Remaining tokens before the budget is exhausted.
    pub fn remaining(&self) -> u64 {
        self.max_tokens.saturating_sub(self.used_tokens)
    }

    /// Whether the budget is fully consumed.
    pub fn is_exhausted(&self) -> bool {
        self.used_tokens >= self.max_tokens
    }

    /// Check if a given cost can be afforded without exceeding the budget.
    pub fn can_afford(&self, cost: u64) -> bool {
        self.used_tokens + cost <= self.max_tokens
    }

    /// Utilization ratio in `[0.0, 1.0+]` (can exceed 1.0 if overspent).
    pub fn utilization(&self) -> f64 {
        if self.max_tokens == 0 {
            return if self.used_tokens == 0 {
                0.0
            } else {
                f64::INFINITY
            };
        }
        self.used_tokens as f64 / self.max_tokens as f64
    }

    /// Reset the budget to zero usage.
    pub fn reset(&mut self) {
        self.used_tokens = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_budget_is_empty() {
        let b = TokenBudget::new(100);
        assert_eq!(b.used_tokens(), 0);
        assert_eq!(b.remaining(), 100);
        assert!(!b.is_exhausted());
    }

    #[test]
    fn spend_reduces_remaining() {
        let mut b = TokenBudget::new(100);
        assert!(b.spend(40));
        assert_eq!(b.remaining(), 60);
        assert_eq!(b.used_tokens(), 40);
    }

    #[test]
    fn spend_returns_false_when_exceeding() {
        let mut b = TokenBudget::new(10);
        assert!(!b.spend(20));
        assert!(b.is_exhausted());
    }

    #[test]
    fn can_afford_check() {
        let mut b = TokenBudget::new(100);
        assert!(b.can_afford(100));
        assert!(!b.can_afford(101));
        b.spend(50);
        assert!(b.can_afford(50));
        assert!(!b.can_afford(51));
    }

    #[test]
    fn utilization_tracks_ratio() {
        let mut b = TokenBudget::new(100);
        assert_eq!(b.utilization(), 0.0);
        b.spend(50);
        assert!((b.utilization() - 0.5).abs() < f64::EPSILON);
        b.spend(50);
        assert!((b.utilization() - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn reset_clears_usage() {
        let mut b = TokenBudget::new(100);
        b.spend(80);
        b.reset();
        assert_eq!(b.used_tokens(), 0);
        assert_eq!(b.remaining(), 100);
    }

    #[test]
    fn zero_budget_exhausted() {
        let b = TokenBudget::new(0);
        assert!(b.is_exhausted());
        assert!(!b.can_afford(1));
    }
}
