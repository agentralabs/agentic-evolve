//! PatternComposer — composes multiple patterns into a single output.

use std::collections::HashMap;

use crate::types::error::{EvolveError, EvolveResult};
use crate::types::pattern::Pattern;

/// Result of composing patterns.
#[derive(Debug, Clone, serde::Serialize)]
pub struct CompositionResult {
    pub code: String,
    pub patterns_used: Vec<String>,
    pub coverage: f64,
    pub gaps: Vec<String>,
}

/// Composes multiple patterns together.
#[derive(Debug, Default)]
pub struct PatternComposer;

impl PatternComposer {
    pub fn new() -> Self {
        Self
    }

    pub fn compose(
        &self,
        patterns: &[&Pattern],
        bindings: &HashMap<String, String>,
        order: Option<&[usize]>,
    ) -> EvolveResult<CompositionResult> {
        if patterns.is_empty() {
            return Err(EvolveError::CompositionError("No patterns to compose".to_string()));
        }

        let ordered: Vec<&&Pattern> = match order {
            Some(indices) => indices.iter()
                .filter_map(|&i| patterns.get(i))
                .collect(),
            None => patterns.iter().collect(),
        };

        let mut code_parts = Vec::new();
        let mut patterns_used = Vec::new();
        let mut total_placeholders = 0;
        let mut bound_placeholders = 0;

        for pattern in &ordered {
            let mut rendered = pattern.template.clone();
            for (key, value) in bindings {
                let placeholder = format!("{{{{{key}}}}}");
                if rendered.contains(&placeholder) {
                    rendered = rendered.replace(&placeholder, value);
                    bound_placeholders += 1;
                }
            }
            total_placeholders += count_placeholders(&rendered) + bound_placeholders;
            code_parts.push(rendered);
            patterns_used.push(pattern.id.as_str().to_string());
        }

        let code = code_parts.join("\n\n");
        let gaps = find_unbound_placeholders(&code);
        let coverage = if total_placeholders == 0 {
            1.0
        } else {
            bound_placeholders as f64 / total_placeholders as f64
        };

        Ok(CompositionResult {
            code,
            patterns_used,
            coverage,
            gaps,
        })
    }
}

fn count_placeholders(template: &str) -> usize {
    let re = regex::Regex::new(r"\{\{\w+\}\}").unwrap();
    re.find_iter(template).count()
}

fn find_unbound_placeholders(code: &str) -> Vec<String> {
    let re = regex::Regex::new(r"\{\{(\w+)\}\}").unwrap();
    re.captures_iter(code)
        .map(|cap| cap[1].to_string())
        .collect()
}
