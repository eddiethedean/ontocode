# ontocore-plugin

> **Experimental — v0.14 foundation.** Manifest parsing and discovery only. **Not a stable plugin API** — see [plugin model](../../docs/ontocore/plugin-model.md) for the v0.14+ host roadmap.

Plugin host foundation for OntoCore:

- TOML plugin manifest parsing (see [PLUGIN_SPEC](../../docs/design/PLUGIN_SPEC.md))
- Discovery of manifests under `.ontocore/plugins/*.toml`

```toml
ontocore-plugin = "0.17"
```

Enable via the `ontocore` façade feature:

```toml
ontocore = { version = "0.17", features = ["plugins"] }
```

**Current version: 0.17.0**
