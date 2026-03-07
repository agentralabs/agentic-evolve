//! Transport layer — I/O for stdio.

pub mod framing;
pub mod stdio;

pub use stdio::StdioTransport;
