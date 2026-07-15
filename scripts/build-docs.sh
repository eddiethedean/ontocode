#!/usr/bin/env bash
# Build the Read the Docs / MkDocs site with CI-equivalent flags (no plugin warnings).
#
# Fast by default: git-revision + HTML minify stay off unless READTHEDOCS is set
# (production) or you opt in:
#   ENABLE_GIT_REVISION_DATE=true ./scripts/build-docs.sh
#   ENABLE_MKDOCS_MINIFY=true ./scripts/build-docs.sh
# `--strict` does not add meaningful cost; keep it always for CI/RTD parity.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

export DISABLE_MKDOCS_2_WARNING=true

if ! command -v mkdocs >/dev/null 2>&1; then
  echo "error: mkdocs not found — run: pip install -r docs/requirements.txt" >&2
  exit 1
fi

# Drop leftover mkdocs processes so overlapping runs do not fight over site/.
if command -v pkill >/dev/null 2>&1; then
  pkill -f "mkdocs build" 2>/dev/null || true
  pkill -f "mkdocs serve" 2>/dev/null || true
fi

if [[ "${ENABLE_GIT_REVISION_DATE:-}" == "1" || "${ENABLE_GIT_REVISION_DATE:-}" == "true" ]]; then
  echo "note: ENABLE_GIT_REVISION_DATE set — revision stamps enabled (slow on large trees)" >&2
fi
if [[ "${ENABLE_MKDOCS_MINIFY:-}" == "true" ]]; then
  echo "note: ENABLE_MKDOCS_MINIFY set — HTML minify enabled" >&2
fi

mkdocs build --strict "$@"
