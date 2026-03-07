# Contributing to AgenticEvolve

Thank you for your interest in contributing to AgenticEvolve! This document provides guidelines for contributing to the project.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR_USERNAME/agentic-evolve.git`
3. Create a feature branch: `git checkout -b my-feature`
4. Make your changes
5. Run the tests (see below)
6. Commit and push
7. Open a pull request

## Development Setup

This is a Cargo workspace monorepo. All Rust crates are under `crates/`.

### Rust Workspace

```bash
# Build everything (core + MCP server + CLI + FFI)
cargo build --workspace

# Run all tests
cargo test --workspace

# Core library only
cargo test -p agentic-evolve-core

# MCP server only
cargo test -p agentic-evolve-mcp

# Run the CLI
cargo run -p agentic-evolve-cli -- pattern list

# Run the MCP server
cargo run -p agentic-evolve-mcp -- serve
```

## Ways to Contribute

### Report Bugs

File an issue with:
- Steps to reproduce
- Expected behavior
- Actual behavior
- System info (OS, Rust version)

### Add an MCP Tool

1. Create a new tool handler in `crates/agentic-evolve-mcp/src/tools/`
2. Register it in `crates/agentic-evolve-mcp/src/tools/registry.rs`
3. Add tests in `crates/agentic-evolve-mcp/tests/`

### Write Examples

1. Add a new example in `examples/`
2. Ensure it runs without errors
3. Add a docstring explaining what it demonstrates
4. Update `examples/README.md`

### Improve Documentation

All docs are in `docs/`. Fix typos, add examples, clarify explanations -- all welcome.

## Code Guidelines

- **Rust**: Follow standard Rust conventions. Run `cargo clippy` and `cargo fmt`.
- **Tests**: Every feature needs tests. We maintain 250+ tests across the stack.
- **Documentation**: Update docs when changing public APIs.

## Commit Messages

Use clear, descriptive commit messages:
- `feat: new pattern matching algorithm`
- `fix: memory leak in pattern store`
- `docs: add crystallization guide`

## Pull Request Guidelines

- Keep PRs focused -- one feature or fix per PR
- Include tests for new functionality
- Update documentation if needed
- Ensure all tests pass before submitting
- Write a clear PR description

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
