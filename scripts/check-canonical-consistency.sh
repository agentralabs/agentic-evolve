#!/usr/bin/env bash
set -euo pipefail
FAIL=0
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
echo "=== Canonical Consistency Check ==="
[ -f "$ROOT/sister.manifest.json" ] && echo "PASS: sister.manifest.json" || { echo "FAIL: sister.manifest.json"; FAIL=1; }
[ -f "$ROOT/CLAUDE.md" ] && echo "PASS: CLAUDE.md" || { echo "FAIL: CLAUDE.md"; FAIL=1; }
[ -f "$ROOT/docs/ecosystem/CANONICAL_SISTER_KIT.md" ] && echo "PASS: CANONICAL_SISTER_KIT.md" || { echo "FAIL: CANONICAL_SISTER_KIT.md"; FAIL=1; }
for doc in architecture.md cli-reference.md configuration.md ffi-reference.md mcp-tools.md mcp-resources.md mcp-prompts.md troubleshooting.md quickstart.md concepts.md integration-guide.md faq.md benchmarks.md api-reference.md; do
  [ -f "$ROOT/docs/public/$doc" ] && echo "PASS: $doc" || { echo "FAIL: $doc"; FAIL=1; }
done
SVG_COUNT=$(ls "$ROOT/assets/"*.svg 2>/dev/null | wc -l | tr -d ' ')
[ "$SVG_COUNT" -ge 4 ] && echo "PASS: $SVG_COUNT SVGs" || { echo "FAIL: $SVG_COUNT SVGs (need 4)"; FAIL=1; }
[ $FAIL -ne 0 ] && { echo "=== FAILED ==="; exit 1; }
echo "=== ALL PASSED ==="
