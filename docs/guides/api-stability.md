# API stability (pre-1.0)

OntoCode and OntoCore are **pre-1.0**. Published crates use **0.19.x** on crates.io (latest tagged); the workspace on `main` may be **0.20.x** unreleased. Minor releases may add or change APIs until v1.0.0.

**Canonical capabilities:** [What ships today](../SHIPPED.md)

## v0.17 API freeze scope (path to 1.0)

The following modules are **documented and intended to stabilize** toward 1.0:

| Module / surface | Crate | Notes |
|------------------|-------|-------|
| `Workspace`, catalog index | `ontocore` | Primary embedding entry |
| Core model types | `ontocore-core` | `Entity`, `Diagnostic`, IRI helpers |
| SQL / SPARQL query | `ontocore-query` | Virtual tables; new tables may be added pre-1.0 |
| Diagnostics | `ontocore-diagnostics` | Rule codes + `DiagnosticConfig` |
| Semantic diff | `ontocore-diff` | `DiffResult`, `format_diff_*` |
| Docs export | `ontocore-docs` | `export_workspace`, hierarchy/property renderers |
| OWL / OBO patch | `ontocore-owl`, `ontocore-obo` | Patch op JSON shapes |
| Plugin host (MVP) | `ontocore-plugin` | Manifest schema, `PluginHost` — **experimental** until 1.0 |

**May still change pre-1.0:** internal indexer modules, LSP field additions, webview `postMessage` types (ship with extension), SQL column additions, plugin trait signatures.

**Frozen at 1.0 (target):** CLI command names, exit codes, stable Rust types above, documented LSP `ontocore/*` methods, semver-stable plugin API.

## Stability tiers

| Tier | Surface | Stability | Notes |
|------|---------|-----------|-------|
| **A — Stable enough for CI** | `ontocore validate`, `query`, `sparql`, `classify`, `diff`, `docs`, `patch`, `robot`, `plugins`, `workflow` CLI | High for **commands and exit codes** | Pin with `cargo install ontocore-cli --locked --version 0.19.0`. Exit codes documented in [workspace limits](../workspace-limits.md). |
| **B — Documented, may evolve** | LSP custom methods (`ontocore/*`) | Medium | Wire format in [LSP API](../lsp-api.md) and [JSON Schema](../lsp-protocol.schema.json). Minor releases may add fields or methods. |
| **C — Library APIs** | `ontocore` and `ontocore-*` Rust crates | Medium-low | Public types used by CLI/LSP are more stable than internal modules. Pin exact versions in `Cargo.toml`. |
| **D — Experimental** | Plugin author traits, SHACL full validation, MCP, Python/TS SDKs | Low | Plugin host **shipped v0.14–v0.17** — see [Plugin authoring](plugins.md). Stable ecosystem API is **v1.0**. |

## What we commit to before v1.0

- **Document** breaking changes in [migration guides](../migration/README.md) and [changelog](../changelog.md).
- **Keep CLI command names** stable where possible (`validate`, `query`, `classify`, etc.).
- **Publish** LSP JSON Schema alongside releases when wire format changes.

## What may change between minors

- Rust public API on `ontocore-*` crates (prefer pinning `0.19` in Cargo.toml).
- LSP request/response fields (clients should tolerate unknown fields).
- SQL virtual table columns (check [sql-reference](../sql-reference.md) per release).
- Webview `postMessage` payloads (extension + webview-ui ship together in the VSIX).
- Plugin manifest schema and subprocess contract (documented in [Plugin authoring](plugins.md)).

## Recommended pinning

**CI / ops:**

```bash
cargo install ontocore-cli --locked --version 0.19.0
```

**Rust embedding:**

```toml
ontocore = "0.19"
ontocore-core = "0.19"
```

**VS Code:** install OntoCode **0.19.0** (latest tagged) from Marketplace, Open VSX, or a release VSIX — the bundled `ontocore-lsp` matches the extension version.

## Enterprise evaluation

- **Production readiness:** [Production readiness](production-readiness.md)
- **Security:** [Security](../security.md)
- **LGPL (horned-owl):** [LGPL compliance](lgpl-compliance.md)
