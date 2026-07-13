# OntoCore LSP API (v0.19)

> **Status:** Documents behavior in **OntoCore v0.19.0**. Pre-1.0 APIs may change.
> Canonical feature list: [What ships today](SHIPPED.md).

This document describes **what ships today** in `ontocore-lsp`. For the **v1.0 target** (extended plugin methods), see [LSP_SPEC.md](design/LSP_SPEC.md).

## Start with the schema (recommended)

If you are integrating OntoCore outside VS Code (custom editor, scripts, automation), treat the JSON schema as the **canonical, machine-readable contract** for this release:

- **LSP JSON Schema:** [`lsp-protocol.schema.json`](lsp-protocol.schema.json) (ships with product **v0.19.0**)

### Schema vs product version

The schema file is the wire contract for the **current product release**. Until v1.0, minor product releases may add or change fields — always pin OntoCore and consume the schema from the **same tagged release**. Historical labels such as “v0.17 schema” in older docs referred to the product release that last expanded the contract, not a separate schema versioning scheme.

### Versioning and pinning (pre-1.0)

Until v1.0, minor releases may change request/response fields.
For stable integrations:

- **Pin OntoCore to **0.19.0**`) in your tooling.
- Prefer consuming `lsp-protocol.schema.json` from the same tagged release you deploy.

## Wire format

LSP JSON uses **snake_case** for enums serialized from Rust (`EntityKind`, `ParseStatus`, `OntologyFormat`), e.g. `"kind": "class"`, `"parse_status": "ok"`. SQL virtual tables use the same snake_case strings via `as_str()` on core enums (e.g. `ParseStatus::as_str()` → `"ok"`, `EntityKind::as_str()` → `"class"`, `axiom_kind` → `"sub_class_of"`).

**Reference links (implementation):**

- Types: [`protocol.rs` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/crates/ontocore-lsp/src/protocol.rs)
- JSON Schema: [`lsp-protocol.schema.json`](lsp-protocol.schema.json) — query, patch, reasoner, refactor, graph, semantic diff, schema browser, PR summary, plugin payloads, explanation alternatives.
- Handlers: [`handlers.rs` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/crates/ontocore-lsp/src/handlers.rs)
- Extension client: [`client.ts` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/extension/src/lsp/client.ts)

## Transport

- stdio (VS Code language client)

## Standard LSP capabilities

| Capability | Status |
|------------|--------|
| `textDocument/hover` | Implemented (basic entity info) |
| `textDocument/documentSymbol` | Implemented |
| `workspace/symbol` | Implemented |
| `textDocument/definition` | Implemented |
| Diagnostics | **Implemented** — server pushes `textDocument/publishDiagnostics` after each reindex |
| `textDocument/completion` | **Implemented (v0.11)** — Turtle prefix, QName, and IRI contexts |
| `textDocument/codeAction` | **Implemented (v0.11)** — diagnostic quick fixes |
| Rename | **Implemented** — `textDocument/rename` + `textDocument/prepareRename` |
| Find references | **Implemented** — `textDocument/references` |

### `textDocument/completion` (v0.11)

Advertised with trigger characters `:`, `<`, and `@`. Applies to **Turtle (`.ttl`)** files only.

| Context | When | Items |
|---------|------|-------|
| Prefix declaration | After `@prefix` / `@base` | Namespace IRIs from the indexed catalog |
| QName prefix | Before `:` in a QName | Declared `@prefix` names |
| QName local | After `prefix:` | Entity short names matching the prefix (classes, properties, individuals) |
| IRI bracket | Inside `<` … | Full IRIs from the catalog (capped at 100 items) |

Results are capped at **100 items** per request.

### `textDocument/codeAction` (v0.11)

Quick fixes are offered when a diagnostic includes encoded `QuickFix` data in `Diagnostic.data`. The client shows a lightbulb on supported codes:

| Diagnostic code | Quick fix behavior |
|-----------------|-------------------|
| `undefined_prefix` | Insert `@prefix` declaration for the missing prefix |
| `missing_label` | Apply patch to add a default label |
| `broken_import` | Remove the broken `owl:imports` line |

Other diagnostic codes (`parse_error`, `duplicate_label`, `orphan_class`, …) publish diagnostics but do not ship quick fixes in v0.11.

Code actions use kind `quickfix` and return workspace edits (insert text, remove line, or apply Turtle patch).

## Custom methods

All custom methods use the `ontocore/` prefix.

### `ontocore/indexWorkspace`

Rebuild the workspace catalog.

**Params:** `IndexWorkspaceParams`

```json
{ "workspace_uri": "file:///path/to/workspace", "disk_cache": true }
```

| Field | Type | Description |
|-------|------|-------------|
| `workspace_uri` | string? | Workspace folder URI; omitted uses initialized workspace |
| `disk_cache` | boolean | When `true`, persist parse snapshots under `.ontocore/cache/` (v0.10+) |

`workspace_uri` is optional; the server uses the initialized workspace folder when omitted. Legacy clients may send `workspaceUri` (camelCase); prefer `workspace_uri` for new integrations.

**Result:** `IndexWorkspaceResult`

| Field | Type | Description |
|-------|------|-------------|
| `stats` | `CatalogStats` | Counts after indexing (see below) |
| `indexed_at` | number | Unix timestamp (seconds) |

**`CatalogStats` fields:**

| Field | Description |
|-------|-------------|
| `ontology_count` | Indexed ontology documents |
| `class_count` | Classes |
| `object_property_count` | Object properties |
| `data_property_count` | Data properties |
| `annotation_property_count` | Annotation properties |
| `individual_count` | Named individuals |
| `axiom_count` | Extracted axioms |
| `annotation_count` | Annotation triples |
| `triple_count` | Total RDF triples (Oxigraph) |
| `error_count` | Documents with parse errors |
| `diagnostic_error_count` | Lint errors |
| `diagnostic_warning_count` | Lint warnings |
| `diagnostic_info_count` | Lint info |

```json
{
  "stats": {
    "ontology_count": 1,
    "class_count": 2,
    "object_property_count": 1,
    "data_property_count": 0,
    "annotation_property_count": 0,
    "individual_count": 1,
    "axiom_count": 2,
    "annotation_count": 4,
    "triple_count": 12,
    "error_count": 0,
    "diagnostic_error_count": 0,
    "diagnostic_warning_count": 0,
    "diagnostic_info_count": 0
  },
  "indexed_at": 1718000000
}
```

### `ontocore/getCatalogSnapshot`

Return documents, entities, and class hierarchy for the explorer UI.

**Params:** `null`

**Result:** `CatalogSnapshot`

| Field | Type | Description |
|-------|------|-------------|
| `documents` | `OntologyDocument[]` | Indexed files (`id`, `path`, `format`, `parse_status`, …) |
| `entities` | `Entity[]` | All extracted entities |
| `hierarchy` | `ClassHierarchy` | `edges`, `parents`, `children` maps (asserted) |
| `stats` | `CatalogStats`? | **(v0.17+)** Same counts as `indexWorkspace` (when catalog is indexed) |
| `active_ontology_id` | string? | **(v0.17+)** Active ontology id when set via `setActiveOntology` |
| `diagnostics` | `DiagnosticSummary[]` | Lint summaries for explorer |
| `reasoner` | `ReasonerSnapshot` (optional) | Last reasoner run — inferred hierarchy, unsatisfiable classes |

**`Entity` fields:** `iri`, `short_name`, `kind`, `ontology_id`, `labels[]`, `comments[]`, `deprecated`, optional `source_location`.

**Errors:** `NOT_INDEXED` if the workspace has not been indexed.

### `ontocore/getEntity`

Return detailed entity information for the inspector.

**Params:** `GetEntityParams`

```json
{ "iri": "http://example.org/people#Person" }
```

**Result:** `GetEntityResult` with `detail` (`EntityDetail`):

| Field | Description |
|-------|-------------|
| `entity` | Core `Entity` record |
| `parents` | Parent class IRIs |
| `children` | Child class IRIs |
| `axioms` | `EntityAxiomSummary[]` — structured axiom rows for inspector and Manchester editor |
| `source` | Optional `{ path, line, column }` |
| `editable` | `true` when the entity's declaring file supports patch write-back (`.ttl` or `.obo` per v0.12); see [Patch reference](patch-reference.md) |
| `document_path` | Filesystem path to declaring file |

**`EntityAxiomSummary` fields:**

| Field | Description |
|-------|-------------|
| `kind` | `sub_class_of` or `equivalent_class` |
| `display` | Human-readable label (e.g. `SubClassOf ex:Person` or Manchester string) |
| `manchester` | Manchester expression when the axiom is complex (optional) |
| `parent_iri` | Named parent class IRI for simple `SubClassOf` (optional) |
| `editable` | Whether the axiom can be edited via patch write-back |

**Errors:** `NOT_INDEXED`, `ENTITY_NOT_FOUND`

### `ontocore/query`

Run a SQL-like query against the indexed workspace catalog.

**Params:** `QueryParams`

```json
{ "sql": "SELECT short_name, labels FROM classes" }
```

**Result:** `TabularQueryResult`

| Field | Description |
|-------|-------------|
| `columns` | Column names |
| `rows` | Array of row objects (`column` → string value) |
| `truncated` | `true` when the server row cap was hit (optional) |

**Errors:** `NOT_INDEXED`, `QUERY_FAILED`

### `ontocore/sparql`

Run SPARQL against the indexed catalog store.

**Params:** `SparqlParams`

```json
{ "query": "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10" }
```

**Result:** `TabularQueryResult` (same shape as `ontocore/query`).

**Errors:** `NOT_INDEXED`, `QUERY_FAILED`

### `ontocore/parseManchester`

Parse and validate a Manchester class expression; return normalized text, Turtle fragment, expression tree, diagnostics, and catalog-based completion lists.

**Params:** `ParseManchesterParams`

```json
{
  "expression": "ex:hasRecord some ex:MedicalRecord",
  "axiom_kind": "sub_class_of",
  "entity_iri": "http://example.org/clinic#Patient",
  "document_uri": "file:///path/to/ontology.ttl"
}
```

**Result:** `ParseManchesterResult`

| Field | Description |
|-------|-------------|
| `normalized` | Canonical Manchester string |
| `turtle_fragment` | Turtle axiom fragment for preview |
| `tree` | JSON expression tree for UI |
| `diagnostics` | Parse errors (`PatchDiagnostic[]`) |
| `completions` | `ManchesterCompletions` — classes, object/data properties, XSD datatypes from catalog |

**Errors:** `NOT_INDEXED`, `MANCHESTER_INVALID`

### `ontocore/applyAxiomPatch`

Apply patch operations to Turtle (`.ttl`) or OBO (`.obo`) documents. See [authoring.md](authoring.md) and [OBO authoring](ontocode/obo-authoring.md).

**Buffer-first (VS Code):** Reads the open document buffer when available, applies patches in memory, updates the buffer, writes disk, then reindexes. See [errors.md](errors.md) for `APPLIED_NOT_INDEXED`.

**Params:** `ApplyAxiomPatchParams`

Legacy patch array (unchanged since v0.11):

```json
{
  "document_uri": "file:///path/to/ontology.ttl",
  "patches": [
    { "op": "add_label", "entity_iri": "http://ex#Person", "value": "Human" }
  ],
  "preview_only": false
}
```

**v0.19+ transaction envelope (optional):** same operations wrapped for `ontocore-edit` apply path. Legacy `patches` arrays remain accepted.

```json
{
  "document_uri": "file:///path/to/ontology.obo",
  "transaction": {
    "changes": [
      { "op": "add_label", "entity_iri": "http://ex#Person", "value": "Human" }
    ]
  },
  "preview_only": false
}
```

Either `patches` or `transaction.changes` may be supplied (not both required). See [migration v0.19](migration/v0.19.md).

**Result:** `ApplyAxiomPatchResult` (flattened patch result + optional entity):

| Field | Description |
|-------|-------------|
| `applied` | `true` if patches were written (false for preview-only or validation failure) |
| `preview_text` | Updated file preview when `preview_only: true` (Turtle or OBO text) |
| `diagnostics` | `PatchDiagnostic[]` on failure (`severity`, `message`) |
| `document_path` | Path to modified file |
| `entity_detail` | Updated `EntityDetail` after successful apply (LSP only) |
| `workspace_edit` | Optional LSP `WorkspaceEdit` for buffer sync (VS Code client applies this) |
| `reindex_warning` | Present when apply succeeded but reindex failed |

**`EntityAxiomSummary` kinds:** `sub_class_of`, `equivalent_class`, `disjoint_class`, `domain`, `range`, `property_chain` (property chains editable via patch ops since v0.12).

**Import ops (v0.11):** `add_import` and `remove_import` — see [patch-reference.md](patch-reference.md) and [Manage Imports](ontocode/manage-imports.md).

See [patch-reference.md](patch-reference.md) for CLI `ApplyPatchResult` examples and [errors.md](errors.md) for failure codes.

**Errors:** `PATCH_INVALID`, `UNSUPPORTED_FORMAT`, `NOT_INDEXED`, `APPLIED_NOT_INDEXED`

### `ontocore/runReasoner`

Run OWL classification via OntoLogos 1.0.0 (`el`, `rl`, `rdfs`, `dl`, `auto`).

**Params:** `RunReasonerParams`

```json
{ "profile": "el", "auto_detect": true }
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `profile` | string | `"el"` | `el`, `rl`, `rdfs`, `dl`, or `auto` |
| `auto_detect` | boolean | `true` | Run profile detection warnings via `ontologos-profile` |

