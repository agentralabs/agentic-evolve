#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "ERROR: $*" >&2
  exit 1
}

echo "[1/5] Verify pattern store and retrieval"
cargo test --quiet -p agentic-evolve-core --test phase2_storage 2>/dev/null \
  || fail "Pattern store tests failed"

echo "[2/5] Verify matching engines"
cargo test --quiet -p agentic-evolve-core --test phase3_matching 2>/dev/null \
  || fail "Matching tests failed"

echo "[3/5] Verify crystallization"
cargo test --quiet -p agentic-evolve-core --test phase4_crystallization 2>/dev/null \
  || fail "Crystallization tests failed"

echo "[4/5] Verify composition"
cargo test --quiet -p agentic-evolve-core --test phase5_composition 2>/dev/null \
  || fail "Composition tests failed"

echo "[5/5] Verify collective learning"
cargo test --quiet -p agentic-evolve-core --test phase6_collective 2>/dev/null \
  || fail "Collective learning tests failed"

echo "Primary evolve problem checks passed (P01-P08)"
