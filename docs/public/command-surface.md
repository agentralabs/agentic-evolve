---
status: stable
---

# Command Surface (38 Commands)

## Pattern Commands

| Command | Description |
|---------|-------------|
| `evolve pattern store` | Store a new pattern |
| `evolve pattern get` | Retrieve a pattern by name or ID |
| `evolve pattern search` | Search patterns by domain, language, or tags |
| `evolve pattern list` | List all patterns |
| `evolve pattern delete` | Remove a pattern |
| `evolve pattern export` | Export patterns to JSON |
| `evolve pattern import` | Import patterns from JSON |
| `evolve pattern count` | Count patterns |
| `evolve pattern tags` | List all tags |
| `evolve pattern domains` | List all domains |

## Match Commands

| Command | Description |
|---------|-------------|
| `evolve match signature` | Match by function signature |
| `evolve match context` | Match by surrounding code context |
| `evolve match semantic` | Match by semantic similarity |
| `evolve match fuzzy` | Fuzzy text match |

## Crystallization Commands

| Command | Description |
|---------|-------------|
| `evolve crystallize` | Crystallize source into a pattern |

## Body Commands

| Command | Description |
|---------|-------------|
| `evolve body get` | Get template body |
| `evolve body preview` | Preview instantiated body |

## Composition Commands

| Command | Description |
|---------|-------------|
| `evolve compose run` | Compose patterns |
| `evolve compose preview` | Preview composition |

## Coverage Commands

| Command | Description |
|---------|-------------|
| `evolve coverage file` | Check coverage for a file |
| `evolve coverage summary` | Coverage summary |

## Stats Commands

| Command | Description |
|---------|-------------|
| `evolve stats overview` | Overview statistics |
| `evolve stats patterns` | Pattern statistics |
| `evolve stats usage` | Usage statistics |
| `evolve stats success` | Success rate statistics |
| `evolve stats decay` | Decay statistics |

## Usage Commands

| Command | Description |
|---------|-------------|
| `evolve usage update` | Record usage event |
| `evolve usage top` | Top used patterns |
| `evolve usage bottom` | Least used patterns |

## Lifecycle Commands

| Command | Description |
|---------|-------------|
| `evolve promote` | Promote high-performing patterns |
| `evolve decay` | Apply decay to stale patterns |

## Optimization Commands

| Command | Description |
|---------|-------------|
| `evolve optimize full` | Full optimization pass |
| `evolve optimize duplicates` | Find and merge duplicates |
| `evolve optimize prune` | Remove low-value patterns |

## Server Commands

| Command | Description |
|---------|-------------|
| `evolve serve` | Start MCP server |

## Info Commands

| Command | Description |
|---------|-------------|
| `evolve info` | System information |
| `evolve version` | Version information |
