#!/usr/bin/env bash
# Run cargo-mutants on path_jail + OWL patch (manual / nightly).
# Requires: cargo install cargo-mutants
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if ! command -v cargo-mutants >/dev/null 2>&1 && ! cargo mutants --version >/dev/null 2>&1; then
  echo "cargo-mutants not found. Install with: cargo install cargo-mutants" >&2
  exit 1
fi

# File globs with a slash match the full workspace-relative path.
echo "==> mutants: ontocore-core path_jail.rs"
cargo mutants -p ontocore-core \
  --file 'crates/ontocore-core/src/path_jail.rs' \
  --test-tool=cargo \
  "$@"

# Include workspace package so tests/owl_patch_oracles.rs kills apply_one_patch no-ops.
echo "==> mutants: ontocore-owl patch.rs (tests: ontocore-owl + ontocode)"
cargo mutants -p ontocore-owl \
  --file 'crates/ontocore-owl/src/patch.rs' \
  --test-package ontocore-owl \
  --test-package ontocode \
  --test-tool=cargo \
  "$@"

echo "Mutants runs finished. Inspect mutants.out/ for survivors."
