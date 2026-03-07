//! Composition — combine multiple patterns and fill gaps.

pub mod adapter;
pub mod composer;
pub mod gap_filler;
pub mod weaver;

pub use adapter::AdapterGenerator;
pub use composer::PatternComposer;
pub use gap_filler::GapFiller;
pub use weaver::IntegrationWeaver;
