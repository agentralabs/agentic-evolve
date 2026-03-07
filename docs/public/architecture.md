---
status: stable
---

# Architecture

## Workspace Structure

```
agentic-evolve/
  crates/
    agentic-evolve-core/   # Core library
    agentic-evolve-mcp/    # MCP server (14 tools)
    agentic-evolve-cli/    # CLI (38 commands)
    agentic-evolve-ffi/    # FFI bindings
  python/                  # Python package
  npm/wasm/                # WASM bindings
```

## Crate Responsibilities

### agentic-evolve-core

- Types: Pattern, Template, MatchResult, PatternId, Skill, Error
- Storage: PatternStore, Format (`.aevolve`), Index, Versioner
- Matching: Signature, Context, Semantic, Fuzzy, Composite
- Crystallization: Extractor, VariableDetector, TemplateGenerator, ConfidenceCalculator
- Composition: Composer, GapFiller, AdapterGenerator, IntegrationWeaver
- Collective: UsageTracker, SuccessTracker, DecayManager, PromotionEngine
- Optimization: Optimizer, CacheManager
- Bridges: trait-based with NoOp defaults

### agentic-evolve-mcp

- JSON-RPC 2.0 transport over stdio
- Content-Length framing with 8 MiB limit
- 14 registered tools
- Session management

### agentic-evolve-cli

- 38 commands organized into subcommand groups
- Clap-based argument parsing

### agentic-evolve-ffi

- C-compatible FFI surface
- Pattern store operations exposed as extern functions

## Data Flow

1. Source code enters via CLI or MCP
2. Crystallization engine extracts patterns
3. Patterns stored in `.aevolve` binary format
4. Matching engines find relevant patterns on query
5. Composition engine combines patterns on demand
6. Collective learning updates statistics continuously
