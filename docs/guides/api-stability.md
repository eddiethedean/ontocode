# API stability (pre-1.0)

OntoCode and OntoCore are **pre-1.0**. Published crates use **0.12.x** semver, but minor releases may add or change APIs until v1.0.0. This page tiers surfaces by stability so integrators can assess risk.

**Canonical capabilities:** [What ships today](../SHIPPED.md)

## Stability tiers

| Tier | Surface | Stability | Notes |
|------|---------|-----------|-------|
| **A — Stable enough for CI** | `ontocore validate`, `query`, `sparql`, `classify`, `diff`, `docs`, `patch`, `robot` CLI | High for **commands and exit codes** | Pin with `cargo install ontocore-cli --locked --version 0.12.0`. Exit codes documented in [workspace limits](../workspace-limits.md). |
| **B — Documented, may evolve** | LSP custom methods (`ontocore/*`) | Medium | Wire format in [LSP API](../lsp-api.md) and [JSON Schema](../lsp-protocol.schema.json). Minor releases may add fields or methods. |
| **C — Library APIs** | `ontocore` and `ontocore-*` Rust crates | Medium-low | Public types used by CLI/LSP are more stable than internal modules. Pin exact versions in `Cargo.toml`. |
| **D — Experimental / not shipped** | Plugin host, SHACL, MCP, Python/TS SDKs | N/A | Design specs only — see [Plugin model](../ontocore/plugin-model.md). |

## What we commit to before v1.0

- **Document** breaking changes in [migration guides](../migration/README.md) and [changelog](../changelog.md).
- **Keep CLI command names** stable where possible (`validate`, `query`, `classify`, etc.).
- **Publish** LSP JSON Schema alongside releases when wire format changes.

## What may change between minors

- Rust public API on `ontocore-*` crates (prefer pinning `0.12` in Cargo.toml).
- LSP request/response fields (clients should tolerate unknown fields).
- SQL virtual table columns (check [sql-reference](../sql-reference.md) per release).
- Webview `postMessage` payloads (extension + webview-ui ship together in the VSIX).

## Recommended pinning

**CI / ops:**

```bash
cargo install ontocore-cli --locked --version 0.12.0
```

**Rust embedding:**

```toml
ontocore = "0.12"
ontocore-core = "0.12"
```

**VS Code:** install OntoCode **0.12.0** from Marketplace, Open VSX, or a release VSIX — the bundled `ontocore-lsp` matches the extension version.

## Enterprise evaluation

- **Production readiness:** [Production readiness](production-readiness.md)
- **Security:** [Security](../security.md)
- **LGPL (horned-owl):** [LGPL compliance](lgpl-compliance.md)
- **Support policy:** [Governance](governance.md)

## After v1.0

v1.0.0 will semver **library and LSP wire format** under documented stability rules. Until then, treat this page and [SHIPPED.md](../SHIPPED.md) as the integration contract.
