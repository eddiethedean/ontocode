# ontocore-lsp

> Part of **OntoCore** (semantic workspace engine).

Language server for OntoCode and LSP integrators — stdio transport, custom `ontocore/*` methods.

**v0.10:** multi-root workspace indexing, incremental reindex, semantic diff (`ontocore/semanticDiff`), optional disk cache via `ontocore/indexWorkspace`.

**v0.11:** Turtle `textDocument/completion` (prefix, QName, IRI); diagnostic quick fixes via `textDocument/codeAction`; import patch ops (`add_import`, `remove_import`).

## Install

```bash
cargo install ontocore-lsp --locked
```

## Documentation

- [VS Code extension docs](https://ontocode-vs.readthedocs.io/en/latest/guides/vscode-extension/)
- [Rust & CLI docs](https://ontocode-vs.readthedocs.io/en/latest/guides/rust-crates/)
- [LSP API](https://ontocode-vs.readthedocs.io/en/latest/lsp-api/)
- [Install VS Code](https://ontocode-vs.readthedocs.io/en/latest/vscode-install/)
- [docs.rs](https://docs.rs/ontocore-lsp)

**Current version: 0.26.2**

## License

MIT OR Apache-2.0
