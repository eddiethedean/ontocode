# ontocore-plugin

> **Experimental — v0.14+ host foundation.** Manifest parsing and discovery. **Not a stable plugin API** — implement against the TOML/subprocess model in **[Plugin authoring](../../docs/guides/plugins.md)**.

Plugin host foundation for OntoCore:

- TOML plugin manifest parsing (schema documented in the author guide)
- Discovery of manifests under `.ontocore/plugins/*.toml`

```toml
ontocore-plugin = "0.20"
```

Enable via the `ontocore` façade feature:

```toml
ontocore = { version = "0.20", features = ["plugins"] }
```

Historical design notes only (do not implement from): [PLUGIN_SPEC.md](../../docs/design/PLUGIN_SPEC.md).

**Current version: 0.20.0**
