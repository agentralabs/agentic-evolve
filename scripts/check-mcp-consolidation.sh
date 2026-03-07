#!/usr/bin/env bash
set -eu
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FAIL=0
echo "=== MCP Consolidation Check ==="
# Check no unwraps in MCP production code
for f in $(find "$ROOT/crates/"*-mcp/src -name "*.rs" 2>/dev/null); do
  COUNT=$(awk '/#\[cfg\(test\)\]/{exit} /\.unwrap\(\)/' "$f" | wc -l | tr -d ' ')
  [ "$COUNT" -gt 0 ] && { echo "FAIL: $COUNT unwraps in $(basename $f)"; FAIL=1; }
done
[ $FAIL -eq 0 ] && echo "PASS: No unwraps in MCP production code"
[ $FAIL -ne 0 ] && { echo "=== FAILED ==="; exit 1; }
echo "=== ALL PASSED ==="
