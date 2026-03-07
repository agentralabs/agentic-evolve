//! Minimal FFI facade for AgenticEvolve.

pub fn agentic_evolve_ffi_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