**Result:** `RunReasonerResult`

| Field | Description |
|-------|-------------|
| `profile_used` | Profile that ran |
| `consistent` | `false` when unsatisfiable classes exist |
| `unsatisfiable` | Class IRIs |
| `inferred_edge_count` | Count of inferred-only subsumption edges |
| `new_inferences` | Inferred `SubclassEdge[]` not present in asserted hierarchy |
| `warnings` | Profile / construct warnings |
| `duration_ms` | Wall-clock classification time |
| `snapshot` | Full `ReasonerSnapshot` (also attached to subsequent `getCatalogSnapshot`) |

**Errors:** `NOT_INDEXED`, `REASONER_FAILED`

### `ontocore/getExplanation` (v0.15+)

Return an EL/RL/RDFS/DL explanation for an unsatisfiable class. Responses may include **multiple alternative justifications** and **staleness metadata** for cache invalidation.

**Params:** `GetExplanationParams`

```json
{ "class_iri": "http://example.org#Invalid", "profile": "el" }
```

**Result:** `GetExplanationResult`

| Field | Description |
|-------|-------------|
| `class_iri` | Requested class |
| `steps` | Primary justification — ordered `ExplanationStep[]` (`index`, `rule`, `display`, optional IRIs) |
| `text` | Rendered proof text for the primary justification |
| `alternatives` | **(v0.15+)** Additional `ExplanationResult` objects (each with `steps` and `text`) when the reasoner provides multiple justifications |
| `indexed_at` | **(v0.15+)** Unix timestamp (seconds) of the workspace index used when the explanation was generated |
| `content_hash` | **(v0.15+)** Workspace content hash at explanation time — compare on later calls to detect stale explanations after edits |

