//! TemplateGenerator — generates templates from code and detected variables.

use crate::types::pattern::PatternVariable;

/// Generates pattern templates by replacing variable parts with placeholders.
#[derive(Debug, Default)]
pub struct TemplateGenerator;

impl TemplateGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, code: &str, variables: &[PatternVariable]) -> String {
        let mut template = code.to_string();

        for var in variables {
            if let Some(default) = &var.default {
                if !default.is_empty() {
                    let placeholder = format!("{{{{{}}}}}", var.name);
                    template = template.replacen(default, &placeholder, 1);
                }
            }
        }

        template
    }

    pub fn apply_bindings(
        &self,
        template: &str,
        bindings: &std::collections::HashMap<String, String>,
    ) -> String {
        let mut result = template.to_string();
        for (key, value) in bindings {
            let placeholder = format!("{{{{{key}}}}}");
            result = result.replace(&placeholder, value);
        }
        result
    }

    pub fn extract_placeholders(&self, template: &str) -> Vec<String> {
        let re = regex::Regex::new(r"\{\{(\w+)\}\}").unwrap();
        re.captures_iter(template)
            .map(|cap| cap[1].to_string())
            .collect()
    }

    pub fn has_unbound_placeholders(&self, template: &str) -> bool {
        template.contains("{{") && template.contains("}}")
    }
}
