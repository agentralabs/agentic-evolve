//! VariableDetector — detects variable elements in code.

use crate::types::pattern::{Language, PatternVariable};

/// Detects which parts of code are variable (change between uses).
#[derive(Debug, Default)]
pub struct VariableDetector;

impl VariableDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn detect(&self, code: &str, language: &Language) -> Vec<PatternVariable> {
        let mut variables = Vec::new();

        // Detect string literals
        let string_re = regex::Regex::new(r#""([^"]{3,})""#).unwrap();
        for (i, cap) in string_re.captures_iter(code).enumerate() {
            variables.push(PatternVariable {
                name: format!("STRING_{}", i),
                var_type: "string".to_string(),
                pattern: None,
                default: Some(cap[1].to_string()),
            });
        }

        // Detect numeric literals
        let num_re = regex::Regex::new(r"\b(\d{2,})\b").unwrap();
        for (i, cap) in num_re.captures_iter(code).enumerate() {
            variables.push(PatternVariable {
                name: format!("NUMBER_{}", i),
                var_type: "number".to_string(),
                pattern: Some(r"\d+".to_string()),
                default: Some(cap[1].to_string()),
            });
        }

        // Detect type names (language-specific)
        match language {
            Language::Rust => {
                let type_re = regex::Regex::new(r"\b([A-Z][a-zA-Z0-9]+)\b").unwrap();
                let mut seen = std::collections::HashSet::new();
                for cap in type_re.captures_iter(code) {
                    let name = &cap[1];
                    if !is_common_type(name) && seen.insert(name.to_string()) {
                        variables.push(PatternVariable {
                            name: format!("TYPE_{name}"),
                            var_type: "type".to_string(),
                            pattern: Some(r"[A-Z]\w+".to_string()),
                            default: Some(name.to_string()),
                        });
                    }
                }
            }
            Language::Python => {
                let type_re = regex::Regex::new(r"\b([A-Z][a-zA-Z0-9]+)\b").unwrap();
                let mut seen = std::collections::HashSet::new();
                for cap in type_re.captures_iter(code) {
                    let name = &cap[1];
                    if !is_common_python_type(name) && seen.insert(name.to_string()) {
                        variables.push(PatternVariable {
                            name: format!("TYPE_{name}"),
                            var_type: "type".to_string(),
                            pattern: Some(r"[A-Z]\w+".to_string()),
                            default: Some(name.to_string()),
                        });
                    }
                }
            }
            _ => {}
        }

        variables
    }
}

fn is_common_type(name: &str) -> bool {
    matches!(
        name,
        "String"
            | "Vec"
            | "HashMap"
            | "HashSet"
            | "Option"
            | "Result"
            | "Box"
            | "Arc"
            | "Rc"
            | "Mutex"
            | "RwLock"
            | "Cell"
            | "RefCell"
            | "Self"
            | "Ok"
            | "Err"
            | "Some"
            | "None"
            | "Default"
            | "Debug"
            | "Clone"
            | "Copy"
            | "Send"
            | "Sync"
            | "Display"
            | "Error"
            | "Serialize"
            | "Deserialize"
            | "Value"
            | "Path"
            | "PathBuf"
    )
}

fn is_common_python_type(name: &str) -> bool {
    matches!(
        name,
        "True"
            | "False"
            | "None"
            | "List"
            | "Dict"
            | "Set"
            | "Tuple"
            | "Optional"
            | "Union"
            | "Any"
            | "Type"
            | "Callable"
            | "Iterator"
            | "Exception"
            | "ValueError"
            | "TypeError"
            | "KeyError"
    )
}
