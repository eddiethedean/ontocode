#!/usr/bin/env bash
# Package first-success tutorial files for GitHub Releases (ontocode-tutorial.zip).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

OUT="${1:-ontocode-tutorial.zip}"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

mkdir -p "$TMP/ontocode-tutorial"
for f in example.ttl complex-classes.ttl demo.obo; do
  cp "fixtures/$f" "$TMP/ontocode-tutorial/"
done

(
  cd "$TMP"
  zip -r "$ROOT/$OUT" ontocode-tutorial
)

echo "Created $ROOT/$OUT"