Clients should treat explanations as **stale** when `indexed_at` or `content_hash` no longer matches the current workspace snapshot. The OntoCode explanation panel shows a stale warning and offers re-run.

**Errors:** `NOT_INDEXED`, `EXPLANATION_FAILED`

### `ontocore/getGraph` (v0.7)

Returns graph nodes and edges for visualization webviews.

**Params:**

| Field | Type | Description |
|-------|------|-------------|
| `graph_kind` | string | `class`, `property`, `import`, or `neighborhood` |
| `root_iri` | string? | Required for `neighborhood` |
| `depth` | number? | BFS depth for neighborhood (default 2, max 5) |
| `include_inferred` | boolean? | Include reasoner edges when snapshot exists |
| `filters` | object? | `ontology_iri`, `hide_deprecated` |

**Graph panel (v0.15):** The React graph webview supports **asserted**, **inferred only**, and **combined** edge modes, plus grid/circle/stack layouts and node search. These map to `include_inferred` and reasoner snapshot state on the LSP request.

**Result:** `{ graph: { nodes, edges, truncated, graph_kind } }`

### `ontocore/runRobot` (v0.7)

Runs a ROBOT CLI subcommand.

**Params:** `{ subcommand, args?, robot_path? }`

**Result:** `{ exit_code, stdout, stderr }`

