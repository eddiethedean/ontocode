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
cargo mutants -p ontocore-core --file 'crates/ontocore-core/src/path_jail.rs' --test-tool=cargo "$@"

echo "==> mutants: ontocore-owl patch.rs"
cargo mutants -p ontocore-owl --file 'crates/ontocore-owl/src/patch.rs' --test-tool=cargo "$@"

echo "Mutants runs finished. Inspect mutants.out/ for survivors."
