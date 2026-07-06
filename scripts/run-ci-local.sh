#!/usr/bin/env bash
# Run CI workflow steps locally (mirrors .github/workflows/ci.yml + extension-vscode-e2e darwin-arm64).
set -uo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export ROOT
cd "$ROOT"

# Use workspace target/ (avoids sandbox temp cargo dirs when run from IDE agents).
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"
export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"

PLATFORM="$(uname -s | tr '[:upper:]' '[:lower:]')"
case "$(uname -m)" in
  x86_64) ARCH="x64" ;;
  aarch64 | arm64) ARCH="arm64" ;;
  *) ARCH="$(uname -m)" ;;
esac
EXT_PLATFORM="${PLATFORM}-${ARCH}"

FAILED=()
PASSED=()

run_step() {
  local name="$1"
  shift
  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "▶ ${name}"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  if "$@"; then
    echo "✓ PASS: ${name}"
    PASSED+=("${name}")
  else
    echo "✗ FAIL: ${name}"
    FAILED+=("${name}")
  fi
}

run_step "rustfmt" cargo fmt --all -- --check

run_step "documentation version sync" ./scripts/check-doc-versions.sh

run_step "clippy" \
  cargo clippy --workspace --all-targets --all-features -- -D warnings

run_step "cargo test" bash -c '
  cargo build -p ontocore-lsp -p ontocore-cli --bins
  cargo test --workspace
'

run_step "MSRV (1.88)" bash -c '
  rustup run 1.88 cargo build -p ontocore-lsp -p ontocore-cli --bins
  rustup run 1.88 cargo test --workspace
'

run_step "CLI smoke" bash -c '
  set -euo pipefail
  cargo run -- query fixtures "SELECT * FROM classes"
  cargo run -- validate fixtures
  cargo run -- query fixtures "SELECT code FROM diagnostics"
  cargo run -- sparql fixtures "SELECT ?s WHERE { ?s ?p ?o } LIMIT 1"
'

run_step "release build" bash -c '
  set -euo pipefail
  cargo build --release --locked -p ontocore-cli -p ontocore-lsp --bins
  ./target/release/ontocore inspect fixtures
'

run_step "LSP smoke + reasoner tests" bash -c '
  cargo build --locked -p ontocore-lsp --bins
  cargo test -p ontocode --test lsp_smoke
  cargo test -p ontocode --test lsp_reasoner
'

run_step "webview-ui tests" bash -c '
  cd extension/webview-ui
  npm ci
  npm test
'

run_step "extension build + unit tests" bash -c '
  set -euo pipefail
  cargo build -p ontocore-lsp --bins
  cd extension
  npm ci
  npm run compile
  ONTOCORE_LSP_BIN="$ROOT/target/debug/ontocore-lsp" npm test
'

run_step "extension VSIX unpack e2e" bash -c '
  set -euo pipefail
  cargo build -p ontocore-lsp --bins
  mkdir -p "extension/server/linux-x64"
  cp target/debug/ontocore-lsp extension/server/linux-x64/ontocore-lsp
  chmod +x extension/server/linux-x64/ontocore-lsp
  cd extension
  npx vsce package --no-dependencies -o /tmp/ontocode-ci-local.vsix
  rm -rf /tmp/ontocode-vsix-unpack-local
  unzip -q /tmp/ontocode-ci-local.vsix -d /tmp/ontocode-vsix-unpack-local
  export ONTOCODE_E2E_EXTENSION_ROOT=/tmp/ontocode-vsix-unpack-local/extension
  export ONTOCORE_LSP_BIN="$ROOT/target/debug/ontocore-lsp"
  npm test
'

run_step "cargo audit" cargo audit

run_step "crate packaging dry-run" bash -c '
  set -euo pipefail
  cargo publish -p ontocore-core --dry-run
  cargo publish -p ontocore-robot --dry-run
  cargo build -p ontocore-obo -p ontocore-diagnostics -p ontocore-owl -p ontocore-cli -p ontocore
'

run_step "mkdocs strict build" bash -c '
  pip install -q -r docs/requirements.txt
  mkdocs build --strict
'

run_step "VS Code extension e2e (${EXT_PLATFORM})" bash -c "
  set -euo pipefail
  cargo build -p ontocore-lsp --bins
  ./scripts/prepare-extension-server.sh '${EXT_PLATFORM}'
  chmod -x extension/server/${EXT_PLATFORM}/ontocore-lsp
  cd extension
  npm ci
  npm run compile
  npm run compile:vscode-test
  npm run test:vscode
"

echo ""
echo "════════════════════════════════════════════════════════════"
echo "CI local summary: ${#PASSED[@]} passed, ${#FAILED[@]} failed"
echo "════════════════════════════════════════════════════════════"
if ((${#FAILED[@]} > 0)); then
  echo "Failed:"
  for f in "${FAILED[@]}"; do
    echo "  - ${f}"
  done
  exit 1
fi
echo "All checks passed."
exit 0
