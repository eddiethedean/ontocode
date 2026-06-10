#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PLATFORM="$(uname -s | tr '[:upper:]' '[:lower:]')"
case "$(uname -m)" in
  x86_64) ARCH="x64" ;;
  aarch64 | arm64) ARCH="arm64" ;;
  *) ARCH="$(uname -m)" ;;
esac
SERVER_DIR="$ROOT/extension/server/${PLATFORM}-${ARCH}"

cargo build --release -p ontoindex-lsp

mkdir -p "$SERVER_DIR"
cp "$ROOT/target/release/ontoindex-lsp" "$SERVER_DIR/ontoindex-lsp"
chmod +x "$SERVER_DIR/ontoindex-lsp"

cd "$ROOT/extension"
npm ci
npm run compile

echo "Extension built. Server binary at $SERVER_DIR/ontoindex-lsp"
