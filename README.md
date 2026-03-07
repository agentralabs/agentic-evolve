<p align="center">
  <img src="assets/github-hero-pane.svg" alt="AgenticEvolve hero" width="980">
</p>

<p align="center">
  <a href="https://crates.io/crates/agentic-evolve-core"><img src="https://img.shields.io/crates/v/agentic-evolve-core.svg" alt="crates.io"></a>
  <img src="https://img.shields.io/badge/tests-448%20passing-brightgreen.svg" alt="Tests">
  <img src="https://img.shields.io/badge/clients-Claude%20%7C%20Cursor%20%7C%20Windsurf%20%7C%20Cody-orange.svg" alt="Multi-client">
</p>

<p align="center">
  <a href="#install"><img src="https://img.shields.io/badge/cargo_install-agentic--evolve--cli-F59E0B?style=for-the-badge&logo=rust&logoColor=white" alt="cargo install"></a>
  <a href="#mcp-server"><img src="https://img.shields.io/badge/MCP_Server-agentic--evolve--mcp-10B981?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyNCIgaGVpZ2h0PSIyNCIgdmlld0JveD0iMCAwIDI0IDI0IiBmaWxsPSJub25lIiBzdHJva2U9IndoaXRlIiBzdHJva2Utd2lkdGg9IjIiPjxwYXRoIGQ9Ik0xMiAydjIwTTIgMTJoMjAiLz48L3N2Zz4=&logoColor=white" alt="MCP Server"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-22C55E?style=for-the-badge" alt="MIT License"></a>
  <a href="docs/public/concepts.md"><img src="https://img.shields.io/badge/Capabilities-22-8B5CF6?style=for-the-badge" alt="22 Capabilities"></a>
  <a href="docs/public/api-reference.md"><img src="https://img.shields.io/badge/format-.evolve-3B82F6?style=for-the-badge" alt=".evolve format"></a>
</p>

<p align="center">
  <strong>The Library That Remembers How to Build</strong>
</p>

<p align="center">
  <em>Pattern crystallization for AI agents -- verified code patterns stored, matched, and composed so the hundredth build takes seconds, not minutes.</em>
</p>

<p align="center">
  <a href="#quickstart">Quickstart</a> · <a href="#problems-solved">Problems Solved</a> · <a href="#how-it-works">How It Works</a> · <a href="#capabilities">Capabilities</a> · <a href="#mcp-tools">MCP Tools</a> · <a href="#benchmarks">Benchmarks</a> · <a href="#install">Install</a> · <a href="docs/public/api-reference.md">API</a> · <a href="docs/public/concepts.md">Concepts</a>
</p>

---

> Sister in the Agentra ecosystem | `.evolve` format | 22 Capabilities | 14 MCP Tools | 38 CLI Commands

<p align="center">
  <img src="assets/github-terminal-pane.svg" alt="AgenticEvolve terminal demo" width="980">
</p>

## Why AgenticEvolve

Every AI agent writes the same code from scratch every time. It generates a REST handler for the tenth time as if it has never seen one before. It writes the same error handling boilerplate, the same pagination logic, the same auth middleware -- from zero, every time, with no memory of what worked before.

The current fixes do not work. Template engines produce rigid output that breaks when requirements deviate. Snippet libraries store text, not patterns -- they cannot adapt to new variable names, types, or structures. Fine-tuning is expensive, slow, and locked to one model.

**Current AI:** Cold start on every build. No reuse. No learning. No acceleration.
**AgenticEvolve:** Crystallizes verified patterns from successful builds and matches them to new contexts, so the first build takes a minute and the hundredth takes fifteen seconds.

<a name="quickstart"></a>

## Quickstart

```bash
cargo install agentic-evolve-cli
evolve --help
```

<a name="problems-solved"></a>

## Problems Solved (Read This First)

