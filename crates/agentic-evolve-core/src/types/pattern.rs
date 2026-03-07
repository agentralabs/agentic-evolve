//! Pattern types — the core data model for stored patterns.

use serde::{Deserialize, Serialize};

use super::ids::PatternId;

/// Programming language of a pattern.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    JavaScript,
    Go,
    Java,
    CSharp,
    Cpp,
    C,
    Shell,
    Other(String),
}

impl Language {
    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "rust" | "rs" => Self::Rust,
            "python" | "py" => Self::Python,
            "typescript" | "ts" => Self::TypeScript,
            "javascript" | "js" => Self::JavaScript,
            "go" | "golang" => Self::Go,
            "java" => Self::Java,
            "csharp" | "c#" | "cs" => Self::CSharp,
            "cpp" | "c++" => Self::Cpp,
            "c" => Self::C,
            "shell" | "bash" | "sh" | "zsh" => Self::Shell,
            other => Self::Other(other.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Rust => "rust",
            Self::Python => "python",
            Self::TypeScript => "typescript",
            Self::JavaScript => "javascript",
            Self::Go => "go",
            Self::Java => "java",
            Self::CSharp => "csharp",
            Self::Cpp => "cpp",
            Self::C => "c",
            Self::Shell => "shell",
            Self::Other(s) => s.as_str(),
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Function signature for pattern matching.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub name: String,
    pub params: Vec<ParamSignature>,
    pub return_type: Option<String>,
    pub language: Language,
    pub is_async: bool,
    pub visibility: Visibility,
}

impl FunctionSignature {
    pub fn new(name: &str, language: Language) -> Self {
        Self {
            name: name.to_string(),
            params: Vec::new(),
            return_type: None,
            language,
            is_async: false,
            visibility: Visibility::Public,
        }
    }
}

/// Parameter in a function signature.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamSignature {
    pub name: String,
    pub param_type: String,
    pub is_optional: bool,
}

/// Visibility of a function.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}

/// A variable element in a pattern template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternVariable {
    pub name: String,
    pub var_type: String,
    pub pattern: Option<String>,
    pub default: Option<String>,
}

/// A stored pattern — the core entity of AgenticEvolve.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub id: PatternId,
    pub name: String,
    pub domain: String,
    pub language: Language,
    pub signature: FunctionSignature,
    pub template: String,
    pub variables: Vec<PatternVariable>,
    pub confidence: f64,
    pub usage_count: u64,
    pub success_count: u64,
    pub version: u32,
    pub tags: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub last_used: i64,
    pub content_hash: String,
}

impl Pattern {
    pub fn new(
        name: &str,
        domain: &str,
        language: Language,
        signature: FunctionSignature,
        template: &str,
        variables: Vec<PatternVariable>,
        confidence: f64,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        let content_hash = blake3::hash(template.as_bytes()).to_hex().to_string();
        Self {
            id: PatternId::new(),
            name: name.to_string(),
            domain: domain.to_string(),
            language,
            signature,
            template: template.to_string(),
            variables,
            confidence,
            usage_count: 0,
            success_count: 0,
            version: 1,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
            last_used: now,
            content_hash,
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.usage_count == 0 {
            return 0.0;
        }
        self.success_count as f64 / self.usage_count as f64
    }

    pub fn record_use(&mut self, success: bool) {
        self.usage_count += 1;
        if success {
            self.success_count += 1;
        }
        self.last_used = chrono::Utc::now().timestamp();
    }
}