### `ontocore/findUsages` (v0.8)

Return structured IRI usages across the workspace catalog and Turtle text spans.

**Params:** `{ "iri": "http://example.org/people#Person" }`

**Result:** `{ "usages": UsageSummary[] }` — each usage has `iri`, `referenced_iri`, `file`, `line`, `column`, `kind`, `context` (byte ranges are not serialized on the wire).

### `ontocore/previewRefactor` (v0.8)

Build a refactor plan without writing files.

**Params:** flattened refactor request — one of:

| `kind` | Fields |
|--------|--------|
| `rename_iri` | `from_iri`, `to_iri` |
| `migrate_namespace` | `from_base`, `to_base` |
| `move_entity` | `entity_iri`, `target_file` |
| `extract_module` | `entity_iris[]`, `output_file`, `leave_stub?` |

**Result:** `RefactorPlan` — `{ changes: FileChange[], warnings: string[] }` where each change has `path`, `preview_text`, `original_text`, and `hunks`.

### `ontocore/applyRefactor` (v0.8)

Apply a previously previewed refactor plan. The server re-previews from `request`, verifies the submitted `plan` matches, validates paths against the workspace jail, writes atomically, syncs open buffers, and reindexes.

**Params:** `{ "plan": RefactorPlan, "request": RefactorRequest, "preview_only"?: boolean }`

