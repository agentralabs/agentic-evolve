---
status: stable
---

# Runtime and Install Sync

## Runtime Modes

AgenticEvolve supports three runtime profiles:

- **Desktop** -- Full MCP integration with auto-configured clients
- **Terminal** -- MCP integration with terminal-optimized workflows
- **Server** -- Headless mode with token-based authentication

## Install Sync

The installer ensures consistent state across all configured MCP clients:

1. Binary installed to `~/.local/bin/`
2. MCP config merged (not overwritten) into detected client configs
3. Pattern store initialized at default location
4. Post-install verification via `evolve info`

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `AEVOLVE_STORE_PATH` | `~/.agentra/evolve/patterns.aevolve` | Pattern store location |
| `AGENTRA_INSTALL_PROFILE` | `desktop` | Install profile |
| `AGENTIC_TOKEN` | (none) | Auth token for server mode |
| `AGENTRA_WORKSPACE_ROOT` | (auto-detect) | Workspace root override |
