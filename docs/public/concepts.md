---
status: stable
---

# Core Concepts

## Patterns

A pattern is a reusable code template with metadata. Each pattern has:

- **Name** -- Human-readable identifier
- **Domain** -- Category (web, cli, data, etc.)
- **Language** -- Target language (rust, python, etc.)
- **Template body** -- Code with variable placeholders
- **Confidence** -- How reliable the pattern is (0.0 to 1.0)
- **Tags** -- Searchable labels

## Crystallization

Crystallization is the process of extracting a reusable template from verified source code:

1. **Extraction** -- Parse source code to identify reusable structure
2. **Variable Detection** -- Find values that should become template variables
3. **Template Generation** -- Create the parameterized template
4. **Confidence Calculation** -- Score the template's reliability

## Matching

Four matching engines find the right pattern:

- **Signature Matching** -- Compare function signatures
- **Context Matching** -- Compare surrounding code context
- **Semantic Matching** -- Compare meaning and intent
- **Fuzzy Matching** -- Approximate text matching

## Composition

Composition weaves multiple patterns into a single output:

- **Gap Filling** -- Insert glue code between patterns
- **Adapter Generation** -- Create type adapters between pattern interfaces
- **Integration Weaving** -- Merge pattern bodies with conflict resolution

## Collective Learning

The pattern library improves over time through:

- **Usage Tracking** -- Record which patterns get used
- **Success Tracking** -- Record which uses succeed
- **Decay** -- Reduce confidence of unused patterns
- **Promotion** -- Boost confidence of successful patterns
