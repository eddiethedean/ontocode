# Ecosystem Architecture

> **Canonical copy:** [docs/architecture.md](docs/architecture.md) (also on [Read the Docs](https://ontocode-vs.readthedocs.io/en/latest/architecture/)).
>
> Edit **`docs/architecture.md`** for content changes. This root file is a GitHub landing pointer so links from the repository root stay valid.

**Latest tagged: v0.24.0 — v0.24 ships today. OntoCode (VS Code) + OntoCore (CLI/LSP/library).

## Quick map

```text
OntoCode (VS Code) ──ontocore-lsp──► OntoCore (Rust engine)
                                      ├── Ontologos (reasoning)
                                      └── Oxigraph / Horned-OWL
```

## Related

| Document | When |
|----------|------|
| [docs/architecture.md](docs/architecture.md) | Full ecosystem overview |
| [docs/design/ARCHITECTURE.md](docs/design/ARCHITECTURE.md) | Contributor crate layout |
| [docs/ontocore/architecture.md](docs/ontocore/architecture.md) | Short OntoCore stack |
| [Platform overview (GitHub)](docs/platform/OVERVIEW.md) | OntoUI / WorkspaceStore implementers |
