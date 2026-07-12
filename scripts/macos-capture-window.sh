#!/usr/bin/env bash
# Capture the VS Code / Electron Extension Development Host window to a PNG (macOS).
# Prefers screencapture -l <windowId> so we never accidentally grab the whole desktop.
# Usage: macos-capture-window.sh <output.png>
set -euo pipefail

OUT="${1:?usage: $0 <output.png>}"
mkdir -p "$(dirname "$OUT")"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BOUNDS_BIN="$ROOT/target/macos-window-bounds"

# Bring Extension Development Host forward when possible.
osascript <<'APPLESCRIPT' >/dev/null 2>&1 || true
tell application "System Events"
  set candidates to {"Electron", "Code - OSS", "Code", "Visual Studio Code"}
  repeat with procName in candidates
    if exists process procName then
      set frontmost of process procName to true
      delay 0.5
      exit repeat
    end if
  end repeat
end tell
APPLESCRIPT

WININFO=""
if [[ -x "$BOUNDS_BIN" ]]; then
  WININFO="$("$BOUNDS_BIN" 2>/dev/null || true)"
fi

capture_ok=0
if [[ -n "$WININFO" ]]; then
  IFS=',' read -r WID X Y W H <<<"$WININFO"
  echo "macos-capture-window: windowId=$WID bounds=${X},${Y},${W},${H}" >&2
  # -l captures that window even if partially occluded; -o avoids shadow
  if screencapture -x -o -l"$WID" "$OUT" 2>/dev/null; then
    capture_ok=1
  elif (( W >= 100 && H >= 100 )); then
    if screencapture -x -R"${X},${Y},${W},${H}" "$OUT" 2>/dev/null; then
      capture_ok=1
    fi
  fi
fi

if (( capture_ok == 0 )); then
  # Do NOT fall back to full-desktop capture — that leaks unrelated UI into docs.
  echo "macos-capture-window: could not locate VS Code/Electron window" >&2
  echo "  (grant Screen Recording + Accessibility to Cursor, keep Extension Development Host visible)" >&2
  exit 1
fi

if [[ ! -s "$OUT" ]]; then
  echo "macos-capture-window: empty output $OUT" >&2
  exit 1
fi

if command -v sips >/dev/null 2>&1; then
  sips -z 1024 1536 "$OUT" >/dev/null 2>&1 || sips --resampleWidth 1536 "$OUT" >/dev/null 2>&1 || true
fi

echo "wrote $OUT ($(wc -c <"$OUT" | tr -d ' ') bytes)"
