---
status: stable
---

# Troubleshooting

## Installation Issues

### Binary not found after install

Add `~/.local/bin` to your PATH:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Add this to your shell profile (`~/.zshrc`, `~/.bashrc`) for persistence.

### Permission denied during install

Ensure the install directory is writable:

```bash
mkdir -p ~/.local/bin
chmod u+w ~/.local/bin
```

### Cargo install fails

Ensure Rust toolchain is up to date:

```bash
rustup update stable
```

## MCP Server Issues

### Server not appearing in MCP client

1. Restart your MCP client (Claude Desktop, Cursor, etc.)
2. Check that the config was merged correctly:
   - Claude Desktop: `~/Library/Application Support/Claude/claude_desktop_config.json`
   - Cursor: `~/.cursor/mcp.json`
3. Verify the binary exists: `ls ~/.local/bin/agentic-evolve-mcp`

### Server crashes on startup

Check for port conflicts or corrupted store:

```bash
evolve info
evolve serve --verbose
```

### Tool not found error (-32803)

The MCP client is calling a tool that does not exist. Check the tool name against the 14 registered tools in [mcp-tools.md](mcp-tools.md).

## Common Errors

### "Store file corrupted"

The `.aevolve` file failed BLAKE3 checksum validation. Restore from backup or recreate:

```bash
evolve pattern export --output backup.json
rm patterns.aevolve
evolve pattern import --input backup.json
```

### "Pattern not found"

The pattern name or ID does not exist in the store. List available patterns:

```bash
evolve pattern list
```

## Performance Tips

- Keep pattern count under 10,000 for optimal matching speed
- Run `evolve optimize full` periodically to clean up the store
- Use domain filters in searches to reduce scan scope
