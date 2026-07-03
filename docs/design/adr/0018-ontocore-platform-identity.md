# ADR-0018: OntoCore platform identity

## Status

Accepted (v0.9.0)

## Context

The Rust semantic workspace engine was introduced as **OntoIndex** (`ontoindex` CLI, `ontoindex-*` crates). As the platform matures, it needs a distinct identity separate from the VS Code product (**OntoCode**) and the reasoning stack (**OntoLogos**).

Splitting into a separate repository prematurely would create churn while APIs are still evolving. The monorepo already has a clean architectural split between `crates/` (engine) and `extension/` (IDE).

## Decision

1. **OntoCore** is the platform brand for the Rust semantic workspace engine.
2. **OntoCode** is the VS Code IDE powered by OntoCore.
3. Implementation crates retain **`ontoindex-*` names** until the public API stabilizes at v1.0.
4. Add a public façade crate **`ontocore`** that re-exports implementation crates and provides `Workspace` as the ergonomic entry point.
5. Keep compatibility aliases in v0.9:
   - CLI binary: `ontoindex`
   - LSP binary: `ontoindex-lsp`
   - LSP methods: `ontoindex/*`
6. Staged rename plan:
   - v0.10: `ontocore` CLI alias
   - v1.0: `ontocore` CLI primary; optional `ontocore-*` crate renames
7. Repository remains a monorepo until API stability and release process justify a split.

## Consequences

### Positive

- Clear branding for users, contributors, and future bindings (Python, TypeScript, MCP).
- `ontocore` crate gives embedders a single dependency without immediate crate renames.
- OntoCode marketplace story stays focused on the IDE.

### Negative / mitigations

- Two names (`OntoCore` vs `ontoindex-*`) until v1.0 — documented in [ontocore/index.md](../../ontocore/index.md).
- Documentation must be updated gradually; high-traffic pages lead with OntoCore.

## References

- [OntoCore roadmap](../../ontocore/roadmap.md)
- [ARCHITECTURE.md](../ARCHITECTURE.md)
- [ROADMAP.md](../ROADMAP.md) v0.9 section
