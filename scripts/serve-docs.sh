#!/usr/bin/env bash
# Serve the MkDocs site locally (suppresses MkDocs 2 plugin banner).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

export DISABLE_MKDOCS_2_WARNING=true

if ! command -v mkdocs >/dev/null 2>&1; then
  echo "error: mkdocs not found — run: pip install -r docs/requirements.txt" >&2
  exit 1
fi

mkdocs serve "$@"
