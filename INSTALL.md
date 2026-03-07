# Installation Guide

## Quick Install (Default: Desktop Profile)

```bash
curl -fsSL https://agentralabs.tech/install/evolve | bash
```

## Install Profiles

### Desktop (default)

Installs binary and auto-configures detected MCP clients (Claude Desktop, Cursor, Windsurf, VS Code, etc.).

```bash
curl -fsSL https://agentralabs.tech/install/evolve/desktop | bash
```

### Terminal

Installs binary and configures MCP clients, optimized for terminal-first workflows.

```bash
curl -fsSL https://agentralabs.tech/install/evolve/terminal | bash
```

### Server

Installs binary only, no desktop config changes. Requires token-based auth.

```bash
curl -fsSL https://agentralabs.tech/install/evolve/server | bash
```

## Alternative Install Methods

### Cargo (Rust)

```bash
cargo install agentic-evolve-cli
cargo install agentic-evolve-mcp
```

### Python Installer

```bash
pip install aevolve-installer && aevolve-install install --auto
```

### npm

```bash
npm install @agenticamem/evolve
```

## Standalone Guarantee

AgenticEvolve works as a fully standalone tool. No other Agentra sisters are required. Integration with AgenticMemory, AgenticCodebase, and other sisters is optional and additive.
