---
status: stable
---

# MCP Prompts

## evolve_find_pattern

Find the best matching pattern for a coding task.

**Arguments:**

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| task | string | yes | Description of the coding task |
| language | string | no | Target language |

**Behavior:** Searches across all matching engines (signature, context, semantic, fuzzy) and returns the highest-confidence match with template body and variable bindings.

**Example:**

```
User: Find a pattern for JWT authentication middleware in Rust
Agent: [calls evolve_match_signature, evolve_match_context]
       Found "jwt-auth-middleware" (confidence: 0.91)
       Template has 3 variables: secret_key, token_header, expiry_duration
```

## evolve_crystallize_code

Crystallize a block of code into a reusable pattern.

**Arguments:**

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| code | string | yes | Source code to crystallize |
| name | string | no | Suggested pattern name |

**Behavior:** Runs extraction, variable detection, template generation, and confidence calculation. Returns the crystallized pattern for review before storing.

## evolve_compose_solution

Compose multiple patterns into a complete solution.

**Arguments:**

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| requirements | string | yes | Description of what to build |
| patterns | string[] | no | Specific patterns to use |

**Behavior:** Identifies relevant patterns, runs composition with gap filling, and returns the combined output with integration notes.
