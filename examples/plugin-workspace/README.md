# Plugin workspace example

Minimal ontology folder with sample `.ontocore/plugins/` manifests for the OntoCore plugin host (v0.14+).

Full authoring guide: [Plugin authoring](https://ontocode-vs.readthedocs.io/en/latest/guides/plugins/).

## Layout

```text
plugin-workspace/
  demo.ttl
  demo_ui_view.sh
  .ontocore/plugins/
    naming-validator.toml
    demo-ui-view.toml
    owlmake.toml
```

## Try it (git clone)

```bash
# From repo root — list discovered plugins
cargo run -- plugins list --path examples/plugin-workspace

# Validate (runs validator plugins)
cargo run -- validate examples/plugin-workspace
```

With an installed CLI:

```bash
ontocore plugins list --path examples/plugin-workspace
ontocore validate examples/plugin-workspace
```

## VS Code

1. **File → Open Folder…** → `examples/plugin-workspace`
2. Run **OntoCode: Index Workspace**
3. Use **Plugins: Open View…** / **Plugins: Run Command…** for UI contributions from `demo-ui-view.toml`

## Notes

- Plugin manifests are **workspace-local** (not a marketplace).
- Subprocess plugins need an executable `entry` (see `demo_ui_view.sh`).
- Declare `permissions` explicitly on new plugins — see the plugins guide.
