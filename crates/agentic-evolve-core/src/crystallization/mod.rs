//! Crystallization — extract patterns from successful code.

pub mod confidence;
pub mod extractor;
pub mod template_generator;
pub mod variable_detector;

pub use confidence::ConfidenceCalculator;
pub use extractor::PatternExtractor;
pub use template_generator::TemplateGenerator;
pub use variable_detector::VariableDetector;
