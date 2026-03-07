---
status: stable
---

# MCP Tools (14)

## evolve_pattern_store

Store a new pattern in the library.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| name | string | yes | Pattern name |
| domain | string | yes | Domain category |
| language | string | yes | Target language |
| body | string | yes | Template body |
| tags | string[] | no | Searchable tags |
| confidence | number | no | Initial confidence (0.0-1.0) |

## evolve_pattern_get

Retrieve a pattern by name or ID.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| name | string | no | Pattern name |
| id | string | no | Pattern ID |

## evolve_pattern_search

Search patterns by domain, language, or tags.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| domain | string | no | Filter by domain |
| language | string | no | Filter by language |
| tags | string[] | no | Filter by tags |
| limit | number | no | Max results (default: 10) |

## evolve_pattern_list

List all patterns with optional filters.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| domain | string | no | Filter by domain |
| limit | number | no | Max results |

## evolve_pattern_delete

Remove a pattern from the library.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| name | string | no | Pattern name |
| id | string | no | Pattern ID |

## evolve_match_signature

Match patterns by function signature.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| signature | string | yes | Function signature to match |
| limit | number | no | Max results |
| min_score | number | no | Minimum match score |

## evolve_match_context

Match patterns by surrounding code context.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| context | string | yes | Code context |
| limit | number | no | Max results |

## evolve_crystallize

Crystallize source code into a reusable pattern.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| source | string | yes | Source code to crystallize |
| domain | string | no | Domain category |
| language | string | no | Language |
| confidence | number | no | Minimum confidence threshold |

## evolve_get_body

Retrieve the template body of a pattern.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| name | string | no | Pattern name |
| id | string | no | Pattern ID |

## evolve_compose

Compose multiple patterns into one.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| patterns | string[] | yes | Pattern names or IDs |
| strategy | string | no | Composition strategy |

## evolve_coverage

Check pattern coverage for a file or project.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| path | string | yes | File or directory path |
| depth | number | no | Scan depth |

## evolve_confidence

Calculate confidence score for a pattern match.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| pattern_id | string | yes | Pattern ID |
| context | string | no | Optional context for contextual scoring |

## evolve_update_usage

Record a pattern usage event.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| pattern_id | string | yes | Pattern ID |
| success | boolean | no | Whether usage was successful |

## evolve_optimize

Run optimization pass on the pattern library.

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| mode | string | no | Optimization mode: full, duplicates, prune |
