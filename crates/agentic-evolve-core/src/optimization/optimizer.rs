//! PatternOptimizer — optimizes pattern storage by deduplication and pruning.

use crate::types::pattern::Pattern;

/// Result of optimization.
#[derive(Debug, Clone)]
pub struct OptimizationReport {
    pub patterns_before: usize,
    pub patterns_after: usize,
    pub duplicates_removed: usize,
    pub pruned: usize,
    pub merged: usize,
    pub bytes_saved: usize,
}

/// Optimizes pattern storage.
#[derive(Debug, Default)]
pub struct PatternOptimizer;

impl PatternOptimizer {
    pub fn new() -> Self {
        Self
    }

    pub fn find_duplicates(&self, patterns: &[&Pattern]) -> Vec<(String, String)> {
        let mut duplicates = Vec::new();
        for i in 0..patterns.len() {
            for j in (i + 1)..patterns.len() {
                if patterns[i].content_hash == patterns[j].content_hash {
                    duplicates.push((
                        patterns[i].id.as_str().to_string(),
                        patterns[j].id.as_str().to_string(),
                    ));
                }
            }
        }
        duplicates
    }

    pub fn find_similar(
        &self,
        patterns: &[&Pattern],
        threshold: f64,
    ) -> Vec<(String, String, f64)> {
        let mut similar = Vec::new();
        for i in 0..patterns.len() {
            for j in (i + 1)..patterns.len() {
                let sim = template_similarity(&patterns[i].template, &patterns[j].template);
                if sim >= threshold && patterns[i].content_hash != patterns[j].content_hash {
                    similar.push((
                        patterns[i].id.as_str().to_string(),
                        patterns[j].id.as_str().to_string(),
                        sim,
                    ));
                }
            }
        }
        similar
    }

    pub fn suggest_pruning(
        &self,
        patterns: &[&Pattern],
        min_confidence: f64,
        min_uses: u64,
    ) -> Vec<String> {
        patterns
            .iter()
            .filter(|p| p.confidence < min_confidence && p.usage_count < min_uses)
            .map(|p| p.id.as_str().to_string())
            .collect()
    }

    pub fn optimize_report(&self, patterns: &[&Pattern]) -> OptimizationReport {
        let duplicates = self.find_duplicates(patterns);
        let prunable = self.suggest_pruning(patterns, 0.2, 2);

        OptimizationReport {
            patterns_before: patterns.len(),
            patterns_after: patterns.len() - duplicates.len() - prunable.len(),
            duplicates_removed: duplicates.len(),
            pruned: prunable.len(),
            merged: 0,
            bytes_saved: (duplicates.len() + prunable.len()) * 1024, // Estimate
        }
    }
}

fn template_similarity(a: &str, b: &str) -> f64 {
    let a_lines: Vec<&str> = a.lines().collect();
    let b_lines: Vec<&str> = b.lines().collect();
    let max_lines = a_lines.len().max(b_lines.len());
    if max_lines == 0 {
        return 1.0;
    }
    let matching = a_lines
        .iter()
        .zip(b_lines.iter())
        .filter(|(a, b)| a.trim() == b.trim())
        .count();
    matching as f64 / max_lines as f64
}
