# Architecture tour (~15 minutes)

One-page map for new contributors: how OntoCode, OntoCore, and the webviews connect.

> **Audience:** Contributors and integrators. Evaluators should start with [Architecture](../architecture.md) and [What ships today](../SHIPPED.md).

## Stack at a glance

```text
VS Code (extension/) ──stdio──► ontocore-lsp ──► ontocore (facade)
                                      │              │
                                      │              ├── ontocore-catalog (index)
                                      │              ├── ontocore-query (SQL/SPARQL)
                                      │              ├── ontocore-owl / ontocore-obo (write-back)
                                      │              ├── ontocore-reasoner → Ontologos
                                      │              └── ontocore-plugin (host)
                                      │
React webviews (extension/webview-ui/) ◄── postMessage ── extension host
```

## Repository layout

| Path | Role |
|------|------|
| [`crates/ontocore/`](https://github.com/eddiethedean/ontocode/tree/main/crates/ontocore) | Public Rust façade — `Workspace` API |
| [`crates/ontocore-lsp/`](https://github.com/eddiethedean/ontocode/tree/main/crates/ontocore-lsp) | Language server — LSP + custom `ontocore/*` JSON-RPC |
| [`crates/ontocore-cli/`](https://github.com/eddiethedean/ontocode/tree/main/crates/ontocore-cli) | `ontocore` CLI binary |
| [`extension/`](https://github.com/eddiethedean/ontocode/tree/main/extension) | VS Code extension — trees, commands, LSP client, webview host |
| [`extension/webview-ui/`](https://github.com/eddiethedean/ontocode/tree/main/extension/webview-ui) | React panels (Inspector, graphs, Query Workbench, …) |
| [`fixtures/`](https://github.com/eddiethedean/ontocode/tree/main/fixtures) | Sample ontologies for tests and tutorials |
| [`docs/`](../index.md) | Read the Docs source (this site) |

Deep crate layout: [design/ARCHITECTURE.md](../design/ARCHITECTURE.md) (contributors only).

## Request flow (edit in VS Code)

1. User edits in **Entity Inspector** (React webview).
2. Webview sends a patch/refactor message to the extension host ([webview protocol](../webview-protocol.md)).
3. Extension calls LSP `ontocore/applyAxiomPatch` or refactor methods ([LSP API](../lsp-api.md)).
4. `ontocore-lsp` applies via `ontocore-edit` transactions and format adapters (`ontocore-owl`, `ontocore-obo`).
5. Updated file text returns to VS Code; catalog re-indexes incrementally.

## Where to change what

| You want to… | Start here |
|--------------|------------|
| Fix Turtle/OBO write-back | `crates/ontocore-owl/`, `crates/ontocore-obo/` |
| Add LSP method | `crates/ontocore-lsp/src/handlers.rs`, [LSP API](../lsp-api.md) |
| Add CLI command | `crates/ontocore-cli/src/main.rs`, [CLI reference](../cli-reference.md) |
| Add VS Code command | `extension/src/commands/` |
| Add React panel | `extension/webview-ui/src/panels/` |
| Add diagnostic rule | `crates/ontocore-diagnostics/` |

## Next steps

- [Internals](../internals.md) — role-based contributor paths
- [Testing matrix](testing-matrix.md) — which tests to run by change type
- [Extension development](extension-development.md)
- [LSP hello world](lsp-hello-world.md)
