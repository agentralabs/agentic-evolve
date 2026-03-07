#!/usr/bin/env bash
# Run criterion benchmarks for AgenticEvolve core.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$ROOT_DIR"

echo "=== AgenticEvolve Benchmark Suite ==="
echo ""

if [ "${1:-}" = "--help" ] || [ "${1:-}" = "-h" ]; then
    echo "Usage: $0 [filter]"
    echo ""
    echo "  filter  Optional regex to select specific benchmarks"
    echo ""
    echo "Examples:"
    echo "  $0                    # Run all benchmarks"
    echo "  $0 cache              # Run only cache benchmarks"
    echo "  $0 signature          # Run only signature matching benchmarks"
    echo "  $0 pattern_creation   # Run a single benchmark"
    exit 0
fi

FILTER="${1:-}"

if [ -n "$FILTER" ]; then
    echo "Running benchmarks matching: $FILTER"
    echo ""
    cargo bench --bench core_bench -p agentic-evolve-core -- "$FILTER"
else
    echo "Running all benchmarks..."
    echo ""
    cargo bench --bench core_bench -p agentic-evolve-core
fi

echo ""
echo "HTML reports available in: target/criterion/"
