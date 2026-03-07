---
status: stable
---

# CLI Reference

## Global Options

```
evolve [OPTIONS] <COMMAND>

Options:
  --store <PATH>    Path to .aevolve store (default: auto-detect)
  --verbose         Enable verbose output
  --json            Output as JSON
  -h, --help        Print help
  -V, --version     Print version
```

## Commands

### pattern store

```
evolve pattern store --name <NAME> --domain <DOMAIN> --lang <LANG> [--body <BODY>] [--file <FILE>] [--tags <TAGS>] [--confidence <FLOAT>]
```

### pattern get

```
evolve pattern get --name <NAME> | --id <ID>
```

### pattern search

```
evolve pattern search [--domain <DOMAIN>] [--lang <LANG>] [--tags <TAGS>] [--limit <N>]
```

### match signature

```
evolve match signature --input <SIGNATURE> [--limit <N>] [--min-score <FLOAT>]
```

### match context

```
evolve match context --input <CONTEXT> [--limit <N>]
```

### crystallize

```
evolve crystallize --file <PATH> [--confidence <FLOAT>] [--domain <DOMAIN>] [--lang <LANG>]
```

### compose run

```
evolve compose run --patterns <P1,P2,...> [--output <PATH>]
```

### serve

```
evolve serve [--mode <MODE>] [--port <PORT>]
```

See `evolve --help` and `evolve <command> --help` for full details.
