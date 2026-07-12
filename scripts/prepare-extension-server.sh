#!/usr/bin/env bash
# Copy a built ontocore-lsp binary into extension/server/<platform>/ for extension tests or VSIX packaging.
#
# On macOS, recreate the destination directory before copying so Gatekeeper
# does not SIGKILL a previously quarantined binary (common with xattrs on
# external volumes / CI copies).
set -euo pipefail

PLATFORM="${1:?Usage: $0 <platform> e.g. linux-x64, darwin-arm64, win32-x64>}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEST="$ROOT/extension/server/$PLATFORM"
BIN="${ONTOCORE_LSP_BIN:-$ROOT/target/debug/ontocore-lsp}"

rm -rf "$DEST"
mkdir -p "$DEST"

if [[ "$PLATFORM" == win32-* ]]; then
  if [[ -f "${BIN}.exe" ]]; then
    cp "${BIN}.exe" "$DEST/ontocore-lsp.exe"
  else
    cp "$BIN" "$DEST/ontocore-lsp.exe"
  fi
else
  cp "$BIN" "$DEST/ontocore-lsp"
  chmod +x "$DEST/ontocore-lsp"
  # Clear quarantine / provenance that can cause SIGKILL on launch (macOS).
  if command -v xattr >/dev/null 2>&1; then
    xattr -cr "$DEST" 2>/dev/null || true
  fi
fi

echo "Bundled LSP at $DEST"
