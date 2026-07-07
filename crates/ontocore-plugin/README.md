# ontocore-plugin

Plugin host foundation for OntoCore — manifest parsing and workspace discovery (v0.14 MVP).

Full runtime hosting ships in v0.14; this crate provides:

- TOML plugin manifest parsing (see [PLUGIN_SPEC](../../docs/design/PLUGIN_SPEC.md))
- Discovery of manifests under `.ontocore/plugins/*.toml`

```toml
ontocore-plugin = "0.13"
```

**Current version: 0.13.0**
