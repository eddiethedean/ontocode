# Plugin authoring (v0.16+)

> **Implement against this page today:** workspace TOML manifests + subprocess JSON contract.
> The React/TypeScript [Plugin API spec](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/PLUGIN_API_SPEC.md) describes a **future** OntoUI host contract — not the shipped VS Code integration.

OntoCore’s plugin host (MVP since v0.14) supports **permissions**, **versioned API** (`api_version = "1"`), and **UI contributions**. Since **v0.16**, the OntoCode extension hosts `preferences_pages` and `context_actions` (not schema-only).

## UI contribution matrix (v0.16+)

| Manifest field | VS Code | CLI / LSP | Status |
|----------------|---------|-----------|--------|
| `[[ui.commands]]` | Command palette (**Plugins: Run Command…**) | `ontocore/runPlugin` | **Shipped** |
| `[[ui.views]]` | Dockable panel (**Plugins: Open View…**) | `ontocore/runPlugin` with `action: "ui_view"` | **Shipped** |
| `[[ui.inspector_cards]]` | Entity Inspector cards | Via validate/index | **Shipped** (v0.14) |
| `[[ui.preferences_pages]]` | **Plugins: Open Preferences…** | Via `listPlugins` metadata | **Shipped** (v0.16) |
| `[[ui.context_actions]]` | Entity / ontology context menus | Via `listPlugins` metadata | **Shipped** (v0.16) |

## Permissions

Declare in `[plugin]`:

```toml
permissions = ["workspace.read", "workspace.write", "external_process"]
```

| Permission | Required for |
|------------|--------------|
| `workspace.read` | Validate, export, list plugins, read workspace files |
| `workspace.write` | Plugins that write into the workspace |
| `external_process` | Subprocess `entry` binaries |

Manifests that omit `permissions` receive backward-compatible defaults (`workspace.read`, and `external_process` when `entry` is set). **New plugins should always declare permissions explicitly.**

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
- OntoCode command **Plugins: Open Preferences…** — open plugin preference pages (v0.16+)
- OntoCode command **Run Workflow (owlmake)** — workflow scaffold output channel
- Context menus — plugin `context_actions` on entities/ontologies (v0.16+)

## Debugging failures

| Symptom | Check |
|---------|-------|
| Plugin not listed | Manifest path `.ontocore/plugins/*.toml`; `api_version = "1"`; `ontocore plugins list` |
| `INDEX_FAILED` on `runPlugin` | Subprocess `entry` on `PATH`; permissions include `external_process`; stdout is valid JSON |
| No diagnostics in Problems | Plugin returned empty `diagnostics` or validate action was not triggered |
| Permission denied | Declared `permissions` too narrow for the action |

## Security: `external_process`

Plugins with `entry` and `external_process` run **arbitrary binaries** chosen by the workspace manifest. Treat trust like CI scripts:

- Prefer allowlisting plugin directories in enterprise deployments.
- Review `entry` paths before opening untrusted ontology repos.
- Restricted Mode / Workspace Trust still apply to custom LSP/ROBOT paths; plugins that spawn processes need an explicit permission declaration.

Threat-model overview: [Security](../security.md).

## Stability

Plugin APIs are **pre-1.0** and may change until OntoCore 1.0.

**Canonical author guide: this page only.** Historical trait sketches live on GitHub as non-product background ([PLUGIN_SPEC.md](https://github.com/eddiethedean/ontocode/blob/main/docs/design/PLUGIN_SPEC.md)) and are **excluded from public docs search**. The React/TypeScript [Plugin API spec](https://github.com/eddiethedean/ontocode/blob/main/docs/ui/PLUGIN_API_SPEC.md) is a **future** OntoUI target — not the shipped VS Code/CLI contract.