**Result:** `{ "files_written": number, "reindex_warning"?: string }`

### `ontocore/semanticDiff` (v0.10+)

Compare semantic catalogs between git refs, directories, or indexed workspace snapshots. Alias: `ontocore/getSemanticDiff`.

**Params:** `SemanticDiffParams`

| Field | Type | Description |
|-------|------|-------------|
| `left_ref` | string? | Git left ref (default `HEAD`) or `INDEXED` / `CATALOG` for indexed catalog (legacy alias: `WORKSPACE`) |
| `right_ref` | string? | Git right ref (default `WORKTREE`) or `INDEXED` / `CATALOG` for indexed catalog (legacy alias: `WORKSPACE`) |
| `left_path` | string? | Left directory when comparing two paths on disk |
| `right_path` | string? | Right directory |
| `reasoner` | boolean? | Enrich diff with reasoner unsatisfiability changes |
| `format` | string? | `pr-summary` returns `{ "formatted": string }` PR-ready Markdown (v0.13+) |

When both `left_path` and `right_path` are set, git refs are ignored and directories are compared directly (paths must resolve within workspace roots).

**Result:** `{ "diff": DiffResult }` — axiom-level changes, entity additions/removals, breaking-change flags. With `format: "pr-summary"`, also includes `formatted` Markdown string. See [Semantic diff guide](ontocode/semantic-diff.md).

**Errors:** `INVALID_PARAMS` (bad refs, paths outside workspace, git errors), `NOT_INDEXED` (indexed-catalog ref before first index), `REASONER_FAILED` (when `reasoner: true` enrichment fails)

