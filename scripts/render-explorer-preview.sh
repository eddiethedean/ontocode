#!/usr/bin/env bash
# Regenerate docs/media/explorer-preview.png from the SVG source and sync to extension/media/.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SVG="$ROOT/docs/media/explorer-preview.svg"
PNG="$ROOT/docs/media/explorer-preview.png"
EXT_PNG="$ROOT/extension/media/explorer-preview.png"
WIDTH=960
HEIGHT=560

if [[ ! -f "$SVG" ]]; then
  echo "error: missing $SVG" >&2
  exit 1
fi

rendered=0
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

if command -v npx >/dev/null 2>&1; then
  if npx --yes @resvg/resvg-js-cli "$SVG" "$PNG" --fit-width "$WIDTH" >/dev/null 2>&1; then
    rendered=1
  fi
fi

if [[ "$rendered" -eq 0 ]] && command -v rsvg-convert >/dev/null 2>&1; then
  rsvg-convert -w "$WIDTH" -h "$HEIGHT" "$SVG" -o "$PNG"
  rendered=1
fi

if [[ "$rendered" -eq 0 ]] && command -v magick >/dev/null 2>&1; then
  magick -background none -resize "${WIDTH}x${HEIGHT}" "$SVG" "$PNG"
  rendered=1
fi

if [[ "$rendered" -eq 0 ]] && command -v qlmanage >/dev/null 2>&1; then
  qlmanage -t -s "$WIDTH" -o "$tmpdir" "$SVG" >/dev/null 2>&1
  thumb="$tmpdir/$(basename "$SVG").png"
  if [[ -f "$thumb" ]] && command -v sips >/dev/null 2>&1; then
    cp "$thumb" "$PNG"
    sips -z "$HEIGHT" "$WIDTH" "$PNG" >/dev/null
    rendered=1
  fi
fi

if [[ "$rendered" -eq 0 ]]; then
  echo "error: no SVG renderer found (npx @resvg/resvg-js-cli, rsvg-convert, magick, or qlmanage+sips)" >&2
  exit 1
fi

cp "$PNG" "$EXT_PNG"
echo "Wrote $PNG"
echo "Synced $EXT_PNG"
