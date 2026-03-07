---
status: stable
---

# MCP Resources

## URI Scheme

AgenticEvolve exposes resources via the `evolve://` URI scheme.

## Available Resources

### evolve://store/stats

Returns pattern store statistics including pattern count, total size, and index health.

**Format:** JSON

```json
{
  "pattern_count": 142,
  "total_size_bytes": 28400,
  "index_entries": 142,
  "domains": ["web", "cli", "data"],
  "languages": ["rust", "python"]
}
```

### evolve://store/patterns

Returns a summary list of all patterns.

**Format:** JSON array

### evolve://pattern/{id}

Returns full pattern details including template body.

**Format:** JSON

## Cross-Sister References

- `amem://` -- AgenticMemory resources
- `acb://` -- AgenticCodebase resources
- `avis://` -- AgenticVision resources

AgenticEvolve can reference memory nodes and codebase files when bridge traits are connected.
