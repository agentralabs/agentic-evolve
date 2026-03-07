---
status: stable
---

# API Reference

## Core Types

### Pattern

```rust
pub struct Pattern {
    pub id: PatternId,
    pub name: String,
    pub domain: String,
    pub language: String,
    pub tags: Vec<String>,
    pub template: Template,
    pub confidence: f64,
    pub usage: UsageStats,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Template

```rust
pub struct Template {
    pub body: String,
    pub variables: Vec<Variable>,
    pub version: u32,
}
```

### MatchResult

```rust
pub struct MatchResult {
    pub pattern_id: PatternId,
    pub score: f64,
    pub match_type: MatchType,
    pub bindings: HashMap<String, String>,
}
```

## Store Operations

- `PatternStore::new(path)` -- Create or open a pattern store
- `store.insert(pattern)` -- Add a pattern
- `store.get(id)` -- Retrieve by ID
- `store.search(query)` -- Search patterns
- `store.delete(id)` -- Remove a pattern
- `store.list()` -- List all patterns

## Matching

- `SignatureMatcher::match_against(signature, store)` -- Match by signature
- `ContextMatcher::match_against(context, store)` -- Match by context
- `SemanticMatcher::match_against(text, store)` -- Semantic match
- `FuzzyMatcher::match_against(text, store)` -- Fuzzy match

## Crystallization

- `Extractor::extract(source)` -- Extract pattern from source
- `VariableDetector::detect(source)` -- Find template variables
- `TemplateGenerator::generate(source, variables)` -- Generate template
- `ConfidenceCalculator::calculate(template)` -- Score confidence

## Composition

- `Composer::compose(patterns)` -- Compose multiple patterns
- `GapFiller::fill(composed)` -- Fill gaps between patterns
- `AdapterGenerator::adapt(a, b)` -- Generate type adapter
