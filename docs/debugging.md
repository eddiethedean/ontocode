# Debugging OntoCode / OntoCore

Contributor guide for debugging the VS Code extension, language server, React webviews, and Rust engine.

For end-user problems, see [Troubleshooting](troubleshooting.md).

## Quick reference

| Component | Where to look |
|-----------|----------------|
| Language server | **View → Output → OntoCore Language Server** |
| Extension host | **View → Output → Extension Host** (filter `OntoCode`) |
| Rust backtraces | `RUST_BACKTRACE=1` when running `ontocore-lsp` or tests |
| Custom LSP binary | `ontocode.lspPath` (trusted workspaces) or `ONTOCORE_LSP_BIN` for tests |

## Language server (`ontocore-lsp`)

### Run from terminal (stdio)

```bash
cargo build -p ontocore-lsp --bins
./target/debug/ontocore-lsp
```

The server speaks JSON-RPC over stdin/stdout. For interactive debugging, attach a debugger to the process or add `eprintln!` in `crates/ontocore-lsp/src/`.

### Point VS Code at a debug binary

```bash
cargo build -p ontocore-lsp --bins
```

Set **OntoCode: Lsp Path** (`ontocode.lspPath`) to the absolute path of `target/debug/ontocore-lsp`. **Trusted workspaces only** — Restricted Mode ignores custom paths and uses the bundled server.

Extension tests use:

```bash
export ONTOCORE_LSP_BIN="$(pwd)/target/debug/ontocore-lsp"
cd extension && npm test
```

### Common LSP issues

| Symptom | What to check |
|---------|----------------|
| `NOT_INDEXED` | Run `ontocore/indexWorkspace` or open a trusted workspace folder |
| Empty catalog after patch | `APPLIED_NOT_INDEXED` — reindex failed after write; check Output |
| Stale diagnostics | Debounced reindex; force **Index Workspace** |

Wire format: [LSP API](lsp-api.md) · source: `crates/ontocore-lsp/src/protocol.rs`, `handlers.rs`.

## VS Code extension host

### F5 / Run Extension

1. Build LSP: `cargo build -p ontocore-lsp --bins`
2. Open the `extension/` folder in VS Code
3. **Run → Start Debugging** (F5) — launches **Run Extension** host
4. In the Extension Development Host window, open a folder with `.ttl` files and trust it

Optional: set `ontocode.lspPath` in the dev host to your debug `ontocore-lsp` binary.

### Extension unit tests

```bash
cargo build -p ontocore-lsp --bins
cd extension
export ONTOCORE_LSP_BIN="$(pwd)/../target/debug/ontocore-lsp"
npm ci && npm test
```

### VS Code E2E matrix (local)

Mirrors [.github/workflows/extension-vscode-e2e.yml](https://github.com/eddiethedean/ontocode/blob/main/.github/workflows/extension-vscode-e2e.yml):

```bash
cargo build -p ontocore-lsp --bins
cd extension
npm ci && npm run compile
npm run test:vscode
```

On Linux headless CI, the workflow uses `xvfb-run` around the test command.

## React webviews (`extension/webview-ui/`)

Webviews load `webview-ui/dist` with `?panel=` routing (`inspector`, `queryWorkbench`, `semanticDiff`, `imports`, …).

### Build and test

```bash
cd extension/webview-ui
npm ci && npm test
cd ..
npm run compile   # builds webview-ui then extension host
```

!!! warning "`npm run watch` skips Vite"
    From `extension/`, `npm run watch` rebuilds the host bundle only. After React/panel changes, run `npm run build:webview` (or full `npm run compile`) before F5.

### Message protocol

Extension host ↔ React messages: [Webview protocol](webview-protocol.md) · types in `extension/src/webviews/messages.ts`.

If a panel shows the Smoke fallback, check `?panel=` is on `window.location.search` before React boots (see v0.11.3 panel routing fix in [changelog](changelog.md)).

## Rust workspace

```bash
cargo test --workspace
RUST_BACKTRACE=1 cargo test -p ontocode --test lsp_smoke
```

### Minimal test without ROBOT

Java and ROBOT are **optional** — needed only for manual `ontocore robot` development:

```bash
cargo test --workspace
```

If `ontocore-robot` tests fail locally without Java/ROBOT, run crate tests excluding robot or install ROBOT per [ROBOT interop guide](guides/robot-interop.md).

### Golden / fixture snapshots

```bash
ONTOINDEX_UPDATE_GOLDEN=1 cargo test golden_classes
ONTOINDEX_UPDATE_FIXTURE_SNAPSHOT=1 cargo test -p ontocode --test fixture_snapshot
```

## Documentation and release debugging

```bash
./scripts/check-doc-versions.sh
pip install -r docs/requirements.txt && ./scripts/build-docs.sh
```

## Related

- [Contributing](contributing.md)
- [Internals](internals.md)
- [Implementation architecture](design/ARCHITECTURE.md)
- [LSP API](lsp-api.md)
