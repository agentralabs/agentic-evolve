---
status: stable
---

# Integration Guide

## Sister Integration

AgenticEvolve integrates with other Agentra sisters via bridge traits:

- **AgenticMemory** -- Store pattern decisions as memory nodes; retrieve historical pattern choices
- **AgenticCodebase** -- Scan codebase for crystallization candidates; track pattern coverage
- **AgenticVision** -- Visualize pattern relationships and composition graphs
- **AgenticIdentity** -- Associate patterns with agent identity and preferences

## Bridge Traits

All bridges use trait-based integration with NoOp defaults for standalone operation:

```rust
pub trait MemoryBridge: Send + Sync {
    fn record_pattern_use(&self, pattern_id: &str) -> Result<()>;
    fn recall_pattern_context(&self, query: &str) -> Result<Vec<String>>;
}
```

## MCP Integration

AgenticEvolve exposes 14 MCP tools that any MCP client can use. See [mcp-tools.md](mcp-tools.md) for the complete reference.

## Custom Integration

Use the FFI bindings for integration with non-Rust systems. See [ffi-reference.md](ffi-reference.md).