### `ontocore/listSqlSchema` (v0.13+)

Returns SQL virtual table metadata for the Query Workbench schema browser.

**Params:** none (uses indexed workspace catalog)

**Result:** `{ "tables": [{ "name": string, "columns": [{ "name": string, "type": string }] }] }`

Includes core tables (`classes`, `properties`, …) and Horned-OWL axiom projections. See [sql-reference.md](sql-reference.md).

**Errors:** `NOT_INDEXED`

### `textDocument/semanticTokens/full` (v0.13+)

Standard LSP semantic tokens for **Turtle** (`.ttl`) and **OBO** (`.obo`) open documents.

**Token types:** `namespace`, `iri`, `keyword`, `comment`, `string`

Requires document text in the LSP open-document buffer (unsaved buffers supported).

### `ontocore/listPlugins` (v0.14+)

Returns discovered workspace plugins from `.ontocore/plugins/*.toml` plus built-in registration metadata.

**Params:** none (uses indexed workspace root)

**Result:** `{ "plugins": PluginDescriptor[] }`

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Plugin id |
| `name` | string | Display name |
| `version` | string | Manifest version |
| `kind` | string | `validator`, `exporter`, `workflow`, … |
| `manifest_path` | string | Absolute path to manifest TOML |
| `capabilities` | object | `build`, `validate`, `release`, `diagnostics`, `export` flags |
| `permissions` | array | **(v0.15+)** Declared permission strings (`workspace.read`, `workspace.write`, `external_process`) |
| `api_version` | string? | **(v0.15+)** Manifest API version (e.g. `"1"`) |
| `ui.commands` | array | `{ id, title, scope? }` palette contributions |
| `ui.views` | array | **(v0.15+)** `{ id, title }` dockable view contributions |
| `ui.inspector_cards` | array | `{ id, title, applies_to, command? }` inspector slots |
| `in_process` | boolean | `true` for built-in reference plugins |

**Errors:** `NOT_INDEXED`, `INDEX_FAILED` (discovery/host failure)

### `ontocore/listCommands` (v0.17+)

Return stable command metadata for menus, toolbars, and enablement.

**Params:** none

**Result:** `{ "commands": CommandDescriptor[] }`

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Command id (e.g. `ontocode.newOntology`) |
| `title` | string | Display title |
| `category` | string | Menu category (`File`, `Edit`, …) |
| `enablement` | string[] | Predicates: `always`, `has_ontology`, `is_dirty`, `has_selection`, `reasoner_running`, `reasoner_idle`, `can_edit_selection` |
| `undo_label` | string? | Semantic undo label for edits |
| `dialog_id` | string? | Associated dialog schema id |

**Errors:** none (empty `commands` if the server has no registered descriptors)

### `ontocore/getWorkspaceUiState` (v0.17+)

Return enablement inputs for the command registry.

**Params:**

| Field | Type | Description |
|-------|------|-------------|
| `selection_iri` | string? | Focused entity IRI from the client |
| `dirty_document_count` | number | Client-reported dirty document count |
| `active_ontology_id` | string? | Preferred active ontology id |

**Result:** `WorkspaceUiState` (`has_ontology`, `is_dirty`, `has_selection`, `selection_editable`, `reasoner_*`, `active_ontology_id`, optional `stats`)

**Errors:** none

### `ontocore/getDialogSchema` (v0.17+)

**Params:** `{ "dialog_id": string }`

**Result:** `{ "schema": DialogSchema }` with `id`, `title`, `fields[]`, `primary_action`.

Known dialog ids include `new_ontology`, `export_ontology`, `save_as`, `prefix_manager`, `ontology_metadata`, `search`, `metrics`, `delete_entity`, `reasoner_settings`, `preferences`, `import`, `rename`.

**Errors:** `INVALID_PARAMS` (unknown `dialog_id`)

### `ontocore/createOntology` (v0.17+)

