//! GapFiller — fills gaps between composed patterns.

use crate::types::error::EvolveResult;

/// Fills gaps between composed patterns with glue code.
#[derive(Debug, Default)]
pub struct GapFiller;

impl GapFiller {
    pub fn new() -> Self {
        Self
    }

    pub fn fill_gaps(&self, code: &str, gaps: &[GapDescription]) -> EvolveResult<String> {
        let mut result = code.to_string();
        for gap in gaps {
            let placeholder = format!("/* GAP: {} */", gap.description);
            let filler = self.generate_filler(gap);
            result = result.replace(&placeholder, &filler);
        }
        Ok(result)
    }

    pub fn identify_gaps(&self, code: &str) -> Vec<GapDescription> {
        let mut gaps = Vec::new();
        let re = regex::Regex::new(r"/\* GAP: (.*?) \*/").unwrap();
        for (i, cap) in re.captures_iter(code).enumerate() {
            gaps.push(GapDescription {
                index: i,
                description: cap[1].to_string(),
                gap_type: GapType::Missing,
                context_before: String::new(),
                context_after: String::new(),
            });
        }
        gaps
    }

    fn generate_filler(&self, gap: &GapDescription) -> String {
        match gap.gap_type {
            GapType::TypeConversion => format!("// TODO: Convert type for {}", gap.description),
            GapType::ErrorHandling => format!(
                "// Error handling for {}\nreturn Err(\"unimplemented\".into());",
                gap.description
            ),
            GapType::Initialization => format!(
                "// Initialize {}\nlet {} = Default::default();",
                gap.description,
                gap.description.to_lowercase().replace(' ', "_")
            ),
            GapType::Missing => format!("// TODO: Implement {}", gap.description),
        }
    }
}

/// Description of a gap between patterns.
#[derive(Debug, Clone)]
pub struct GapDescription {
    pub index: usize,
    pub description: String,
    pub gap_type: GapType,
    pub context_before: String,
    pub context_after: String,
}

/// Type of gap.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GapType {
    TypeConversion,
    ErrorHandling,
    Initialization,
    Missing,
}
