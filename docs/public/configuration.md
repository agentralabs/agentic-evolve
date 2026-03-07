---
status: stable
---

# Configuration

## Environment Variables

| Variable | Default | Allowed Values | Effect |
|----------|---------|----------------|--------|
| `AEVOLVE_STORE_PATH` | `~/.agentra/evolve/patterns.aevolve` | Any file path | Pattern store location |
| `AEVOLVE_LOG_LEVEL` | `info` | `trace`, `debug`, `info`, `warn`, `error` | Log verbosity |
| `AGENTRA_INSTALL_PROFILE` | `desktop` | `desktop`, `terminal`, `server` | Install profile |
| `AGENTRA_WORKSPACE_ROOT` | (auto-detect) | Any directory path | Workspace root override |
| `AGENTRA_PROJECT_ROOT` | (auto-detect) | Any directory path | Project root override |
| `AGENTIC_TOKEN` | (none) | Any string | Auth token for server mode |
| `AGENTIC_TOKEN_FILE` | (none) | File path | Auth token file for server mode |
| `AGENTRA_RUNTIME_MODE` | (none) | `server` | Force server runtime mode |

## Config File

AgenticEvolve reads `~/.agentra/evolve/config.toml` if present:

```toml
[store]
path = "~/.agentra/evolve/patterns.aevolve"

[matching]
default_limit = 10
min_confidence = 0.5

[collective]
decay_rate = 0.01
promotion_threshold = 0.8

[server]
mode = "full"
```

## Runtime Modes

- **Desktop** -- Full MCP integration, auto-configured clients
- **Terminal** -- MCP integration, terminal-optimized
- **Server** -- Token-required, no desktop config changes
