//! PatternExtractor — extracts patterns from successful code.

use crate::types::error::{EvolveError, EvolveResult};
use crate::types::pattern::{FunctionSignature, Language, ParamSignature, Pattern, Visibility};
use crate::types::skill::SuccessfulExecution;

use super::confidence::ConfidenceCalculator;
use super::template_generator::TemplateGenerator;
use super::variable_detector::VariableDetector;

/// Extracts reusable patterns from successfully executed code.
#[derive(Debug, Default)]
pub struct PatternExtractor {
    variable_detector: VariableDetector,
    template_generator: TemplateGenerator,
    confidence_calculator: ConfidenceCalculator,
}

impl PatternExtractor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extract(&self, execution: &SuccessfulExecution) -> EvolveResult<Vec<Pattern>> {
        let functions = self.extract_functions(&execution.code, &execution.language)?;
        let mut patterns = Vec::new();

        for func in functions {
            let variables = self
                .variable_detector
                .detect(&func.body, &execution.language);
            let template = self.template_generator.generate(&func.body, &variables);
            let confidence = self.confidence_calculator.calculate(execution);

            if confidence >= 0.5 {
                let pattern = Pattern::new(
                    &func.name,
                    &execution.domain,
                    execution.language.clone(),
                    FunctionSignature {
                        name: func.name.clone(),
                        params: func.params.clone(),
                        return_type: func.return_type.clone(),
                        language: execution.language.clone(),
                        is_async: func.is_async,
                        visibility: func.visibility.clone(),
                    },
                    &template,
                    variables,
                    confidence,
                );
                patterns.push(pattern);
            }
        }

        Ok(patterns)
    }

    fn extract_functions(
        &self,
        code: &str,
        language: &Language,
    ) -> EvolveResult<Vec<ExtractedFunction>> {
        match language {
            Language::Rust => self.extract_rust_functions(code),
            Language::Python => self.extract_python_functions(code),
            _ => self.extract_generic_functions(code),
        }
    }

    fn extract_rust_functions(&self, code: &str) -> EvolveResult<Vec<ExtractedFunction>> {
        let mut functions = Vec::new();
        let re = regex::Regex::new(
            r"(?m)^(\s*)(pub\s+)?(async\s+)?fn\s+(\w+)\s*(\([^)]*\))\s*(->\s*[^{]+)?\s*\{",
        )
        .map_err(|e| EvolveError::CrystallizationError(e.to_string()))?;

        for cap in re.captures_iter(code) {
            let is_pub = cap.get(2).is_some();
            let is_async = cap.get(3).is_some();
            let name = cap[4].to_string();
            let params_str = &cap[5];
            let return_type = cap
                .get(6)
                .map(|m| m.as_str().trim_start_matches("->").trim().to_string());

            // Extract body (simple brace counting)
            let fn_start = cap.get(0).map(|m| m.end()).unwrap_or(0);
            let body = extract_braced_body(code, fn_start);

            let params = parse_rust_params(params_str);

            functions.push(ExtractedFunction {
                name,
                params,
                return_type,
                body,
                is_async,
                visibility: if is_pub {
                    Visibility::Public
                } else {
                    Visibility::Private
                },
            });
        }

        Ok(functions)
    }

    fn extract_python_functions(&self, code: &str) -> EvolveResult<Vec<ExtractedFunction>> {
        let mut functions = Vec::new();
        let re = regex::Regex::new(
            r"(?m)^(\s*)(async\s+)?def\s+(\w+)\s*\(([^)]*)\)\s*(?:->\s*([^:]+))?\s*:",
        )
        .map_err(|e| EvolveError::CrystallizationError(e.to_string()))?;

        for cap in re.captures_iter(code) {
            let indent = cap[1].len();
            let is_async = cap.get(2).is_some();
            let name = cap[3].to_string();
            let params_str = &cap[4];
            let return_type = cap.get(5).map(|m| m.as_str().trim().to_string());

            let fn_end = cap.get(0).map(|m| m.end()).unwrap_or(0);
            let body = extract_indented_body(code, fn_end, indent);

            let params = parse_python_params(params_str);

            functions.push(ExtractedFunction {
                name,
                params,
                return_type,
                body,
                is_async,
                visibility: Visibility::Public,
            });
        }

        Ok(functions)
    }

    fn extract_generic_functions(&self, code: &str) -> EvolveResult<Vec<ExtractedFunction>> {
        // Generic: treat entire code as one function body
        Ok(vec![ExtractedFunction {
            name: "main".to_string(),
            params: Vec::new(),
            return_type: None,
            body: code.to_string(),
            is_async: false,
            visibility: Visibility::Public,
        }])
    }
}

#[derive(Debug, Clone)]
struct ExtractedFunction {
    name: String,
    params: Vec<ParamSignature>,
    return_type: Option<String>,
    body: String,
    is_async: bool,
    visibility: Visibility,
}

fn extract_braced_body(code: &str, start: usize) -> String {
    let mut depth = 1;
    let mut end = start;
    for (i, ch) in code[start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end = start + i;
                    break;
                }
            }
            _ => {}
        }
    }
    code[start..end].trim().to_string()
}

fn extract_indented_body(code: &str, start: usize, base_indent: usize) -> String {
    let mut lines = Vec::new();
    for line in code[start..].lines() {
        if line.trim().is_empty() {
            lines.push(String::new());
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        if indent > base_indent {
            lines.push(line.to_string());
        } else if !lines.is_empty() {
            break;
        }
    }
    lines.join("\n").trim().to_string()
}

fn parse_rust_params(params_str: &str) -> Vec<ParamSignature> {
    let inner = params_str.trim_start_matches('(').trim_end_matches(')');
    inner
        .split(',')
        .filter_map(|p| {
            let p = p.trim();
            if p.is_empty() || p == "&self" || p == "&mut self" || p == "self" {
                return None;
            }
            let parts: Vec<&str> = p.splitn(2, ':').collect();
            if parts.len() == 2 {
                Some(ParamSignature {
                    name: parts[0].trim().to_string(),
                    param_type: parts[1].trim().to_string(),
                    is_optional: parts[1].contains("Option"),
                })
            } else {
                None
            }
        })
        .collect()
}

fn parse_python_params(params_str: &str) -> Vec<ParamSignature> {
    params_str
        .split(',')
        .filter_map(|p| {
            let p = p.trim();
            if p.is_empty() || p == "self" || p == "cls" {
                return None;
            }
            let parts: Vec<&str> = p.splitn(2, ':').collect();
            let name = parts[0].trim().to_string();
            let param_type = if parts.len() > 1 {
                parts[1]
                    .split('=')
                    .next()
                    .unwrap_or("Any")
                    .trim()
                    .to_string()
            } else {
                "Any".to_string()
            };
            let is_optional = p.contains('=');
            Some(ParamSignature {
                name,
                param_type,
                is_optional,
            })
        })
        .collect()
}
