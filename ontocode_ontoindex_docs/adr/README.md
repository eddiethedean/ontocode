# Architecture Decision Records

Canonical ADRs live in this directory. The former `adrs/` folder was merged here; do not add duplicate ADR trees.

## Index

| ADR | Title | Status |
|-----|-------|--------|
| [0001](0001-use-rust-for-ontoindex.md) | Use Rust for OntoIndex | Accepted |
| [0002](0002-use-horned-owl.md) | Use Horned-OWL for OWL modeling | **Superseded** (v0.2 uses Oxigraph extraction) |
| [0003](0003-use-oxigraph.md) | Use Oxigraph | Accepted |
| [0004](0004-use-datafusion-for-sql.md) | Use DataFusion for SQL | **Superseded** (v0.2 uses sqlparser virtual tables) |
| [0005](0005-local-first-by-default.md) | Local-first by default | Accepted |
| [0006](0006-patch-based-write-back.md) | Patch-based write-back | Accepted (planned v0.4) |
| [0007](0007-language-server-boundary.md) | Language server boundary | Accepted |
| [0008](0008-reasoner-adapters-not-built-in-reasoner.md) | Reasoner adapters | Accepted (planned) |
| [0009](0009-semantic-diff-as-core-feature.md) | Semantic diff as core feature | Accepted (planned) |
| [0010](0010-ai-features-opt-in.md) | AI features opt-in | Accepted (planned) |
| [0011](0011-use-sqlparser-for-sql.md) | Use sqlparser for SQL virtual tables | Accepted |

## Current stack (v0.2)

- **Parsing / triple store:** Oxigraph ([ADR-0003](0003-use-oxigraph.md))
- **SQL-like queries:** sqlparser + virtual tables in `ontoindex-query` ([ADR-0011](0011-use-sqlparser-for-sql.md))
- **Editor integration:** LSP stdio ([ADR-0007](0007-language-server-boundary.md))

Horned-OWL and DataFusion remain documented for historical context and may be revisited for OWL-native editing or heavier analytics later.
