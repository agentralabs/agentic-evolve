//! Error types for AgenticEvolve.

/// All errors that can occur in AgenticEvolve.
#[derive(thiserror::Error, Debug)]
pub enum EvolveError {
    #[error("Pattern not found: {0}")]
    PatternNotFound(String),

    #[error("Skill not found: {0}")]
    SkillNotFound(String),

    #[error("Invalid pattern: {0}")]
    InvalidPattern(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Matching error: {0}")]
    MatchingError(String),

    #[error("Crystallization error: {0}")]
    CrystallizationError(String),

    #[error("Composition error: {0}")]
    CompositionError(String),

    #[error("Template error: {0}")]
    TemplateError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Convenience result type.
pub type EvolveResult<T> = Result<T, EvolveError>;
