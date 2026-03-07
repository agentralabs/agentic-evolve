# Examples

## Pattern Store

```bash
evolve pattern store --name "auth-middleware" --domain web --lang rust --body 'fn auth() {}'
```

## Crystallization

```bash
evolve crystallize --file src/handler.rs --confidence 0.85
```

## Composition

```bash
evolve compose run --patterns auth-middleware,rate-limiter --output combined.rs
```

## MCP Integration

Start the MCP server:

```bash
evolve serve
```

Then use any MCP client to call the 14 available tools.
