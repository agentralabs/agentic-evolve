//! Session management — wraps all core state for the MCP server.

use std::collections::HashMap;
use std::path::Path;

use agentic_evolve_core::collective::{DecayManager, PromotionEngine, SuccessTracker, UsageTracker};
use agentic_evolve_core::composition::PatternComposer;
use agentic_evolve_core::crystallization::PatternExtractor;
use agentic_evolve_core::matching::CompositeMatcher;
use agentic_evolve_core::optimization::{CacheManager, PatternOptimizer};
use agentic_evolve_core::storage::{PatternIndex, PatternStore, PatternVersioner};
use agentic_evolve_core::types::match_result::{MatchContext, MatchResult};
use agentic_evolve_core::types::pattern::{FunctionSignature, Language, Pattern, PatternVariable};
use agentic_evolve_core::types::skill::SuccessfulExecution;
use agentic_evolve_core::EvolveResult;

/// SessionManager wraps all core state for the MCP server.
pub struct SessionManager {
    store: PatternStore,
    index: PatternIndex,
    versioner: PatternVersioner,
    matcher: CompositeMatcher,
    extractor: PatternExtractor,
    composer: PatternComposer,
    usage_tracker: UsageTracker,
    success_tracker: SuccessTracker,
    decay_manager: DecayManager,
    promotion_engine: PromotionEngine,
    optimizer: PatternOptimizer,
    cache: CacheManager,
}

impl SessionManager {
    /// Create a new session manager with optional persistent data directory.
    pub fn new(data_dir: &str) -> EvolveResult<Self> {
        let path = Path::new(data_dir);
        let store = PatternStore::with_data_dir(path)?;

        let mut index = PatternIndex::new();
        for pattern in store.list() {
            index.add(pattern);
        }

        Ok(Self {
            store,
            index,
            versioner: PatternVersioner::new(),
            matcher: CompositeMatcher::new(),
            extractor: PatternExtractor::new(),
            composer: PatternComposer::new(),
            usage_tracker: UsageTracker::new(),
            success_tracker: SuccessTracker::new(),
            decay_manager: DecayManager::default(),
            promotion_engine: PromotionEngine::default(),
            optimizer: PatternOptimizer::new(),
            cache: CacheManager::default(),
        })
    }

    // --- Storage operations ---

    /// Store a new pattern.
    #[allow(clippy::too_many_arguments)]
    pub fn store_pattern(
        &mut self,
        name: &str,
        domain: &str,
        language: Language,
        signature: FunctionSignature,
        template: &str,
        variables: Vec<PatternVariable>,
        confidence: f64,
        tags: Vec<String>,
    ) -> EvolveResult<Pattern> {
        let mut pattern = Pattern::new(name, domain, language, signature, template, variables, confidence);
        pattern.tags = tags;
        self.store.save(&pattern)?;
        self.index.add(&pattern);
        self.versioner.record_version(&pattern, "Initial creation")?;
        Ok(pattern)
    }

    /// Get a pattern by ID.
    pub fn get_pattern(&self, id: &str) -> EvolveResult<&Pattern> {
        self.store.get(id)
    }

    /// Search patterns by query.
    pub fn search_patterns(&self, query: &str) -> Vec<&Pattern> {
        self.store.search(query)
    }

    /// List all patterns.
    pub fn list_patterns(&self) -> Vec<&Pattern> {
        self.store.list()
    }

    /// Delete a pattern by ID.
    pub fn delete_pattern(&mut self, id: &str) -> EvolveResult<Pattern> {
        let pattern = self.store.delete(id)?;
        self.index.remove(&pattern);
        self.cache.invalidate_pattern(id);
        Ok(pattern)
    }

    // --- Matching operations ---

    /// Match a function signature against stored patterns.
    pub fn match_signature(
        &self,
        signature: &FunctionSignature,
        context: &MatchContext,
        limit: usize,
    ) -> EvolveResult<Vec<MatchResult>> {
        let patterns: Vec<&Pattern> = self.store.list();
        self.matcher.find_matches(signature, &patterns, context, limit)
    }

    /// Match patterns with surrounding context.
    pub fn match_context(
        &self,
        signature: &FunctionSignature,
        context: &MatchContext,
        limit: usize,
    ) -> EvolveResult<Vec<MatchResult>> {
        let patterns: Vec<&Pattern> = self.store.list();
        self.matcher.find_matches(signature, &patterns, context, limit)
    }

    // --- Crystallization ---

    /// Crystallize successful code into patterns.
    pub fn crystallize(&mut self, execution: &SuccessfulExecution) -> EvolveResult<Vec<Pattern>> {
        let patterns = self.extractor.extract(execution)?;
        let mut stored = Vec::new();
        for pattern in patterns {
            self.store.save(&pattern)?;
            self.index.add(&pattern);
            self.versioner.record_version(&pattern, "Crystallized from successful execution")?;
            stored.push(pattern);
        }
        Ok(stored)
    }

    // --- Composition ---

    /// Compose multiple patterns together.
    pub fn compose(
        &self,
        pattern_ids: &[String],
        bindings: &HashMap<String, String>,
    ) -> EvolveResult<agentic_evolve_core::composition::composer::CompositionResult> {
        let mut patterns = Vec::new();
        for id in pattern_ids {
            patterns.push(self.store.get(id)?);
        }
        self.composer.compose(&patterns, bindings, None)
    }

    // --- Coverage ---

