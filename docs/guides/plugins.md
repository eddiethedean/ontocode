# Plugin authoring (v0.15)

OntoCore v0.14 ships a **plugin host MVP** with manifest discovery, in-process reference plugins, and subprocess workflow plugins.

## Quick start

1. Create `.ontocore/plugins/*.toml` in your ontology workspace.
2. Run `ontocore plugins list` or `ontocore validate` to execute validator plugins.
3. Install the OntoCode extension and index the workspace — plugin diagnostics appear in the Problems panel.

See [examples/plugin-workspace](https://github.com/eddiethedean/ontocode/tree/main/examples/plugin-workspace) for a working fixture.

## Manifest schema

```toml
[plugin]
name = "my-validator"
version = "0.1.0"
kind = "validator"          # validator | exporter | workflow | …
id = "org.example.my-validator"
api_version = "1"
entry = "my-plugin-cli"     # optional subprocess binary
permissions = ["workspace.read", "external_process"]

[capabilities]
validate = true
diagnostics = true
export = false

[config]
require_label = true
shapes_dir = "shapes"

[[ui.commands]]
id = "org.example.check"
title = "Run my validator"

[[ui.views]]
id = "org.example.view"
title = "My dockable view"

[[ui.preferences_pages]]
id = "org.example.prefs"
title = "My plugin"
category = "Validation"

[[ui.context_actions]]
id = "org.example.ctx"
title = "Validate this class"
scope = "entity"
applies_to = ["class"]
command = "org.example.check"

[[ui.inspector_cards]]
id = "my-card"
title = "My validator"
applies_to = ["class"]
```

## Subprocess contract

Plugins with an `entry` binary are invoked as:

```text
<entry> <action> --workspace <path> [--step <name>] [--view <id>]
```

Supported actions: `validate`, `export`, `workflow`, `ui-view`.

Stdout must be JSON:

```json
{
  "diagnostics": [
    {
      "code": "missing_label",
      "severity": "warning",
      "message": "Entity has no rdfs:label",
      "file": "demo.ttl",
      "entity_iri": "http://example.org/UnlabeledClass"
    }
  ],
  "output_paths": [],
  "logs": "optional log text",
  "view_html": "<h1>optional HTML for ui-view</h1>"
}
```

Reference subprocess CLIs ship with OntoCore:

- `ontocore-plugin-naming`
- `ontocore-plugin-markdown-export`
- `ontocore-plugin-shacl`

## Reference plugins

| Id | Kind | Purpose |
|----|------|---------|
| `ontocode.naming-validator` | validator | Require `rdfs:label` on entities |
| `ontocode.markdown-export` | exporter | Markdown docs via `ontocore-docs` |
| `ontocode.shacl-validator` | validator | SHACL shapes directory check (rudof adapter planned) |
| `owlmake` (external) | workflow | Invoke [owlmake](https://github.com/INCATools/owlmake) from manifest `entry` |

## CLI

```bash
ontocore plugins list /path/to/workspace
ontocore plugins run ontocode.naming-validator --action validate /path/to/workspace
ontocore validate /path/to/workspace    # merges plugin diagnostics
ontocore docs /path/to/workspace -o out --plugin ontocode.markdown-export
ontocore workflow --plugin owlmake --step qc /path/to/workspace
```

## LSP / OntoCode

- `ontocore/listPlugins` — discovered plugins + UI metadata
- `ontocore/runPlugin` — run validate/export/workflow actions (and `ui_view` for views)
- OntoCode command **Plugins: Run Command…** — browse plugin commands
- OntoCode command **Plugins: Open View…** — open plugin-contributed views
- OntoCode command **Run Workflow (owlmake)** — workflow scaffold output channel

## Stability

Plugin APIs are **pre-1.0** and may change until OntoCore 1.0. See [PLUGIN_SPEC.md](../design/PLUGIN_SPEC.md).
