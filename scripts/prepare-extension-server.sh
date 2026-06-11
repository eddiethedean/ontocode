#!/usr/bin/env bash
# Copy a built ontoindex-lsp binary into extension/server/<platform>/ for extension tests or VSIX packaging.
set -euo pipefail

PLATFORM="${1:?Usage: $0 <platform> e.g. linux-x64, darwin-arm64, win32-x64>}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/extension/server/$PLATFORM"
BIN="${ONTOINDEX_LSP_BIN:-$ROOT/target/debug/ontoindex-lsp}"

mkdir -p "$DEST"

if [[ "$PLATFORM" == win32-* ]]; then
  if [[ -f "${BIN}.exe" ]]; then
    cp "${BIN}.exe" "$DEST/ontoindex-lsp.exe"
  else
    cp "$BIN" "$DEST/ontoindex-lsp.exe"
  fi
else
  cp "$BIN" "$DEST/ontoindex-lsp"
  chmod +x "$DEST/ontoindex-lsp"
fi

echo "Bundled LSP at $DEST"
