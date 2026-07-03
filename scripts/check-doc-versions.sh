#!/usr/bin/env bash
# Verify user-facing documentation references the workspace package version.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

VERSION="$(grep -E '^version = ' Cargo.toml | head -1 | sed -E 's/.*"([^"]+)".*/\1/')"
if [[ -z "$VERSION" ]]; then
  echo "error: could not read workspace version from Cargo.toml" >&2
  exit 1
fi

echo "Checking documentation for version ${VERSION}..."

fail=0

check_file_contains() {
  local file="$1"
  local pattern="$2"
  local label="$3"
  if ! grep -qE "$pattern" "$file"; then
    echo "FAIL: $label — expected pattern '$pattern' in $file" >&2
    fail=1
  else
    echo "ok: $label"
  fi
}

check_file_contains "README.md" "v${VERSION}" "README header version"
check_file_contains "docs/index.md" "v${VERSION}" "docs index hero version"
check_file_contains "extension/README.md" "v${VERSION}" "extension README version"
check_file_contains "extension/package.json" "\"version\": \"${VERSION}\"" "extension package.json version"
check_file_contains "docs/guides/enterprise-eval.md" "v${VERSION}" "enterprise eval version"
MINOR_VERSION="${VERSION%.*}"
check_file_contains "SECURITY.md" "${MINOR_VERSION}\.x" "SECURITY.md supported versions"
check_file_contains "docs/release-integrity.md" "VERSION=${VERSION}" "release-integrity example version"
check_file_contains "mkdocs.yml" "site_url: https://ontocode-vs.readthedocs.io/" "mkdocs site_url matches RTD"
check_file_contains "README.md" "readthedocs.org/projects/ontocode-vs/badge" "RTD docs badge slug"

# Reference page titles must match current release
for file in docs/authoring.md docs/sql-reference.md docs/sparql-reference.md docs/patch-reference.md docs/cli-reference.md; do
  if grep -qE "^# .+ \(OntoIndex v0\.5\)" "$file"; then
    echo "FAIL: stale v0.5 title in $file" >&2
    fail=1
  elif ! grep -qE "^# .+ \(Onto(Index|Core) v${VERSION%.*}\)" "$file"; then
    echo "FAIL: reference title in $file should mention OntoIndex or OntoCore v${VERSION%.*}" >&2
    fail=1
  else
    echo "ok: reference title $file"
  fi
done

# Install pins must not reference an older release (allow changelog/migration history)
STALE_PIN_PATHS=(README.md docs extension crates .github)
if rg -q 'VERSION=0\.6\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.6.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.6\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.6.0 install pins"
fi

if rg -q 'VERSION=0\.7\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.7.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.7\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.7.0 install pins"
fi

# User-facing docs must not claim 0.7.x is the current release
USER_FACING_DOCS=(
  docs/faq.md
  docs/getting-started.md
  docs/guides/production-readiness.md
  docs/guides/rust-library.md
  docs/concepts.md
  docs/guides/vscode-extension.md
  docs/security.md
)
for file in "${USER_FACING_DOCS[@]}"; do
  if grep -qE '0\.7\.x \(current\)|ships in v0\.7\.0|OntoIndex v0\.7\.0|OntoCode v0\.7\.0|at \*\*0\.7\.x\*\*' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.7.x current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.7.x current-release claims in user-facing docs"
fi

# docs/security.md supported versions must match SECURITY.md for current minor
if ! grep -q "${MINOR_VERSION}\.x   | Yes" docs/security.md; then
  echo "FAIL: docs/security.md should list ${MINOR_VERSION}.x as supported (Yes)" >&2
  fail=1
else
  echo "ok: docs/security.md supported version"
fi

# v0.8 docs added in adoption review
for file in docs/guides/refactoring.md docs/migration/v0.8.md docs/migration/v0.9.md docs/examples/refactoring.md; do
  if [[ ! -f "$file" ]]; then
    echo "FAIL: missing required doc $file" >&2
    fail=1
  else
    echo "ok: $file exists"
  fi
done

check_file_contains "docs/faq.md" "0\.9\.x" "faq crate version"
check_file_contains "docs/guides/production-readiness.md" "v${VERSION}" "production-readiness version"

# Stale protege-coexistence version banner
if grep -qE 'evaluating OntoCode \*\*v0\.6\*\*|v0\.6 support' docs/guides/protege-coexistence.md; then
  echo "FAIL: stale v0.6 content in docs/guides/protege-coexistence.md" >&2
  fail=1
else
  echo "ok: protege-coexistence version"
fi

# Enterprise adoption pack pages
for file in \
  docs/guides/protege-decision.md \
  docs/guides/production-evidence.md \
  docs/guides/governance.md \
  docs/guides/platform-compatibility.md \
  docs/guides/release-timeline.md; do
  if [[ ! -f "$file" ]]; then
    echo "FAIL: missing enterprise doc $file" >&2
    fail=1
  else
    echo "ok: $file exists"
  fi
done

check_file_contains "docs/guides/protege-coexistence.md" "v0\.9" "protege-coexistence v0.9"
check_file_contains "docs/guides/release-timeline.md" "non-commitment" "release-timeline disclaimer"
if grep -qE 'OBO format \+ ROBOT interop.*Not shipped' docs/guides/enterprise-eval.md; then
  echo "FAIL: enterprise-eval.md contradicts SHIPPED.md on OBO/ROBOT" >&2
  fail=1
else
  echo "ok: enterprise-eval OBO/ROBOT status"
fi

# release-integrity must not pin an old example version
if grep -qE '^VERSION=0\.5\.0' docs/release-integrity.md; then
  echo "FAIL: stale VERSION=0.5.0 in docs/release-integrity.md" >&2
  fail=1
fi

# releasing.md tag example must match workspace version
if ! grep -qE "git tag v${VERSION}" docs/releasing.md; then
  echo "FAIL: docs/releasing.md tag example should use v${VERSION}" >&2
  fail=1
else
  echo "ok: releasing.md tag example"
fi

# Stale v0.5 current-release banners (allow historical mentions in changelog/roadmap)
for file in README.md docs/index.md extension/README.md docs/guides/enterprise-eval.md; do
  if grep -qE 'ships in v0\.5\.0|What ships in v0\.5\.0|included in v0\.5\.0|documentation · v0\.5' "$file"; then
    echo "FAIL: stale v0.5.0 current-release banner in $file" >&2
    fail=1
  fi
done

# RTD URL hygiene (search source paths only — exclude built site/)
RTD_SEARCH_PATHS=(
  README.md CONTRIBUTING.md extension crates docs scripts .github
)

if rg -q 'onto-code\.readthedocs|readthedocs\.org/projects/onto-code' "${RTD_SEARCH_PATHS[@]}"; then
  echo "FAIL: stale onto-code RTD slug found" >&2
  fail=1
else
  echo "ok: no dead onto-code RTD slug"
fi

if rg -q 'https://ontocode-vs\.readthedocs\.io/en/latest/[^)"[:space:]]+\.md' "${RTD_SEARCH_PATHS[@]}"; then
  echo "FAIL: absolute RTD URLs must not use .md extension (use trailing slash paths)" >&2
  rg -n 'https://ontocode-vs\.readthedocs\.io/en/latest/[^)"[:space:]]+\.md' "${RTD_SEARCH_PATHS[@]}" >&2 || true
  fail=1
else
  echo "ok: RTD URLs without .md extension"
fi

if rg -q 'https://ontocode-vs\.readthedocs\.io/"' README.md CONTRIBUTING.md extension crates docs; then
  echo "FAIL: RTD page URLs must include /en/latest/ (not bare project root)" >&2
  rg -n 'https://ontocode-vs\.readthedocs\.io/"' README.md CONTRIBUTING.md extension crates docs >&2 || true
  fail=1
else
  echo "ok: RTD page URLs use /en/latest/"
fi

check_file_contains "extension/package.json" "ontocode/vscode-extension/" "extension homepage VS Code docs path"
check_file_contains "extension/README.md" "ontocode/vscode-extension/" "extension README VS Code docs path"
check_file_contains "docs/guides/vscode-extension.md" "ontocode/vscode-extension" "vscode hub redirect"
check_file_contains "docs/guides/rust-crates.md" "ontocode/vscode-extension" "rust hub cross-link"
check_file_contains "crates/ontoindex-cli/src/main.rs" "OntoCode v${VERSION%.*}" "CLI about string version"
check_file_contains "docs/changelog.md" "v${VERSION}" "docs changelog current release"

if [[ "$fail" -ne 0 ]]; then
  echo "Documentation version check failed." >&2
  exit 1
fi

echo "Documentation version check passed."
