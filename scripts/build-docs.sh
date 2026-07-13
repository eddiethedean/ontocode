#!/usr/bin/env bash
# Build the Read the Docs / MkDocs site with CI-equivalent flags (no plugin warnings).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

export DISABLE_MKDOCS_2_WARNING=true

if ! command -v mkdocs >/dev/null 2>&1; then
  echo "error: mkdocs not found — run: pip install -r docs/requirements.txt" >&2
  exit 1
fi

mkdocs build --strict "$@"
