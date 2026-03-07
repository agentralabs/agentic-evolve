#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "ERROR: $*" >&2
  exit 1
}

find_fixed() {
  local pattern="$1"
  shift
  if command -v rg >/dev/null 2>&1; then
    rg -nF "$pattern" "$@"
  else
    grep -R -n -F -- "$pattern" "$@"
  fi
}

find_regex() {
  local pattern="$1"
  shift
  if command -v rg >/dev/null 2>&1; then
    rg -n "$pattern" "$@"
  else
    grep -R -n -E -- "$pattern" "$@"
  fi
}

assert_contains() {
  local pattern="$1"
  shift
  if ! find_fixed "$pattern" "$@" >/dev/null; then
    fail "Missing required install command: ${pattern}"
  fi
}

http_ok() {
  local url="$1"
  curl -fsSL --retry 3 --retry-delay 1 --retry-connrefused \
    -A "agentra-install-guardrails/1.0 (+https://agentralabs.tech)" \
    "$url" >/dev/null
}

# Front-facing command requirements
assert_contains "curl -fsSL https://agentralabs.tech/install/evolve | bash" README.md docs/quickstart.md
assert_contains "curl -fsSL https://agentralabs.tech/install/evolve/desktop | bash" README.md docs/quickstart.md INSTALL.md
assert_contains "curl -fsSL https://agentralabs.tech/install/evolve/terminal | bash" README.md docs/quickstart.md INSTALL.md
assert_contains "curl -fsSL https://agentralabs.tech/install/evolve/server | bash" README.md docs/quickstart.md INSTALL.md
assert_contains "cargo install agentic-evolve-cli" README.md
assert_contains "cargo install agentic-evolve-mcp" README.md
assert_contains "pip install aevolve-installer && aevolve-install install --auto" README.md

# Invalid patterns
if find_regex "curl -fsSL https://agentralabs.tech/install/evolve \| sh" README.md docs >/dev/null; then
  fail "Found invalid shell invocation for evolve installer"
fi

# Installer health
bash -n scripts/install.sh
bash scripts/install.sh --dry-run >/dev/null
bash scripts/install.sh --profile=desktop --dry-run >/dev/null

terminal_out="$(bash scripts/install.sh --profile=terminal --dry-run 2>&1)"
echo "$terminal_out" | grep -F "Configuring MCP clients..." >/dev/null \
  || fail "Terminal profile must auto-configure MCP clients"
echo "$terminal_out" | grep -F "Detected MCP client configs merged" >/dev/null \
  || fail "Terminal profile must report universal MCP merge"
echo "$terminal_out" | grep -F "What happens after installation:" >/dev/null \
  || fail "Missing post-install guidance block"

server_out="$(bash scripts/install.sh --profile=server --dry-run 2>&1)"
echo "$server_out" | grep -F "Server deployments should enforce auth" >/dev/null \
  || fail "Server profile must include auth guidance"
echo "$server_out" | grep -F 'TOKEN=$(openssl rand -hex 32)' >/dev/null \
  || fail "Server profile must include token generation guidance"

echo "Install command guardrails passed (evolve)."
