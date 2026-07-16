# ontocore-plugin

> **Plugin SDK 1.0** — stable TOML manifest + subprocess JSON host for OntoCore / OntoCode.
> Author against **[Plugin authoring](../../docs/guides/plugins.md)** and the **[SDK 1.0 compatibility policy](../../docs/guides/plugin-policy.md)**.

Plugin host for OntoCore:

- TOML plugin manifest parsing (`api_version = "1"`)
- Discovery under `.ontocore/plugins/*.toml`
- Lifecycle: discover → validate → register → activate / disable (dependency-aware)
- Provider actions: validate, export, workflow, UI view, reasoner, query, refactor, graph

```toml
ontocore-plugin = "0.26"
```

Enable via the `ontocore` façade feature:

```toml
ontocore = { version = "0.26", features = ["plugins"] }
```

Reserved future kinds (`editor`, `language_service`, `tool_window`) and AI providers are documented but not hosted until a later release.

Historical design notes only (do not implement from): [PLUGIN_SPEC.md](../../docs/design/PLUGIN_SPEC.md).

**Current crate version: 0.26.0 (SDK 1.0 surface)**
