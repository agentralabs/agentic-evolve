---
status: stable
---

# AgenticEvolve Overview

AgenticEvolve is the Pattern Library sister in the Agentra ecosystem. It crystallizes verified code patterns for reuse, providing up to 80% of function bodies from patterns on subsequent builds.

## Core Capabilities

- **Pattern Store** -- Persistent storage of verified code patterns in `.aevolve` binary format
- **Multi-Modal Matching** -- Signature, context, semantic, and fuzzy matching engines
- **Crystallization** -- Extract reusable templates from verified source code
- **Composition** -- Weave multiple patterns together with gap filling
- **Collective Learning** -- Usage tracking, success rates, decay, and promotion

## Architecture

AgenticEvolve is organized as a Cargo workspace with 4 crates:

- `agentic-evolve-core` -- Core library with types, storage, matching, crystallization, composition, collective learning, bridges, and optimization
- `agentic-evolve-mcp` -- MCP server with 14 tools
- `agentic-evolve-cli` -- CLI with 38 commands
- `agentic-evolve-ffi` -- FFI bindings for C/Python/Node interop