- **Problem:** AI agents regenerate identical patterns from scratch on every project.
  **Solved:** pattern crystallization extracts reusable patterns from verified code, indexed by signature, domain, and context -- the agent remembers what worked.
- **Problem:** there is no confidence signal for generated code quality.
  **Solved:** every pattern tracks success rate, usage frequency, and decay score -- the agent knows which patterns are battle-tested and which are speculative.
- **Problem:** code reuse requires exact matches, which rarely happen across projects.
  **Solved:** four matching strategies (signature, context, semantic, fuzzy) find applicable patterns even when variable names, types, and structures differ.
- **Problem:** combining multiple patterns into coherent code is manual and error-prone.
  **Solved:** pattern composition with gap filling and adapter generation assembles multiple patterns into working code with consistent types and interfaces.
- **Problem:** pattern libraries grow stale as languages and frameworks evolve.
  **Solved:** decay management and promotion engines automatically deprecate unused patterns and elevate frequently successful ones.

```bash
# Crystallize what works, reuse what is proven -- three commands
evolve crystallize --file src/handlers/user.rs --function create_user
evolve match signature "pub async fn create_user(input: CreateUserInput) -> Result<User>"
evolve compose --patterns p1,p2,p3 --output src/handlers/task.rs
```

---

<a name="how-it-works"></a>

## How It Works

<p align="center">
  <img src="assets/architecture-agentra.svg" alt="AgenticEvolve architecture" width="980">
</p>

### Architecture Overview

```
+-------------------------------------------------------------+
|                     YOUR AI AGENT                           |
|           (Claude, Cursor, Windsurf, Cody)                  |
+----------------------------+--------------------------------+
                             |
                  +----------v----------+
                  |      MCP LAYER      |
                  |   14 Tools + stdio  |
                  +----------+----------+
                             |
+----------------------------v--------------------------------+
|                   EVOLVE ENGINE                              |
+-----------+-----------+------------+-----------+------------+
| Matching  | Crystal-  | Composition| Collective| Token      |
| (4 types) | lization  | Engine     | Learning  | Conservation|
+-----------+-----------+------------+-----------+------------+
                             |
                  +----------v----------+
                  |    .evolve FILE     |
                  | (pattern library)   |
                  +---------------------+
```

<a name="capabilities"></a>

## 22 Capabilities

| Tier | Capabilities | Focus |
|:---|:---|:---|
| **T1: Storage** | Pattern Store, Pattern Index, Pattern Versioner, Pattern Validator | How are patterns stored? |
| **T2: Matching** | Signature Matcher, Context Matcher, Semantic Matcher, Fuzzy Matcher | How are patterns found? |
| **T3: Crystallization** | Pattern Extractor, Variable Detector, Template Generator, Confidence Calculator | How are patterns created? |
| **T4: Composition** | Pattern Composer, Gap Filler, Adapter Generator, Integration Weaver | How are patterns combined? |
| **T5: Collective** | Usage Tracker, Success Tracker, Decay Manager, Promotion Engine | How do patterns evolve? |
| **T6: Optimization** | Pattern Optimizer, Cache Manager | How is performance maintained? |

---

<a name="benchmarks"></a>

## Benchmarks

<p align="center">
  <img src="assets/benchmark-chart.svg" alt="AgenticEvolve benchmarks" width="980">
</p>

---

<a name="install"></a>

## Install

```bash
git clone https://github.com/agentralabs/agentic-evolve.git
cd agentic-evolve
cargo install --path crates/agentic-evolve-cli
```

```bash
curl -fsSL https://agentralabs.tech/install/evolve | bash
```

```bash
cargo install agentic-evolve-cli
cargo install agentic-evolve-mcp
pip install aevolve-installer && aevolve-install install --auto
npm install @agenticamem/evolve
```

**Standalone guarantee:** AgenticEvolve operates fully standalone. No other sister, external service, or orchestrator is required.

---

## License

MIT -- see [LICENSE](LICENSE).
