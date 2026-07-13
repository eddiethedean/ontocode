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

TAGGED_VERSION="$(tr -d '[:space:]' < docs/TAGGED_RELEASE)"
if [[ -z "$TAGGED_VERSION" ]]; then
  echo "error: docs/TAGGED_RELEASE is empty" >&2
  exit 1
fi
TAGGED_MINOR="${TAGGED_VERSION%.*}"

echo "Checking documentation for workspace ${VERSION} (latest tagged: ${TAGGED_VERSION})..."

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
if ! grep -F -- "--version ${TAGGED_VERSION}" README.md >/dev/null; then
  echo "FAIL: README public install pin — expected --version ${TAGGED_VERSION}" >&2
  fail=1
else
  echo "ok: README public install pin"
fi
if ! grep -F -- "--version ${TAGGED_VERSION}" docs/getting-started.md >/dev/null; then
  echo "FAIL: getting-started install pin — expected --version ${TAGGED_VERSION}" >&2
  fail=1
else
  echo "ok: getting-started install pin"
fi
check_file_contains "docs/index.md" "Latest tagged v${TAGGED_VERSION}" "docs index hero tagged version"
check_file_contains "extension/README.md" "v${VERSION}" "extension README version"
check_file_contains "extension/package.json" "\"version\": \"${VERSION}\"" "extension package.json version"
EXT_LOCK_VERSION="$(grep -m1 -E '"version"' extension/package-lock.json | sed -E 's/.*"([^"]+)".*/\1/')"
if [[ "$EXT_LOCK_VERSION" != "$VERSION" ]]; then
  echo "FAIL: extension/package-lock.json version ($EXT_LOCK_VERSION) != package.json ($VERSION)" >&2
  fail=1
else
  echo "ok: extension lockfile version matches package.json"
fi
check_file_contains "docs/guides/enterprise-eval.md" "v${TAGGED_VERSION}" "enterprise eval version"
MINOR_VERSION="${VERSION%.*}"
check_file_contains "SECURITY.md" "${TAGGED_MINOR}\.x" "SECURITY.md tagged supported version"
if [[ "$VERSION" != "$TAGGED_VERSION" ]]; then
  check_file_contains "SECURITY.md" "${MINOR_VERSION}\.x.*unreleased" "SECURITY.md unreleased workspace version note"
fi
check_file_contains "docs/release-integrity.md" "VERSION=${TAGGED_VERSION}" "release-integrity example version"
check_file_contains "docs/TAGGED_RELEASE" "${TAGGED_VERSION}" "TAGGED_RELEASE file"
check_file_contains "mkdocs.yml" "site_url: https://ontocode-vs.readthedocs.io/" "mkdocs site_url matches RTD"
check_file_contains "README.md" "readthedocs.org/projects/ontocode-vs/badge" "RTD docs badge slug"

# Reference page titles must match latest tagged release (public install target)
for file in docs/authoring.md docs/sql-reference.md docs/sparql-reference.md docs/patch-reference.md docs/cli-reference.md docs/errors.md; do
  if grep -qE "^# .+ \(OntoCore v0\.5\)" "$file"; then
    echo "FAIL: stale v0.5 title in $file" >&2
    fail=1
  elif ! grep -qE "^# .+ \(Onto(Index|Core) v${TAGGED_MINOR}\)" "$file"; then
    echo "FAIL: reference title in $file should mention OntoCore v${TAGGED_MINOR} (latest tagged)" >&2
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

if rg -q 'VERSION=0\.10\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.10.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.10\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.10.0 install pins"
fi

if rg -q '--version 0\.10\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.10.0 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.10\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.10.0 install pins"
fi

if rg -q 'VERSION=0\.11\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.11.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.11\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.11.0 install pins"
fi

if rg -q '--version 0\.11\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.11.0 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.11\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.11.0 install pins"
fi

if rg -q 'VERSION=0\.11\.1' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.11.1 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.11\.1' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.11.1 install pins"
fi

if rg -q '--version 0\.11\.1' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.11.1 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.11\.1' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.11.1 install pins"
fi

if rg -q 'VERSION=0\.11\.2' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.11.2 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.11\.2' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.11.2 install pins"
fi

if rg -q '--version 0\.11\.2' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.11.2 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.11\.2' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.11.2 install pins"
fi

if rg -q 'VERSION=0\.11\.3' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.11.3 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.11\.3' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.11.3 install pins"
fi

if rg -q '--version 0\.11\.3' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.11.3 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.11\.3' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.11.3 install pins"
fi

if rg -q 'VERSION=0\.12\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.12.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.12\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.12.0 install pins"
fi

if rg -q '--version 0\.12\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.12.0 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.12\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.12.0 install pins"
fi

if rg -q 'VERSION=0\.13\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale VERSION=0.13.0 found outside changelog/migration/design" >&2
  rg -n 'VERSION=0\.13\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale VERSION=0.13.0 install pins"
fi

if rg -q '--version 0\.13\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale --version 0.13.0 install pin found outside changelog/migration/design" >&2
  rg -n '--version 0\.13\.0' "${STALE_PIN_PATHS[@]}" --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale --version 0.13.0 install pins"
fi

if rg -q 'latest \*\*v0\.11\.x\*\* tag' README.md docs extension crates .github --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' 2>/dev/null; then
  echo "FAIL: stale v0.11.x release tag reference in user-facing docs" >&2
  fail=1
else
  echo "ok: no stale v0.11.x release tag references"
fi
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

# User-facing docs must not claim 0.10.x is the current release
for file in "${USER_FACING_DOCS[@]}" docs/guides/protege-decision.md docs/guides/production-evidence.md docs/guides/release-timeline.md docs/guides/platform-compatibility.md docs/guides/obo-workflow.md docs/guides/lgpl-compliance.md docs/authoring.md docs/patch-reference.md docs/guides/enterprise-eval.md docs/guides/protege-migration.md docs/guides/protege-coexistence.md docs/ontocore/index.md docs/ontocore/rust-api.md docs/ontocode/feature-tour.md docs/architecture.md docs/vision.md docs/lsp-api.md docs/errors.md docs/webview-protocol.md docs/guides/robot-interop.md docs/guides/enterprise-deployment.md docs/guides/performance-sizing.md docs/ci-integration.md docs/guides/first-success.md docs/ontocode/semantic-diff.md; do
  if grep -qE '0\.10\.x \(current\)|0\.10\.0 \| Current|ships in v0\.10\.0|OntoCore v0\.10\.0|OntoCode v0\.10\.0|at \*\*0\.10\.x\*\*|for OntoCode \*\*v0\.10\.0\*\*|OntoCore \*\*v0\.10\.0\*\*|Current release: v0\.10\.0|What v0\.10\.0 delivers|OntoCode v0\.10 is|OntoCode v0\.10 targets|OntoCode v0\.10 supports|evaluating OntoCode \*\*v0\.10\*\*|OntoCode \*\*v0\.10\*\*|OntoCore v0\.10\.0|OntoCode v0\.10\.0\+' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.10.x current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.10.x current-release claims in user-facing docs"
fi

# User-facing docs must not claim 0.11.0 is the current release (0.11.3+)
for file in "${USER_FACING_DOCS[@]}" docs/guides/protege-decision.md docs/guides/production-evidence.md docs/guides/release-timeline.md docs/guides/platform-compatibility.md docs/guides/obo-workflow.md docs/guides/lgpl-compliance.md docs/authoring.md docs/patch-reference.md docs/guides/enterprise-eval.md docs/guides/protege-migration.md docs/guides/protege-coexistence.md docs/ontocore/index.md docs/ontocore/rust-api.md docs/ontocode/feature-tour.md docs/architecture.md docs/vision.md docs/lsp-api.md docs/errors.md docs/webview-protocol.md docs/guides/robot-interop.md docs/guides/enterprise-deployment.md docs/guides/performance-sizing.md docs/ci-integration.md docs/guides/first-success.md docs/ontocode/semantic-diff.md docs/SHIPPED.md docs/index.md README.md extension/README.md; do
  if grep -qE '0\.11\.0 \| Current|Current release: v0\.11\.0|What ships today \(v0\.11\.0\)|ships in v0\.11\.0|OntoCore v0\.11\.0|OntoCode v0\.11\.0|for OntoCode \*\*v0\.11\.0\*\*|OntoCore \*\*v0\.11\.0\*\*|documentation · v0\.11\.0' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.11.0 current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.11.0 current-release claims in user-facing docs"
fi

# User-facing docs must not claim 0.11.1 is the current release (0.11.3+)
for file in "${USER_FACING_DOCS[@]}" docs/guides/protege-decision.md docs/guides/production-evidence.md docs/guides/release-timeline.md docs/guides/platform-compatibility.md docs/guides/obo-workflow.md docs/guides/lgpl-compliance.md docs/authoring.md docs/patch-reference.md docs/guides/enterprise-eval.md docs/guides/protege-migration.md docs/guides/protege-coexistence.md docs/ontocore/index.md docs/ontocore/rust-api.md docs/ontocode/feature-tour.md docs/architecture.md docs/vision.md docs/lsp-api.md docs/errors.md docs/webview-protocol.md docs/guides/robot-interop.md docs/guides/enterprise-deployment.md docs/guides/performance-sizing.md docs/ci-integration.md docs/guides/first-success.md docs/ontocode/semantic-diff.md docs/SHIPPED.md docs/index.md README.md extension/README.md; do
  if grep -qE '0\.11\.1 \| Current|Current release: v0\.11\.1|What ships today \(v0\.11\.1\)|ships in v0\.11\.1|OntoCore v0\.11\.1|OntoCode v0\.11\.1|for OntoCode \*\*v0\.11\.1\*\*|OntoCore \*\*v0\.11\.1\*\*|documentation · v0\.11\.1|What.s included in v0\.11\.1' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.11.1 current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.11.1 current-release claims in user-facing docs"
fi

# User-facing docs must not claim 0.11.2 is the current release (0.11.3+)
for file in "${USER_FACING_DOCS[@]}" docs/guides/protege-decision.md docs/guides/production-evidence.md docs/guides/release-timeline.md docs/guides/platform-compatibility.md docs/guides/obo-workflow.md docs/guides/lgpl-compliance.md docs/authoring.md docs/patch-reference.md docs/guides/enterprise-eval.md docs/guides/protege-migration.md docs/guides/protege-coexistence.md docs/ontocore/index.md docs/ontocore/rust-api.md docs/ontocode/feature-tour.md docs/architecture.md docs/vision.md docs/lsp-api.md docs/errors.md docs/webview-protocol.md docs/guides/robot-interop.md docs/guides/enterprise-deployment.md docs/guides/performance-sizing.md docs/ci-integration.md docs/guides/first-success.md docs/ontocode/semantic-diff.md docs/SHIPPED.md docs/index.md README.md extension/README.md; do
  if grep -qE '0\.11\.2 \| Current|Current release: v0\.11\.2|What ships today \(v0\.11\.2\)|ships in v0\.11\.2|OntoCore v0\.11\.2|OntoCode v0\.11\.2|for OntoCode \*\*v0\.11\.2\*\*|OntoCore \*\*v0\.11\.2\*\*|documentation · v0\.11\.2|What.s included in v0\.11\.2' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.11.2 current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.11.2 current-release claims in user-facing docs"
fi

# User-facing docs must not claim 0.15.x is the current release (0.16+)
CURRENT_RELEASE_STALE_PATHS=(
  ROADMAP.md docs/roadmap.md docs/roadmap-hub.md docs/index.md
  docs/design/README.md extension/README.md docs/platform/OVERVIEW.md
  ARCHITECTURE.md docs/architecture.md
)
for file in "${CURRENT_RELEASE_STALE_PATHS[@]}"; do
  if grep -qE '\*\*Current release:\*\* v0\.15\.0|v0\.15 ships today|v0\.15 foundation shipped|What.s included in v0\.15' "$file" 2>/dev/null; then
    echo "FAIL: stale 0.15.x current-release claim in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: no stale 0.15.x current-release claims in user-facing docs"
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
  if grep -qE 'OntoCore v0\.10' "$file" 2>/dev/null; then
    echo "FAIL: stale OntoCore v0.10 status banner in $file" >&2
    fail=1
  fi
  if grep -qE 'OntoCore v0\.11\.0' "$file" 2>/dev/null; then
    echo "FAIL: stale OntoCore v0.11.0 status banner in $file" >&2
    fail=1
  fi
  if grep -qE 'OntoCore v0\.11\.1' "$file" 2>/dev/null; then
    echo "FAIL: stale OntoCore v0.11.1 status banner in $file" >&2
    fail=1
  fi
  if grep -qE 'OntoCore v0\.11\.2' "$file" 2>/dev/null; then
    echo "FAIL: stale OntoCore v0.11.2 status banner in $file" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: reference pages have no OntoCore v0.8/v0.9/v0.10/v0.11.0/v0.11.1/v0.11.2 banners"
fi

check_file_contains ".github/workflows/release.yml" "publish_with_pause ontocore-obo" "release.yml publishes ontocore-obo"
check_file_contains ".github/workflows/release.yml" "publish_with_pause ontocore-edit" "release.yml publishes ontocore-edit"
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

# docs/security.md supported versions must match SECURITY.md for tagged minor
if ! grep -q "${TAGGED_MINOR}\.x   | Yes" docs/security.md; then
  echo "FAIL: docs/security.md should list ${TAGGED_MINOR}.x as supported (Yes)" >&2
  fail=1
else
  echo "ok: docs/security.md tagged supported version"
fi

# v0.8 docs added in adoption review
for file in docs/guides/refactoring.md docs/migration/v0.8.md docs/migration/v0.9.md docs/migration/v0.10.md docs/examples/refactoring.md docs/ontocode/semantic-diff.md docs/ontocore/rust-api.md docs/guides/protege-migration.md docs/ontocode/feature-tour.md; do
  if [[ ! -f "$file" ]]; then
    echo "FAIL: missing required doc $file" >&2
    fail=1
  else
    echo "ok: $file exists"
  fi
done

check_file_contains "docs/faq.md" "0\.14\.x" "faq crate version"
check_file_contains "docs/guides/release-timeline.md" "${VERSION}.*Current" "release-timeline current version"
check_file_contains "docs/guides/release-timeline.md" "v0\.12.*Shipped" "release-timeline v0.12 shipped"

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

# LSP API must document shipped completion and codeAction (v0.11)
if grep -qE 'Completion \| Planned' docs/lsp-api.md 2>/dev/null; then
  echo "FAIL: docs/lsp-api.md still lists completion as Planned" >&2
  fail=1
else
  echo "ok: lsp-api completion documented as implemented"
fi
if ! grep -q 'textDocument/codeAction' docs/lsp-api.md; then
  echo "FAIL: docs/lsp-api.md missing textDocument/codeAction section" >&2
  fail=1
else
  echo "ok: lsp-api codeAction documented"
fi

check_file_contains "docs/guides/production-readiness.md" "${TAGGED_MINOR}\.x \\(latest tagged\\)" "production-readiness tagged minor"
check_file_contains "docs/ontocore/index.md" "v${TAGGED_VERSION}" "ontocore index tagged version"
check_file_contains "docs/ontocore/rust-api.md" "ontocore = \"${TAGGED_MINOR}\"" "rust-api version pin"
check_file_contains "docs/ontocore/crate-map.md" "ontocore = \"${TAGGED_MINOR}\"" "crate-map version pin"
check_file_contains "docs/ontocode/manage-imports.md" "Manage Imports" "manage-imports guide"
check_file_contains "mkdocs.yml" "ontocode/manage-imports.md" "mkdocs manage-imports guide"
check_file_contains "mkdocs.yml" "migration/v0.14.md" "mkdocs v0.14 migration guide"
check_file_contains "mkdocs.yml" "migration/v0.17.md" "mkdocs v0.17 migration guide"
check_file_contains "mkdocs.yml" "migration/v0.18.md" "mkdocs v0.18 migration guide"
check_file_contains "mkdocs.yml" "v0\\.15 → v0\\.16" "mkdocs v0.16 migration in Help nav"
check_file_contains "docs/guides/production-readiness.md" "v${TAGGED_VERSION}" "production-readiness version"
check_file_contains "mkdocs.yml" "ontocore/rust-api.md" "mkdocs Rust API reference"
check_file_contains "mkdocs.yml" "guides/protege-migration.md" "mkdocs Protégé migration guide"
check_file_contains "mkdocs.yml" "ontocode/feature-tour.md" "mkdocs feature tour"
check_file_contains "mkdocs.yml" "guides/plugins.md" "mkdocs plugins guide"
check_file_contains "mkdocs.yml" "guides/docs-export.md" "mkdocs docs export guide"
check_file_contains "mkdocs.yml" "guides/which-artifact.md" "mkdocs which-artifact in Get started"
check_file_contains "mkdocs.yml" "documentation-index.md" "mkdocs documentation index in Get started"
check_file_contains "mkdocs.yml" "guides/plugins.md" "mkdocs plugins guide in Contribute"
check_file_contains "mkdocs.yml" "known-limitations.md" "mkdocs known limitations"
check_file_contains "mkdocs.yml" "Reference:" "mkdocs Reference tab"
check_file_contains "docs/guides/rust-crates.md" "ontocore = \"${TAGGED_MINOR}\"" "rust-crates version pin"

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

check_file_contains "docs/guides/obo-workflow.md" "OBO write-back" "obo-workflow documents OBO write-back"
if grep -qE 'read-only in the Entity Inspector|Write-back in VS Code remains \*\*Turtle only\*\*' docs/guides/obo-workflow.md; then
  echo "FAIL: docs/guides/obo-workflow.md still claims OBO inspector is read-only" >&2
  fail=1
else
  echo "ok: obo-workflow OBO edit status"
fi
check_file_contains "docs/guides/protege-coexistence.md" "v0\.14" "protege-coexistence v0.14"
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

check_file_contains "extension/package.json" "guides/first-success/" "extension homepage first-success tutorial"
check_file_contains "extension/README.md" "ontocode/vscode-extension/" "extension README VS Code docs path"
check_file_contains "docs/guides/vscode-extension.md" "ontocode/vscode-extension" "vscode hub redirect"
check_file_contains "docs/guides/rust-crates.md" "ontocode/vscode-extension" "rust hub cross-link"
check_file_contains "crates/ontocore-cli/src/main.rs" "OntoCode v${VERSION%.*}" "CLI about string version"
check_file_contains "docs/changelog.md" "v${VERSION}" "docs changelog current release"

for pair in "VISION.md:docs/vision.md:Build the modern open-source platform" \
              "ROADMAP.md:docs/roadmap.md:v0.11 — Editor depth & distribution"; do
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

# ARCHITECTURE.md is a root pointer; canonical body lives in docs/architecture.md
check_file_contains "ARCHITECTURE.md" "docs/architecture.md" "ARCHITECTURE.md points to canonical docs copy"
check_file_contains "docs/architecture.md" "Ontologos thinks" "docs architecture responsibility line"

check_file_contains "docs/roadmap.md" "Shipped releases \\(v0.1–v0.19\\)" "docs roadmap shipped section"
check_file_contains "ROADMAP.md" "Shipped releases \\(v0.1–v0.19\\)" "ROADMAP.md shipped section"
check_file_contains "ROADMAP.md" "v0.14 — Plugin host MVP \\(shipped\\)" "ROADMAP.md v0.14 shipped section"
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

# User-facing crate pins must not reference a previous minor release
CRATE_PIN_PATHS=(docs README.md extension crates CONTRIBUTING.md)
if rg -q 'ontocore = "0\.9"' "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' 2>/dev/null; then
  echo "FAIL: stale ontocore = \"0.9\" pin found outside migration/design/changelog" >&2
  rg -n 'ontocore = "0\.9"' "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale ontocore = \"0.9\" user-facing pins"
fi

if rg -q 'ontocore = "0\.10"' "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' 2>/dev/null; then
  echo "FAIL: stale ontocore = \"0.10\" pin found outside migration/design/changelog" >&2
  rg -n 'ontocore = "0\.10"' "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale ontocore = \"0.10\" user-facing pins"
fi

if rg -q 'ontocore = "0\.11"' "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' 2>/dev/null; then
  echo "FAIL: stale ontocore = \"0.11\" pin found outside migration/design/changelog" >&2
  rg -n 'ontocore = "0\.11"' "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale ontocore = \"0.11\" user-facing pins"
fi

# Stale crate pins for previous minors (0.14–0.18) when current is newer
PREV_MINOR_MAJOR="${MINOR_VERSION%%.*}"
PREV_MINOR_MINOR="${MINOR_VERSION#*.}"
if [[ "$PREV_MINOR_MAJOR" == "0" ]] && [[ "$PREV_MINOR_MINOR" -ge 17 ]]; then
  for stale in 14 15 16 17 18; do
    if rg -q "ontocore = \"0\\.${stale}\"" "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' 2>/dev/null; then
      echo "FAIL: stale ontocore = \"0.${stale}\" pin found outside migration/design/changelog" >&2
      rg -n "ontocore = \"0\\.${stale}\"" "${CRATE_PIN_PATHS[@]}" --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' 2>/dev/null || true
      fail=1
    fi
  done
  if [[ "$fail" -eq 0 ]]; then
    echo "ok: no stale ontocore = \"0.14\"–\"0.18\" user-facing pins"
  fi
fi

# Stale --version install pins for previous minors (0.14–0.18)
INSTALL_PIN_EXCLUDES=(--glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**')
if [[ "$PREV_MINOR_MAJOR" == "0" ]] && [[ "$PREV_MINOR_MINOR" -ge 17 ]]; then
  for stale in 14 15 16 17 18; do
    if rg -q "--version 0\\.${stale}\\." "${STALE_PIN_PATHS[@]}" "${INSTALL_PIN_EXCLUDES[@]}" 2>/dev/null; then
      echo "FAIL: stale --version 0.${stale}.x install pin found outside migration/design/changelog" >&2
      rg -n "--version 0\\.${stale}\\." "${STALE_PIN_PATHS[@]}" "${INSTALL_PIN_EXCLUDES[@]}" 2>/dev/null || true
      fail=1
    fi
  done
  if [[ "$fail" -eq 0 ]]; then
    echo "ok: no stale --version 0.14–0.18 install pins"
  fi
fi

# Stale release-tag guidance (must say current minor, not an older one)
if rg -q 'latest \*\*v0\.15\.x\*\* tag|latest \*\*v0\.14\.x\*\* tag|latest \*\*v0\.16\.x\*\* tag' docs/getting-started.md docs/guides README.md extension 2>/dev/null; then
  echo "FAIL: stale v0.14/v0.15/v0.16 release tag reference in install docs (expected v${MINOR_VERSION}.x)" >&2
  rg -n 'latest \*\*v0\.1[4-6]\.x\*\* tag' docs/getting-started.md docs/guides README.md extension 2>/dev/null || true
  fail=1
else
  echo "ok: no stale v0.14–v0.16 release tag references"
fi
check_file_contains "docs/getting-started.md" "latest \\*\\*v${TAGGED_MINOR}\\.x\\*\\* tag" "getting-started Path D current tag"
check_file_contains "docs/guides/which-artifact.md" "ontocore = \"${TAGGED_MINOR}\"" "which-artifact crate pin"
check_file_contains "docs/guides/api-stability.md" "Published crates use \\*\\*${TAGGED_MINOR}\\.x\\*\\*" "api-stability published crates minor"
check_file_contains "docs/ci-integration.md" "VERSION=${TAGGED_VERSION}" "ci-integration release binary pin"
check_file_contains "docs/faq.md" "version ${TAGGED_VERSION}" "faq CI version pin"
check_file_contains "docs/known-limitations.md" "Latest tagged release: v${TAGGED_VERSION}" "known-limitations tagged release banner"
check_file_contains "docs/index.md" "Latest tagged v${TAGGED_VERSION}" "docs index tagged release banner"
if grep -qE 'semantic diff\) is the v1\.0 goal|Full Protégé parity \(.*semantic diff\)' docs/faq.md 2>/dev/null; then
  echo "FAIL: docs/faq.md contradicts SHIPPED on semantic diff" >&2
  fail=1
else
  echo "ok: faq semantic diff status"
fi

# start.md must not list multi-root support under 'When not to use'
if grep -A20 'When not to use OntoCode' docs/start.md | grep -qE 'Multi-root VS Code workspaces are supported'; then
  echo "FAIL: docs/start.md lists multi-root under 'When not to use'" >&2
  fail=1
else
  echo "ok: start.md multi-root placement"
fi

check_file_contains "docs/ui/ROADMAP_MAPPING.md" "Master checklist" "ui roadmap master checklist"
check_file_contains "docs/start.md" "guides/which-artifact.md" "start.md links which-artifact detail"
check_file_contains "docs/guides/which-artifact.md" "Which artifact do I need" "which-artifact guide title"

# SHIPPED known limitations must reflect Turtle + OBO write-back (v0.12)
if grep -qE '\| Write-back \| \*\*Turtle only\*\*' docs/SHIPPED.md 2>/dev/null; then
  echo "FAIL: docs/SHIPPED.md known limitations still say Turtle-only write-back" >&2
  fail=1
else
  echo "ok: SHIPPED write-back limitations"
fi

# User-facing docs must not claim OBO inspector is read-only (v0.12 write-back)
OBO_READONLY_PATHS=(docs README.md extension crates .github)
if rg -q 'read-only in inspector|OBO is read-only|write-back: Turtle only|write-back is \*\*Turtle only\*\*|writes Turtle only' "${OBO_READONLY_PATHS[@]}" \
  --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/adr/**' 2>/dev/null; then
  echo "FAIL: stale OBO read-only or Turtle-only write-back claim in user-facing docs" >&2
  rg -n 'read-only in inspector|OBO is read-only|write-back: Turtle only|write-back is \*\*Turtle only\*\*|writes Turtle only' "${OBO_READONLY_PATHS[@]}" \
    --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/adr/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale OBO read-only claims"
fi

# Property chain editing shipped in v0.12 — docs must not say view-only
if rg -q 'chains view-only|property chains are view-only|chains are view-only' docs README.md extension crates \
  --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale property chains view-only claim in user-facing docs" >&2
  rg -n 'chains view-only|property chains are view-only|chains are view-only' docs README.md extension crates \
    --glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale property chains view-only claims"
fi

# Architecture banner: during an unreleased minor, allow "in progress" wording
if grep -qE "v${MINOR_VERSION} ships today" ARCHITECTURE.md 2>/dev/null; then
  echo "ok: ARCHITECTURE.md v${MINOR_VERSION} ships today banner"
elif grep -qE "v${MINOR_VERSION} in progress" ARCHITECTURE.md 2>/dev/null; then
  echo "ok: ARCHITECTURE.md v${MINOR_VERSION} in progress banner"
else
  echo "FAIL: ARCHITECTURE.md v${MINOR_VERSION} banner — expected 'ships today' or 'in progress'" >&2
  fail=1
fi
if grep -qE "v${MINOR_VERSION} ships today" docs/architecture.md 2>/dev/null; then
  echo "ok: docs/architecture.md v${MINOR_VERSION} ships today banner"
elif grep -qE "v${MINOR_VERSION} in progress" docs/architecture.md 2>/dev/null; then
  echo "ok: docs/architecture.md v${MINOR_VERSION} in progress banner"
else
  echo "FAIL: docs/architecture.md v${MINOR_VERSION} banner — expected 'ships today' or 'in progress'" >&2
  fail=1
fi

# Stale CLI alias notes
if rg -q 'ontocore alias is planned' docs --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null; then
  echo "FAIL: stale 'ontocore alias is planned' note in docs" >&2
  rg -n 'ontocore alias is planned' docs --glob '!**/migration/**' --glob '!**/design/**' 2>/dev/null || true
  fail=1
else
  echo "ok: no stale ontocore alias notes"
fi

# MkDocs strict: markdown links must not point at directories (use README.md)
if rg -q '\]\(\.\./ui/\)|\]\(ui/\)[^R]' docs --glob '*.md' 2>/dev/null; then
  echo "FAIL: directory-only markdown link (use ui/README.md not ui/)" >&2
  rg -n '\]\(\.\./ui/\)|\]\(ui/\)[^R]' docs --glob '*.md' 2>/dev/null || true
  fail=1
else
  echo "ok: no directory-only ui/ markdown links"
fi

# design/ARCHITECTURE.md shipped banner: current minor if released, else previous tagged minor
PREV_MINOR_VERSION="$(python3 - <<PY
maj, minor, *_ = "${VERSION}".split(".")
print(f"{maj}.{int(minor)-1}")
PY
)"
if grep -qE "Shipped through v${MINOR_VERSION}:" docs/design/ARCHITECTURE.md 2>/dev/null; then
  echo "ok: design ARCHITECTURE shipped banner (current minor)"
elif grep -qE "Shipped through v${PREV_MINOR_VERSION}:" docs/design/ARCHITECTURE.md 2>/dev/null \
  && grep -qE "Unreleased on v${MINOR_VERSION}" docs/design/ARCHITECTURE.md 2>/dev/null; then
  echo "ok: design ARCHITECTURE shipped banner (previous minor + unreleased note)"
elif grep -qE 'Shipped through v0\.(1[0-7]|[0-9]):' docs/design/ARCHITECTURE.md 2>/dev/null; then
  echo "FAIL: docs/design/ARCHITECTURE.md shipped banner still says through an older minor (expected v${MINOR_VERSION} or v${PREV_MINOR_VERSION} + unreleased)" >&2
  fail=1
else
  echo "FAIL: docs/design/ARCHITECTURE.md must say Shipped through v${MINOR_VERSION}: (or v${PREV_MINOR_VERSION}: with Unreleased on v${MINOR_VERSION})" >&2
  fail=1
fi

# LSP API must document OBO write-back alongside Turtle
for file in docs/lsp-api.md docs/ontocore/lsp.md; do
  if grep -qE 'applyAxiomPatch.*Turtle write-back|Turtle write-back only|true` for Turtle write-back' "$file" 2>/dev/null; then
    echo "FAIL: $file still implies Turtle-only applyAxiomPatch" >&2
    fail=1
  fi
done
if [[ "$fail" -eq 0 ]]; then
  echo "ok: LSP docs mention Turtle+OBO write-back"
fi

# patch-reference intro must mention OBO
if grep -qE '^Turtle write-back uses a JSON array' docs/patch-reference.md 2>/dev/null; then
  echo "FAIL: docs/patch-reference.md intro still says Turtle write-back only" >&2
  fail=1
else
  echo "ok: patch-reference intro covers Turtle+OBO"
fi

# FAQ inspector edit must not contradict OBO write-back (v0.12)
if grep -A2 'I cannot edit in the Entity Inspector' docs/faq.md | grep -qE 'Turtle \(\`\.ttl\`\) only' 2>/dev/null; then
  echo "FAIL: docs/faq.md inspector edit answer still says Turtle-only" >&2
  fail=1
else
  echo "ok: faq inspector edit answer"
fi

check_file_contains "docs/roadmap-hub.md" "v${VERSION}" "roadmap-hub current release"
check_file_contains "docs/guides/api-stability.md" "API stability" "api stability page"
check_file_contains "docs/guides/legacy-guide-urls.md" "Legacy guide URLs" "legacy guide redirects page"
check_file_contains "docs/ontocode/obo-authoring.md" "OBO authoring" "obo authoring guide"
check_file_contains "mkdocs.yml" "roadmap-hub.md" "mkdocs roadmap hub"
check_file_contains "mkdocs.yml" "guides/api-stability.md" "mkdocs api stability"
check_file_contains "mkdocs.yml" "guides/legacy-guide-urls.md" "mkdocs legacy redirects"
check_file_contains "mkdocs.yml" "ontocode/obo-authoring.md" "mkdocs obo authoring"

check_file_contains "docs/cursor-prompts/README.md" "Cursor implementation prompts" "cursor prompts index"
check_file_contains "docs/platform/OVERVIEW.md" "Platform overview" "platform overview"
check_file_contains "docs/platform/ONTOUI.md" "OntoUI" "platform OntoUI doc"
check_file_contains "docs/adr/README.md" "Product & platform ADRs" "product adr index"
check_file_contains "docs/glossary.md" "OntoUI" "glossary OntoUI term"
check_file_contains "docs/documentation-index.md" "Documentation index" "docs documentation index"
check_file_contains "docs/documentation-index.md" "v${TAGGED_VERSION}" "documentation-index tagged release"
check_file_contains "mkdocs.yml" "guides/versions-and-channels.md" "mkdocs versions and channels"
check_file_contains "docs/guides/versions-and-channels.md" "Versions and channels" "versions and channels page"
check_file_contains "mkdocs.yml" "guides/architecture-tour.md" "mkdocs architecture tour"
check_file_contains "mkdocs.yml" "guides/testing-matrix.md" "mkdocs testing matrix"
check_file_contains "mkdocs.yml" "guides/procurement-appendix.md" "mkdocs procurement appendix"
check_file_contains "mkdocs.yml" "design/adr/README.md" "mkdocs engineering ADRs"
check_file_contains "mkdocs.yml" "adr/README.md" "mkdocs product ADRs"
check_file_contains "docs/guides/architecture-tour.md" "Architecture tour" "architecture tour page"
check_file_contains "docs/guides/testing-matrix.md" "Contributor testing matrix" "testing matrix page"
check_file_contains "docs/design/adr/README.md" "0020-semantic-transaction" "ADR-0020 in index"
check_file_contains "mkdocs.yml" "Catalog SQL" "mkdocs catalog SQL reference label"
check_file_contains "docs/engineering.md" "Engineering docs \\(GitHub\\)" "engineering pointer page"
check_file_contains "docs/known-limitations.md" "Known limitations" "known limitations page"
check_file_contains "docs/ui/README.md" "OntoUI" "ui readme OntoUI term"

check_file_contains "mkdocs.yml" "guides/owl-xml-workflow.md" "mkdocs owl-xml workflow guide"
check_file_contains "mkdocs.yml" "v0\\.14 → v0\\.15" "mkdocs v0.15 migration in Help nav"
check_file_contains "docs/guides/owl-xml-workflow.md" "read-only catalog" "owl-xml workflow guide"
check_file_contains "docs/ontocore/rust-api.md" "Book ↔ docs.rs crosswalk" "rust-api docs.rs crosswalk"
check_file_contains "docs/troubleshooting.md" "Where to start" "troubleshooting decision tree"
check_file_contains "docs/platform/OVERVIEW.md" "v0.20 foundation shipped" "platform overview shipped banner"

# vision.md must reference current shipped release (not v0.11 or v0.12)
for file in docs/vision.md VISION.md; do
  if grep -qE 'what ships in \*\*v0\.11\*\*|what ships in \*\*v0\.12\*\*|ships in \*\*v0\.11\*\*|ships in \*\*v0\.12\*\*' "$file" 2>/dev/null; then
    echo "FAIL: $file vision banner references stale release (expected v${VERSION%.*})" >&2
    fail=1
  fi
done
if ! grep -qF "what ships in **v${VERSION%.*}**" docs/vision.md 2>/dev/null || \
   ! grep -qF "what ships in **v${VERSION%.*}**" VISION.md 2>/dev/null; then
  echo "FAIL: docs/vision.md and VISION.md must say what ships in v${VERSION%.*}" >&2
  fail=1
else
  echo "ok: vision banner sync v${VERSION%.*}"
fi

check_file_contains "docs/glossary.md" "\\*\\*Implemented\\*\\* \\(v${TAGGED_MINOR}\\)" "glossary OntoCore/OntoCode version"
check_file_contains "docs/glossary.md" "\\*\\*Shipped\\*\\* \\(v${TAGGED_MINOR}\\)" "glossary WorkspaceStore shipped"
check_file_contains "docs/vscode-install.md" "1.85" "vscode-install minimum VS Code version"
check_file_contains "docs/documentation-index.md" "Shipped v${TAGGED_MINOR}" "documentation-index OntoUI shipped"
if grep -q 'Turtle (`.ttl`) only' extension/README.md 2>/dev/null; then
  echo "FAIL: extension/README.md troubleshooting still says Turtle-only inspector" >&2
  fail=1
else
  echo "ok: extension README inspector write-back"
fi
check_file_contains "CONTRIBUTING.md" "build-docs.sh" "contributing documents build-docs script"
check_file_contains "CONTRIBUTING.md" "run-ci-local.sh" "contributing documents local CI script"
check_file_contains "docs/internals.md" "Extension-only" "internals extension-only path"
check_file_contains "docs/guides/lsp-hello-world.md" "ontocore-lsp" "lsp hello-world guide"
check_file_contains "mkdocs.yml" "guides/extension-development.md" "mkdocs extension development guide"
check_file_contains "mkdocs.yml" "guides/lsp-hello-world.md" "mkdocs lsp hello-world guide"
check_file_contains "docs/guides/extension-development.md" "extension/" "extension development guide"
check_file_contains "crates/ontocore-plugin/README.md" "Experimental" "plugin README experimental banner"
check_file_contains "crates/ontocore-obo/README.md" "ontocore-obo" "ontocore-obo README"

# errors.md must reference current release
check_file_contains "docs/errors.md" "v${TAGGED_VERSION}" "errors reference version"

# Canonical SHIPPED matrix must match latest tagged release
check_file_contains "docs/SHIPPED.md" "What ships today \\(v${TAGGED_VERSION}" "SHIPPED header tagged version"
check_file_contains "docs/SHIPPED.md" "Latest tagged: v${TAGGED_VERSION}" "SHIPPED latest tagged line"

# LSP API title matches tagged minor; banner may mention workspace when ahead of tag
check_file_contains "docs/lsp-api.md" "^# OntoCore LSP API \\(v${TAGGED_MINOR}\\)" "lsp-api title minor"
check_file_contains "docs/ontocode/feature-tour.md" "^# OntoCode feature tour \\(current: v${TAGGED_MINOR}\\)" "feature-tour tagged minor"

# rust-library crates claim must match tagged minor
if ! grep -qE "Crates are at \\*\\*${TAGGED_MINOR}\\.x\\*\\*" docs/guides/rust-library.md; then
  echo "FAIL: docs/guides/rust-library.md crates version must be **${TAGGED_MINOR}.x**" >&2
  fail=1
else
  echo "ok: rust-library crates version ${TAGGED_MINOR}.x"
fi

# Public install pins must not target unreleased workspace version
INSTALL_PIN_EXCLUDE_GLOBS=(--glob '!**/changelog.md' --glob '!**/CHANGELOG.md' --glob '!**/migration/**' --glob '!**/design/**' --glob '!**/releasing.md' --glob '!**/TAGGED_RELEASE')
if [[ "$VERSION" != "$TAGGED_VERSION" ]]; then
  if rg -q "--version ${VERSION}" README.md docs extension crates .github "${INSTALL_PIN_EXCLUDE_GLOBS[@]}" 2>/dev/null; then
    echo "FAIL: public --version ${VERSION} install pin (use ${TAGGED_VERSION} from docs/TAGGED_RELEASE)" >&2
    rg -n "--version ${VERSION}" README.md docs extension crates .github "${INSTALL_PIN_EXCLUDE_GLOBS[@]}" 2>/dev/null || true
    fail=1
  else
    echo "ok: no public --version ${VERSION} install pins"
  fi
  if rg -q "VERSION=${VERSION}" README.md docs extension .github "${INSTALL_PIN_EXCLUDE_GLOBS[@]}" 2>/dev/null; then
    echo "FAIL: public VERSION=${VERSION} install pin (use VERSION=${TAGGED_VERSION})" >&2
    rg -n "VERSION=${VERSION}" README.md docs extension .github "${INSTALL_PIN_EXCLUDE_GLOBS[@]}" 2>/dev/null || true
    fail=1
  else
    echo "ok: no public VERSION=${VERSION} install pins"
  fi
  if rg -q "ontocore = \"${MINOR_VERSION}\"" README.md docs extension crates CONTRIBUTING.md "${INSTALL_PIN_EXCLUDE_GLOBS[@]}" 2>/dev/null; then
    echo "FAIL: public ontocore = \"${MINOR_VERSION}\" pin (use \"${TAGGED_MINOR}\" for tagged release)" >&2
    fail=1
  else
    echo "ok: no unreleased ontocore crate pins in user docs"
  fi
fi

# Reject feature-tour / LSP titles pinned to an older minor than tagged (only when workspace == tagged)
if [[ "$VERSION" == "$TAGGED_VERSION" ]]; then
  PREV_MINOR_MINOR="${TAGGED_MINOR##*.}"
  if [[ "$PREV_MINOR_MINOR" =~ ^[0-9]+$ ]] && [[ "$PREV_MINOR_MINOR" -gt 0 ]]; then
    STALE_MINOR="${TAGGED_MINOR%%.*}.$((PREV_MINOR_MINOR - 1))"
    if grep -qE "feature tour \\(current: v${STALE_MINOR}\\)" docs/ontocode/feature-tour.md 2>/dev/null; then
      echo "FAIL: feature-tour still says current: v${STALE_MINOR}" >&2
      fail=1
    fi
    if grep -qE "^# OntoCore LSP API \\(v${STALE_MINOR}\\)" docs/lsp-api.md 2>/dev/null; then
      echo "FAIL: lsp-api title still says v${STALE_MINOR}" >&2
      fail=1
    fi
  fi
fi

# Enterprise eval capability table header must match tagged release
check_file_contains "docs/guides/enterprise-eval.md" "What ships today \\(v${TAGGED_VERSION}" "enterprise-eval capability table version"

# Governance must list tagged minor as current supported stream
if grep -qE "\\*\\*${TAGGED_MINOR}\\.x\\*\\* \\| Yes — current tagged release" docs/guides/governance.md 2>/dev/null; then
  echo "ok: governance tagged release stream"
elif grep -qE "\\*\\*${TAGGED_MINOR}\\.x\\*\\* \\| Yes" docs/guides/governance.md 2>/dev/null; then
  echo "ok: governance tagged release stream"
else
  echo "FAIL: governance must list **${TAGGED_MINOR}.x** as current tagged release" >&2
  fail=1
fi
# Unreleased workspace minor (when ahead of tag) may appear as in-progress
if [[ "$VERSION" != "$TAGGED_VERSION" ]]; then
  if grep -qE "\\*\\*${MINOR_VERSION}\\.x\\*\\* \\| In progress \\(unreleased\\)" docs/guides/governance.md 2>/dev/null; then
    echo "ok: governance in-progress development stream"
  else
    echo "FAIL: governance must list **${MINOR_VERSION}.x** as In progress (unreleased) when workspace > tagged" >&2
    fail=1
  fi
fi

# MkDocs must surface v0.15 migration in Get started / Help
check_file_contains "mkdocs.yml" "migration/v0.16.md" "mkdocs v0.16 migration guide"
check_file_contains "mkdocs.yml" "migration/v0.15.md" "mkdocs v0.15 migration guide"

if [[ "$fail" -ne 0 ]]; then
  echo "Documentation version check failed." >&2
  exit 1
fi

echo "Documentation version check passed."