    /// Get pattern coverage for a set of function signatures.
    pub fn coverage(
        &self,
        signatures: &[FunctionSignature],
        threshold: f64,
    ) -> CoverageReport {
        let all_patterns: Vec<&Pattern> = self.store.list();
        let context = MatchContext::new();
        let mut matched = 0;
        let mut details = Vec::new();

        for sig in signatures {
            let results = self.matcher.find_matches(sig, &all_patterns, &context, 1);
            let best_score = results
                .as_ref()
                .ok()
                .and_then(|r| r.first())
                .map(|r| r.score.combined)
                .unwrap_or(0.0);
            let covered = best_score >= threshold;
            if covered {
                matched += 1;
            }
            details.push(CoverageDetail {
                function_name: sig.name.clone(),
                best_match_score: best_score,
                covered,
            });
        }

        let total = signatures.len();
        CoverageReport {
            total,
            covered: matched,
            coverage: if total == 0 { 1.0 } else { matched as f64 / total as f64 },
            details,
        }
    }

    // --- Confidence ---

    /// Get confidence score for a pattern.
    pub fn confidence(&self, pattern_id: &str) -> EvolveResult<ConfidenceReport> {
        let pattern = self.store.get(pattern_id)?;
        let usage_rate = self.usage_tracker.success_rate(pattern_id);
        let success_rate = self.success_tracker.success_rate(pattern_id);
        let promotion = self.promotion_engine.evaluate(pattern);

        Ok(ConfidenceReport {
            pattern_id: pattern_id.to_string(),
            base_confidence: pattern.confidence,
            usage_success_rate: usage_rate,
            tracker_success_rate: success_rate,
            promotion_decision: format!("{promotion:?}"),
            usage_count: pattern.usage_count,
            success_count: pattern.success_count,
        })
    }

    // --- Usage tracking ---

    /// Update usage statistics for a pattern.
    pub fn update_usage(
        &mut self,
        pattern_id: &str,
        domain: &str,
        success: bool,
    ) -> EvolveResult<()> {
        let pattern = self.store.get_mut(pattern_id)?;
        pattern.record_use(success);
        self.decay_manager.apply_usage_boost(pattern, success);

        // Persist updates
        let pattern_clone = pattern.clone();
        self.store.save(&pattern_clone)?;

        self.usage_tracker.record_use(pattern_id, domain, success);
        self.success_tracker.record(pattern_id, success);
        self.cache.invalidate_pattern(pattern_id);
        Ok(())
    }

    // --- Optimization ---

    /// Optimize pattern storage.
    pub fn optimize(&mut self) -> EvolveResult<OptimizationSummary> {
        let patterns: Vec<&Pattern> = self.store.list();
        let report = self.optimizer.optimize_report(&patterns);
        let decay_report = self.decay_manager.decay_report(&patterns);

        // Apply decay to all patterns
        let ids: Vec<String> = self.store.list().iter().map(|p| p.id.as_str().to_string()).collect();
        let mut decayed = 0;
        for id in &ids {
            if let Ok(pattern) = self.store.get_mut(id) {
                let old = pattern.confidence;
                self.decay_manager.apply_decay(pattern);
                if (old - pattern.confidence).abs() > f64::EPSILON {
                    decayed += 1;
                }
            }
        }

        // Apply promotions
        let mut promoted = 0;
        let mut demoted = 0;
        for id in &ids {
            if let Ok(pattern) = self.store.get_mut(id) {
                let decision = self.promotion_engine.apply_promotion(pattern);
                match decision {
                    agentic_evolve_core::collective::promotion::PromotionDecision::Promote => promoted += 1,
                    agentic_evolve_core::collective::promotion::PromotionDecision::Demote => demoted += 1,
                    _ => {}
                }
            }
        }

        self.cache.clear();

        Ok(OptimizationSummary {
            patterns_total: report.patterns_before,
            duplicates_found: report.duplicates_removed,
            prunable: report.pruned,
            decay_healthy: decay_report.healthy,
            decay_decaying: decay_report.decaying,
            decay_critical: decay_report.critical,
            patterns_decayed: decayed,
            patterns_promoted: promoted,
            patterns_demoted: demoted,
            cache_cleared: true,
        })
    }

    /// Get the function body from the best matching pattern.
    pub fn get_body(
        &self,
        signature: &FunctionSignature,
        context: &MatchContext,
    ) -> EvolveResult<Option<(String, String, f64)>> {
        let patterns: Vec<&Pattern> = self.store.list();
        let results = self.matcher.find_matches(signature, &patterns, context, 1)?;
        match results.into_iter().next() {
            Some(result) => Ok(Some((
                result.pattern.template.clone(),
                result.pattern_id.as_str().to_string(),
                result.score.combined,
            ))),
            None => Ok(None),
        }
    }

    /// Total pattern count.
    pub fn pattern_count(&self) -> usize {
        self.store.count()
    }
}

/// Coverage report for a set of function signatures.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoverageReport {
    pub total: usize,
    pub covered: usize,
    pub coverage: f64,
    pub details: Vec<CoverageDetail>,
}

/// Coverage detail for a single function.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CoverageDetail {
    pub function_name: String,
    pub best_match_score: f64,
    pub covered: bool,
}

/// Confidence report for a pattern.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ConfidenceReport {
    pub pattern_id: String,
    pub base_confidence: f64,
    pub usage_success_rate: f64,
    pub tracker_success_rate: f64,
    pub promotion_decision: String,
    pub usage_count: u64,
    pub success_count: u64,
}

/// Optimization summary.
#[derive(Debug, Clone, serde::Serialize)]
pub struct OptimizationSummary {
    pub patterns_total: usize,
    pub duplicates_found: usize,
    pub prunable: usize,
    pub decay_healthy: usize,
    pub decay_decaying: usize,
    pub decay_critical: usize,
    pub patterns_decayed: usize,
    pub patterns_promoted: usize,
    pub patterns_demoted: usize,
    pub cache_cleared: bool,
}
