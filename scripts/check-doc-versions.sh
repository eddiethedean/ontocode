#!/usr/bin/env bash
# Verify user-facing documentation references the workspace package version.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# Prefer grep -m1 over `grep | head` — under pipefail, GNU grep exits 2 on SIGPIPE.
VERSION="$(grep -m1 -E '^version = ' Cargo.toml | sed -E 's/.*"([^"]+)".*/\1/')"
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
EXT_LOCK_VERSION="$(grep -m1 -E '"version"' extension/package-lock.json | sed -E 's/.*"([^"]+)".*/\1/')"
if [[ "$EXT_LOCK_VERSION" != "$VERSION" ]]; then
  echo "FAIL: extension/package-lock.json version ($EXT_LOCK_VERSION) != package.json ($VERSION)" >&2
  fail=1
else
  echo "ok: extension lockfile version matches package.json"
fi
check_file_contains "docs/guides/enterprise-eval.md" "v${VERSION}" "enterprise eval version"
MINOR_VERSION="${VERSION%.*}"
check_file_contains "SECURITY.md" "${MINOR_VERSION}\.x" "SECURITY.md supported versions"
check_file_contains "docs/release-integrity.md" "VERSION=${VERSION}" "release-integrity example version"
check_file_contains "mkdocs.yml" "site_url: https://ontocode-vs.readthedocs.io/" "mkdocs site_url matches RTD"
check_file_contains "README.md" "readthedocs.org/projects/ontocode-vs/badge" "RTD docs badge slug"

# Reference page titles must match current release
for file in docs/authoring.md docs/sql-reference.md docs/sparql-reference.md docs/patch-reference.md docs/cli-reference.md; do
  if grep -qE "^# .+ \(OntoCore v0\.5\)" "$file"; then
    echo "FAIL: stale v0.5 title in $file" >&2
    fail=1
  elif ! grep -qE "^# .+ \(Onto(Index|Core) v${VERSION%.*}\)" "$file"; then
    echo "FAIL: reference title in $file should mention OntoCore or OntoCore v${VERSION%.*}" >&2
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

if rg -q 'VERSION=0\.8\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.8.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.8\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.8.0 install pins"
fi

if rg -q 'VERSION=0\.9\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.9.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.9\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.9.0 install pins"
fi

if rg -q '--version 0\.9\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.9.0 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.9\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.9.0 install pins"
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
  if grep -qE '0\.7\.x \(current\)|ships in v0\.7\.0|OntoCore v0\.7\.0|OntoCode v0\.7\.0|at \*\*0\.7\.x\*\*' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.7.x current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.7.x current-release claims in user-facing docs"
fi

# User-facing docs must not claim 0.8.x is the current release
for file in "${USER_FACING_DOCS[@]}" docs/guides/protege-decision.md docs/guides/production-evidence.md; do
  if grep -qE '0\.8\.x \(current\)|ships in v0\.8\.0|OntoCore v0\.8\.0|OntoCode v0\.8\.0|at \*\*0\.8\.x\*\*' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.8.x current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.8.x current-release claims in user-facing docs"
fi

# User-facing docs must not claim 0.9.x is the current release
for file in "${USER_FACING_DOCS[@]}" docs/guides/protege-decision.md docs/guides/production-evidence.md docs/guides/release-timeline.md docs/guides/platform-compatibility.md docs/guides/obo-workflow.md docs/guides/lgpl-compliance.md docs/authoring.md docs/patch-reference.md; do
  if grep -qE '0\.9\.x \(current\)|0\.9\.0 \| Current|ships in v0\.9\.0|OntoCore v0\.9\.0|OntoCode v0\.9\.0|at \*\*0\.9\.x\*\*|for OntoCode \*\*v0\.9\.0\*\*|OntoCore \*\*v0\.9\.0\*\*' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.9.x current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.9.x current-release claims in user-facing docs"
fi

# Reference status banners must not contradict OntoCore v{N} titles
for file in docs/authoring.md docs/sql-reference.md docs/sparql-reference.md docs/patch-reference.md docs/lsp-api.md docs/errors.md docs/webview-protocol.md; do
  if grep -qE 'OntoCore v0\.8' "$file" 2>/dev/null; then
    echo "FAIL: stale OntoCore v0.8 status banner in $file" >&2
    fail=1
  fi
  if grep -qE 'OntoCore v0\.9' "$file" 2>/dev/null; then
    echo "FAIL: stale OntoCore v0.9 status banner in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: reference pages have no OntoCore v0.8/v0.9 banners"
fi

check_file_contains ".github/workflows/release.yml" "publish_with_pause ontocore" "release.yml publishes ontocore"

# docs/contributing.md should track root CONTRIBUTING.md (OntoCore branding)
if ! grep -q 'OntoCore' docs/contributing.md; then
  echo "FAIL: docs/contributing.md missing OntoCore branding" >&2
  fail=1
elif ! grep -q 'OntoCore' CONTRIBUTING.md; then
  echo "FAIL: CONTRIBUTING.md missing OntoCore branding" >&2
  fail=1
else
  echo "ok: contributing docs OntoCore branding"
fi

WEBVIEW_PKG_VERSION="$(grep -m1 -E '"version"' extension/webview-ui/package.json | sed -E 's/.*"([^"]+)".*/\1/')"
WEBVIEW_LOCK_VERSION="$(grep -m1 -E '"version"' extension/webview-ui/package-lock.json | sed -E 's/.*"([^"]+)".*/\1/')"
if [[ "$WEBVIEW_PKG_VERSION" != "$WEBVIEW_LOCK_VERSION" ]]; then
  echo "FAIL: extension/webview-ui/package-lock.json version ($WEBVIEW_LOCK_VERSION) != package.json ($WEBVIEW_PKG_VERSION)" >&2
  fail=1
elif [[ "$WEBVIEW_PKG_VERSION" != "$VERSION" ]]; then
  echo "FAIL: extension/webview-ui version ($WEBVIEW_PKG_VERSION) != extension/workspace ($VERSION)" >&2
  fail=1
else
  echo "ok: webview-ui version matches extension and lockfile"
fi

# docs/security.md supported versions must match SECURITY.md for current minor
if ! grep -q "${MINOR_VERSION}\.x   | Yes" docs/security.md; then
  echo "FAIL: docs/security.md should list ${MINOR_VERSION}.x as supported (Yes)" >&2
  fail=1
else
  echo "ok: docs/security.md supported version"
fi

# v0.8 docs added in adoption review
for file in docs/guides/refactoring.md docs/migration/v0.8.md docs/migration/v0.9.md docs/migration/v0.10.md docs/examples/refactoring.md docs/ontocode/semantic-diff.md; do
  if [[ ! -f "$file" ]]; then
    echo "FAIL: missing required doc $file" >&2
    fail=1
  else
    echo "ok: $file exists"
  fi
done

check_file_contains "docs/faq.md" "0\.10\.x" "faq crate version"
check_file_contains "docs/guides/release-timeline.md" "0\.10\.0.*Current" "release-timeline current version"
check_file_contains "docs/guides/release-timeline.md" "v0\.10.*Shipped" "release-timeline v0.10 shipped"

# Stale multi-root limitation (v0.10 indexes all folders)
MULTIROOT_STALE_PATHS=(docs/SHIPPED.md docs/faq.md docs/vscode-install.md docs/guides/first-success.md docs/troubleshooting.md)
for file in "${MULTIROOT_STALE_PATHS[@]}"; do
  if grep -qE 'only the \*\*first\*\* folder is indexed|Only the \*\*first\*\* folder is indexed' "$file" 2>/dev/null; then
    echo "FAIL: stale first-folder-only multi-root claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale first-folder-only multi-root claims"
fi

# LSP API must document shipped semanticDiff (not list as unimplemented)
if grep -q 'getSemanticDiff.*Not implemented\|not implemented yet.*getSemanticDiff\|until that LSP method lands' docs/lsp-api.md 2>/dev/null; then
  echo "FAIL: docs/lsp-api.md still claims semanticDiff is not implemented" >&2
  fail=1
else
  echo "ok: lsp-api semanticDiff documented"
fi
if ! grep -q 'ontocore/semanticDiff' docs/lsp-api.md; then
  echo "FAIL: docs/lsp-api.md missing ontocore/semanticDiff section" >&2
  fail=1
fi

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

check_file_contains "NOTICES" "v${VERSION}" "NOTICES release version"

check_file_contains "docs/guides/protege-coexistence.md" "v0\.10" "protege-coexistence v0.10"
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
check_file_contains "crates/ontocore-cli/src/main.rs" "OntoCode v${VERSION%.*}" "CLI about string version"
check_file_contains "docs/changelog.md" "v${VERSION}" "docs changelog current release"

for pair in "VISION.md:docs/vision.md:Build the modern open-source platform" \
              "ARCHITECTURE.md:docs/architecture.md:Ontologos thinks" \
              "ROADMAP.md:docs/roadmap.md:v0.10 — Semantic Workspace"; do
  root_file="${pair%%:*}"
  rest="${pair#*:}"
  doc_file="${rest%%:*}"
  phrase="${rest#*:}"
  if [[ ! -f "$root_file" ]] || [[ ! -f "$doc_file" ]]; then
    echo "FAIL: missing platform doc $root_file or $doc_file" >&2
    fail=1
  elif ! grep -qF "$phrase" "$root_file" || ! grep -qF "$phrase" "$doc_file"; then
    echo "FAIL: platform doc sync — expected '$phrase' in $root_file and $doc_file" >&2
    fail=1
  else
    echo "ok: platform doc sync $root_file ↔ $doc_file"
  fi
done

check_file_contains "docs/roadmap.md" "Shipped \\(v0.1–v0.9\\)" "docs roadmap shipped section"
check_file_contains "ROADMAP.md" "v1.2 — Ontology Toolchain Platform" "roadmap v1.2 toolchain milestone"
check_file_contains "docs/roadmap.md" "v1.2 — Ontology Toolchain Platform" "docs roadmap v1.2 milestone"
check_file_contains "ROADMAP.md" "owlmake" "roadmap owlmake integration"
check_file_contains "mkdocs.yml" "vision.md" "mkdocs Platform nav"

# User-facing guides must not claim dl/auto are stubbed or not shipped
DL_STUB_GUIDE_PATHS=(
  docs/guides/enterprise-eval.md
  docs/guides/protege-decision.md
  docs/guides/protege-coexistence.md
  docs/guides/production-readiness.md
  docs/guides/release-timeline.md
)
for file in "${DL_STUB_GUIDE_PATHS[@]}"; do
  if grep -qiE 'dl.*stub|auto.*stub|stubbed until OntoLogos 1\.0|Not shipped.*OntoLogos 1\.0|not shipped in v0\.9.*dl|not shipped in v0\.9.*auto' "$file" 2>/dev/null; then
    echo "FAIL: stale dl/auto stub claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale dl/auto stub claims in enterprise guides"
fi

if grep -qiE 'dl.*stub|auto.*stub' docs/workspace-limits.md 2>/dev/null; then
  echo "FAIL: stale dl/auto stub claim in docs/workspace-limits.md" >&2
  fail=1
else
  echo "ok: workspace-limits dl/auto status"
fi

if [[ "$fail" -ne 0 ]]; then
  echo "Documentation version check failed." >&2
  exit 1
fi

echo "Documentation version check passed."
