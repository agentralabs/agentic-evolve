---
status: stable
---

# Playbook: Agent Integration

## Connecting an AI Agent to AgenticEvolve

### Step 1: Install

```bash
curl -fsSL https://agentralabs.tech/install/evolve | bash
```

### Step 2: Configure MCP

The installer auto-configures your MCP client. Verify by checking that `agentic-evolve` appears in your MCP server list.

### Step 3: Store Initial Patterns

Feed your agent's best code patterns into the library:

```bash
evolve crystallize --file src/auth.rs --confidence 0.9
evolve crystallize --file src/handlers.rs --confidence 0.85
```

### Step 4: Use Patterns in Agent Workflow

When your agent needs to generate code:

1. Query `evolve_match_signature` with the target function signature
2. If match found (confidence > 0.7), use pattern template
3. Fill template variables from current context
4. Record usage with `evolve_update_usage`

### Step 5: Monitor and Improve

```bash
evolve stats overview      # Check library health
evolve coverage summary    # Find uncovered areas
evolve optimize full       # Clean up the library
```

## Multi-Agent Setup

Multiple agents can share a pattern store by pointing to the same `.aevolve` file. The MCP server handles concurrent access safely.
