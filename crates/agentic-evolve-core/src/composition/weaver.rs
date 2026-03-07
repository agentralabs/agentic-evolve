//! IntegrationWeaver — weaves patterns together with proper integration.

use crate::types::error::EvolveResult;
use crate::types::pattern::Pattern;

/// Weaves multiple patterns into an integrated whole.
#[derive(Debug, Default)]
pub struct IntegrationWeaver;

impl IntegrationWeaver {
    pub fn new() -> Self {
        Self
    }

    pub fn weave(&self, patterns: &[&Pattern]) -> EvolveResult<WovenResult> {
        let mut imports = std::collections::HashSet::new();
        let mut body_parts = Vec::new();

        for pattern in patterns {
            // Extract imports from template
            for line in pattern.template.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("use ") || trimmed.starts_with("import ")
                    || trimmed.starts_with("from ") || trimmed.starts_with("#include")
                {
                    imports.insert(trimmed.to_string());
                }
            }
            body_parts.push(pattern.template.clone());
        }

        let mut code = String::new();

        // Imports first
        let mut sorted_imports: Vec<_> = imports.into_iter().collect();
        sorted_imports.sort();
        for imp in &sorted_imports {
            code.push_str(imp);
            code.push('\n');
        }
        if !sorted_imports.is_empty() {
            code.push('\n');
        }

        // Bodies
        for (i, part) in body_parts.iter().enumerate() {
            // Strip imports from body
            let body: String = part.lines()
                .filter(|l| {
                    let t = l.trim();
                    !t.starts_with("use ") && !t.starts_with("import ")
                        && !t.starts_with("from ") && !t.starts_with("#include")
                })
                .collect::<Vec<&str>>()
                .join("\n");
            code.push_str(body.trim());
            if i < body_parts.len() - 1 {
                code.push_str("\n\n");
            }
        }

        Ok(WovenResult {
            code,
            patterns_used: patterns.iter().map(|p| p.id.as_str().to_string()).collect(),
            import_count: sorted_imports.len(),
        })
    }
}

/// Result of weaving patterns together.
#[derive(Debug, Clone)]
pub struct WovenResult {
    pub code: String,
    pub patterns_used: Vec<String>,
    pub import_count: usize,
}