Scaffold a new Turtle or OBO ontology file under the workspace.

**Params:** `{ "path", "ontology_iri", "version_iri"?, "format"?, "prefixes"? }`

**Result:** `{ "path", "ontology_iri" }`

**Errors:** `INVALID_PARAMS` (workspace not initialized, path outside roots, file already exists, unsafe path), `INDEX_FAILED` (I/O failure creating the file)

### `ontocore/exportOntology` (v0.17+)

Export/convert an ontology via ROBOT `convert`, with same-format copy fallback when ROBOT is unavailable.

**Params:** `{ "source_path", "output_path", "format"? }`

**Result:** `{ "output_path", "success", "logs"? }` — `success` may be `false` when ROBOT exits non-zero (still an LSP success response).

**Errors:** `INVALID_PARAMS` (path outside workspace), `ROBOT_FAILED` (ROBOT unavailable and formats differ, or copy failure)

### `ontocore/setActiveOntology` (v0.17+)

Set the active ontology id used for new-axiom targeting.

**Params:** `{ "ontology_id": string }` (document id, path, or base IRI)

**Result:** `{ "active_ontology_id": string }`

**Errors:** `NOT_FOUND` (id not in the indexed catalog), `NOT_INDEXED`

### `ontocore/deleteImpact` (v0.17+)

Preview delete impact for an entity.

**Params:** `{ "entity_iri": string }`

**Result:** `{ "entity_iri", "usage_count", "axiom_count", "referencing_entities", "warnings" }`

**Errors:** `NOT_INDEXED`

### `ontocore/runPlugin` (v0.14+, views v0.15)

Run a plugin validate/export/workflow/**ui_view** action.

**Params:**

| Field | Type | Description |
|-------|------|-------------|
| `plugin_id` | string | Plugin id from manifest |
| `action` | string? | `validate` (default), `export`, `workflow`, or **`ui_view`** (v0.15+) |
| `step` | string? | Workflow step when `action` is `workflow` |
| `view_id` | string? | **(v0.15+)** View id from `ui.views` when `action` is `ui_view` |

**Result:** `{ "diagnostics": DiagnosticSummary[], "output_paths": string[], "logs": string?, "view_html": string?, "success": boolean }`

`view_html` is populated for `ui_view` actions — HTML rendered in the plugin view panel.

Plugin diagnostics use `code` values like `plugin:<id>:<code>` and LSP `source` `ontocore-plugin:<id>`. See [errors.md](errors.md#plugin-diagnostic-codes-v014).

**Errors:** `NOT_INDEXED`, `INDEX_FAILED` (plugin not found, missing permission, unsupported action, subprocess failure, or export error)

`getCatalogSnapshot` includes plugin diagnostics merged after index (same wire codes).

## Structured errors

Custom method failures return `LspErrorPayload` in the JSON-RPC error `data` field. Full catalog: [errors.md](errors.md).

| Field | Description |
|-------|-------------|
| `code` | Machine-readable code (`NOT_INDEXED`, `INVALID_PARAMS`, `ENTITY_NOT_FOUND`, `PATCH_INVALID`, `UNSUPPORTED_FORMAT`, `INDEX_FAILED`, `QUERY_FAILED`, `MANCHESTER_INVALID`, `APPLIED_NOT_INDEXED`, `REASONER_FAILED`, `EXPLANATION_FAILED`, …) |
| `message` | Human-readable message |
| `recoverable` | Whether the client can retry |
| `user_action` | Suggested user action (optional) |

## Not implemented yet (see LSP_SPEC)

`textDocument/prepareRename` returns a range when the cursor is on a renameable IRI/QName. Additional LSP features remain planned — see [LSP_SPEC.md](design/LSP_SPEC.md). `textDocument/completion` for Turtle shipped in v0.11; semantic tokens and `listSqlSchema` shipped in v0.13. For semantic diff UX in VS Code, see [Semantic diff guide](ontocode/semantic-diff.md).
