//! Crystallized skill types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::ids::{PatternId, SkillId};
use super::pattern::Language;

/// Metadata about a crystallized skill.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub domain: String,
    pub language: Language,
    pub complexity: Complexity,
    pub source: String,
}

/// Complexity level of a skill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Complexity {
    Simple,
    Medium,
    Complex,
}

/// A crystallized skill — a pattern instantiation that has been verified.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystallizedSkill {
    pub id: SkillId,
    pub pattern_id: PatternId,
    pub code: String,
    pub bindings: HashMap<String, String>,
    pub metadata: SkillMetadata,
    pub verified_count: u64,
    pub last_verified: i64,
    pub created_at: i64,
}

impl CrystallizedSkill {
    pub fn new(
        pattern_id: PatternId,
        code: &str,
        bindings: HashMap<String, String>,
        metadata: SkillMetadata,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: SkillId::new(),
            pattern_id,
            code: code.to_string(),
            bindings,
            metadata,
            verified_count: 1,
            last_verified: now,
            created_at: now,
        }
    }

    pub fn record_verification(&mut self) {
        self.verified_count += 1;
        self.last_verified = chrono::Utc::now().timestamp();
    }
}

/// Represents a successful code execution that can be crystallized into a pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessfulExecution {
    pub code: String,
    pub language: Language,
    pub domain: String,
    pub test_results: Vec<TestResult>,
    pub execution_time_ms: u64,
}

/// Result of a test run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub duration_ms: u64,
}
