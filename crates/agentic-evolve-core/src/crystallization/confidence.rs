//! ConfidenceCalculator — calculates pattern confidence from execution data.

use crate::types::skill::SuccessfulExecution;

/// Calculates confidence score for a pattern based on execution evidence.
#[derive(Debug)]
pub struct ConfidenceCalculator {
    pub test_weight: f64,
    pub execution_time_weight: f64,
    pub code_complexity_weight: f64,
}

impl ConfidenceCalculator {
    pub fn new() -> Self {
        Self {
            test_weight: 0.5,
            execution_time_weight: 0.2,
            code_complexity_weight: 0.3,
        }
    }

    pub fn calculate(&self, execution: &SuccessfulExecution) -> f64 {
        let test_score = self.test_score(execution);
        let time_score = self.time_score(execution);
        let complexity_score = self.complexity_score(&execution.code);

        let raw = test_score * self.test_weight
            + time_score * self.execution_time_weight
            + complexity_score * self.code_complexity_weight;

        raw.clamp(0.0, 1.0)
    }

    fn test_score(&self, execution: &SuccessfulExecution) -> f64 {
        if execution.test_results.is_empty() {
            return 0.5; // No tests = moderate confidence
        }
        let passed = execution.test_results.iter().filter(|t| t.passed).count();
        passed as f64 / execution.test_results.len() as f64
    }

    fn time_score(&self, execution: &SuccessfulExecution) -> f64 {
        // Faster execution = higher confidence (reasonable thresholds)
        match execution.execution_time_ms {
            0..=100 => 1.0,
            101..=500 => 0.9,
            501..=1000 => 0.8,
            1001..=5000 => 0.6,
            _ => 0.4,
        }
    }

    fn complexity_score(&self, code: &str) -> f64 {
        let lines = code.lines().count();
        let nesting = max_nesting_depth(code);

        // Sweet spot: moderate complexity
        let line_score = match lines {
            0..=5 => 0.9,
            6..=50 => 1.0,
            51..=200 => 0.8,
            _ => 0.6,
        };

        let nesting_score = match nesting {
            0..=2 => 1.0,
            3..=4 => 0.8,
            5..=6 => 0.6,
            _ => 0.4,
        };

        (line_score + nesting_score) / 2.0
    }
}

impl Default for ConfidenceCalculator {
    fn default() -> Self {
        Self::new()
    }
}

fn max_nesting_depth(code: &str) -> usize {
    let mut max_depth = 0;
    let mut current: usize = 0;
    for ch in code.chars() {
        match ch {
            '{' | '(' | '[' => {
                current += 1;
                max_depth = max_depth.max(current);
            }
            '}' | ')' | ']' => {
                current = current.saturating_sub(1);
            }
            _ => {}
        }
    }
    max_depth
}
