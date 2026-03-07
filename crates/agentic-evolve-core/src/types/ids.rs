//! Identity types for AgenticEvolve entities.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for any evolve entity.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EvolveId(pub String);

impl EvolveId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn from_string(s: &str) -> Self {
        Self(s.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for EvolveId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for EvolveId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a pattern.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PatternId(pub String);

impl PatternId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn from_string(s: &str) -> Self {
        Self(s.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for PatternId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PatternId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for a crystallized skill.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SkillId(pub String);

impl SkillId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    pub fn from_string(s: &str) -> Self {
        Self(s.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SkillId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SkillId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
