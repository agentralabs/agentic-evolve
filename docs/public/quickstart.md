---
status: stable
---

# Quickstart

## Install

```bash
curl -fsSL https://agentralabs.tech/install/evolve | bash
```

## Store Your First Pattern

```bash
evolve pattern store --name "hello-world" --domain general --lang rust \
  --body 'fn main() { println!("Hello!"); }'
```

## Search Patterns

```bash
evolve pattern search --domain general
```

## Match by Signature

```bash
evolve match signature --input "fn authenticate(req: Request) -> Result<User>"
```

## Crystallize from Source

```bash
evolve crystallize --file handler.rs --confidence 0.85
```

## Compose Patterns

```bash
evolve compose run --patterns auth-middleware,rate-limiter --output combined.rs
```
