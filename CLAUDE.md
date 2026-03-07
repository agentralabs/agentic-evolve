# AgenticEvolve - Claude Code Instructions

## Overview

AgenticEvolve is the Pattern Library sister (#12) in the Agentra ecosystem. It crystallizes verified code patterns for reuse, providing ~80% of function bodies from patterns on subsequent builds.

## Workspace Structure

```
crates/
  agentic-evolve-core/   # Core library: types, storage, matching, crystallization, composition, collective, bridges, optimization
  agentic-evolve-mcp/    # MCP server with 14 tools
  agentic-evolve-cli/    # CLI with 38 commands
  agentic-evolve-ffi/    # FFI bindings (minimal)
```

## Key Conventions

- **Commit style**: conventional prefixes (feat:, fix:, chore:, docs:), NEVER add Co-Authored-By: Claude
- **Zero unwrap()**: All MCP code uses proper error handling
- **MCP Quality**: verb-first imperative descriptions, no trailing periods, -32803 for unknown tool
- **Tests**: phase[N]_*.rs incremental pattern in tests/ directories
- **Bridges**: trait-based with NoOp defaults for standalone operation

## MCP Tools (14)

evolve_pattern_store, evolve_pattern_get, evolve_pattern_search, evolve_pattern_list,
evolve_pattern_delete, evolve_match_signature, evolve_match_context, evolve_crystallize,
evolve_get_body, evolve_compose, evolve_coverage, evolve_confidence, evolve_update_usage,
evolve_optimize

## CLI Commands (38)

pattern (store/get/search/list/delete/export/import/count/tags/domains),
match (signature/context/semantic/fuzzy), crystallize, body (get/preview),
compose (run/preview), coverage (file/summary), stats (overview/patterns/usage/success/decay),
usage (update/top/bottom), promote, decay, optimize (full/duplicates/prune),
serve, info, version

## Testing

```bash
cargo test --workspace           # 321+ tests
cargo clippy --workspace -- -D warnings  # 0 warnings
```

## 22 Inventions (5 Tiers + 2)

Tier 1: Pattern Store, Pattern Index, Pattern Versioner, Pattern Validator
Tier 2: Signature Matcher, Context Matcher, Semantic Matcher, Fuzzy Matcher
Tier 3: Pattern Extractor, Variable Detector, Template Generator, Confidence Calculator
Tier 4: Pattern Composer, Gap Filler, Adapter Generator, Integration Weaver
Tier 5: Usage Tracker, Success Tracker, Decay Manager, Promotion Engine
Tier 6: Pattern Optimizer, Cache Manager
