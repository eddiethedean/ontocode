# OntoIndex LSP API (v0.8)

> **Status:** Documents behavior in **OntoIndex v0.8.0**. Pre-1.0 APIs may change.
> Canonical feature list: [What ships today](SHIPPED.md).

This document describes **what ships today** in `ontoindex-lsp`. For the **v1.0 target** (including `runRobot`, graph APIs), see [LSP_SPEC.md](design/LSP_SPEC.md).

## Wire format

LSP JSON uses **snake_case** for enums serialized from Rust (`EntityKind`, `ParseStatus`, `OntologyFormat`), e.g. `"kind": "class"`, `"parse_status": "ok"`. SQL virtual tables use the same snake_case strings via `as_str()` on core enums (e.g. `ParseStatus::as_str()` → `"ok"`, `EntityKind::as_str()` → `"class"`, `axiom_kind` → `"sub_class_of"`).

**Source of truth:**

- Types: [`protocol.rs` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/crates/ontoindex-lsp/src/protocol.rs)
- JSON Schema (partial v0.7 subset): [`docs/lsp-protocol.schema.json`](lsp-protocol.schema.json) — covers core query/patch methods; `getGraph` and `runRobot` are documented below but not yet in the schema.
- Handlers: [`handlers.rs` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/crates/ontoindex-lsp/src/handlers.rs)
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
| Completion | Planned |
| Rename | **Implemented** — `textDocument/rename` (no `prepareRename` yet) |
| Find references | **Implemented** — `textDocument/references` |

## Custom methods

All custom methods use the `ontoindex/` prefix.

### `ontoindex/indexWorkspace`

Rebuild the workspace catalog.

**Params:** `IndexWorkspaceParams`

```json
{ "workspace_uri": "file:///path/to/workspace" }
```

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

### `ontoindex/getCatalogSnapshot`

Return documents, entities, and class hierarchy for the explorer UI.

**Params:** `null`

**Result:** `CatalogSnapshot`

| Field | Type | Description |
|-------|------|-------------|
| `documents` | `OntologyDocument[]` | Indexed files (`id`, `path`, `format`, `parse_status`, …) |
| `entities` | `Entity[]` | All extracted entities |
| `hierarchy` | `ClassHierarchy` | `edges`, `parents`, `children` maps (asserted) |
| `diagnostics` | `DiagnosticSummary[]` | Lint summaries for explorer |
| `reasoner` | `ReasonerSnapshot` (optional) | Last reasoner run — inferred hierarchy, unsatisfiable classes |

**`Entity` fields:** `iri`, `short_name`, `kind`, `ontology_id`, `labels[]`, `comments[]`, `deprecated`, optional `source_location`.

**Errors:** `NOT_INDEXED` if the workspace has not been indexed.

### `ontoindex/getEntity`

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
| `editable` | `true` for Turtle write-back |
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

### `ontoindex/query`

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

### `ontoindex/sparql`

Run SPARQL against the indexed catalog store.

**Params:** `SparqlParams`

```json
{ "query": "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10" }
```

**Result:** `TabularQueryResult` (same shape as `ontoindex/query`).

**Errors:** `NOT_INDEXED`, `QUERY_FAILED`

### `ontoindex/parseManchester`

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

### `ontoindex/applyAxiomPatch`

Apply Turtle patch operations. See [authoring.md](authoring.md).

**Buffer-first (VS Code):** Reads the open document buffer when available, applies patches in memory, updates the buffer, writes disk, then reindexes. See [errors.md](errors.md) for `APPLIED_NOT_INDEXED`.

**Params:** `ApplyAxiomPatchParams`

```json
{
  "document_uri": "file:///path/to/ontology.ttl",
  "patches": [
    { "op": "add_label", "entity_iri": "http://ex#Person", "value": "Human" }
  ],
  "preview_only": false
}
```

**Result:** `ApplyAxiomPatchResult` (flattened patch result + optional entity):

| Field | Description |
|-------|-------------|
| `applied` | `true` if patches were written (false for preview-only or validation failure) |
| `preview_text` | Turtle preview when `preview_only: true` |
| `diagnostics` | `PatchDiagnostic[]` on failure (`severity`, `message`) |
| `document_path` | Path to modified file |
| `entity_detail` | Updated `EntityDetail` after successful apply (LSP only) |
| `reindex_warning` | Present when apply succeeded but reindex failed |

**`EntityAxiomSummary` kinds:** `sub_class_of`, `equivalent_class`, `disjoint_class`, `domain`, `range`, `property_chain` (property chains are view-only).

See [patch-reference.md](patch-reference.md) for CLI `ApplyPatchResult` examples and [errors.md](errors.md) for failure codes.

**Errors:** `PATCH_INVALID`, `UNSUPPORTED_FORMAT`, `NOT_INDEXED`, `APPLIED_NOT_INDEXED`

### `ontoindex/runReasoner`

Run OWL classification via OntoLogos 0.9.0 (`el`, `rl`, `rdfs`). `dl` and `auto` return an error until OntoLogos 1.0.

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

### `ontoindex/getExplanation`

Return an EL/RL/RDFS explanation for an unsatisfiable class (requires a prior reasoner run or loads ontology on demand).

**Params:** `GetExplanationParams`

```json
{ "class_iri": "http://example.org#Invalid", "profile": "el" }
```

**Result:** `GetExplanationResult`

| Field | Description |
|-------|-------------|
| `class_iri` | Requested class |
| `steps` | Ordered `ExplanationStep[]` (`index`, `rule`, `display`, optional IRIs) |
| `text` | Rendered proof text |

**Errors:** `NOT_INDEXED`, `EXPLANATION_FAILED`

### `ontoindex/getGraph` (v0.7)

Returns graph nodes and edges for visualization webviews.

**Params:**

| Field | Type | Description |
|-------|------|-------------|
| `graph_kind` | string | `class`, `property`, `import`, or `neighborhood` |
| `root_iri` | string? | Required for `neighborhood` |
| `depth` | number? | BFS depth for neighborhood (default 2, max 5) |
| `include_inferred` | boolean? | Include reasoner edges when snapshot exists |
| `filters` | object? | `ontology_iri`, `hide_deprecated` |

**Result:** `{ graph: { nodes, edges, truncated, graph_kind } }`

### `ontoindex/runRobot` (v0.7)

Runs a ROBOT CLI subcommand.

**Params:** `{ subcommand, args?, robot_path? }`

**Result:** `{ exit_code, stdout, stderr }`

### `ontoindex/findUsages` (v0.8)

Return structured IRI usages across the workspace catalog and Turtle text spans.

**Params:** `{ "iri": "http://example.org/people#Person" }`

**Result:** `{ "usages": UsageSummary[] }` — each usage has `iri`, `referenced_iri`, `file`, `line`, `column`, `kind`, `context` (byte ranges are not serialized on the wire).

### `ontoindex/previewRefactor` (v0.8)

Build a refactor plan without writing files.

**Params:** flattened refactor request — one of:

| `kind` | Fields |
|--------|--------|
| `rename_iri` | `from_iri`, `to_iri` |
| `migrate_namespace` | `from_base`, `to_base` |
| `move_entity` | `entity_iri`, `target_file` |
| `extract_module` | `entity_iris[]`, `output_file`, `leave_stub?` |

**Result:** `RefactorPlan` — `{ changes: FileChange[], warnings: string[] }` where each change has `path`, `preview_text`, `original_text`, and `hunks`.

### `ontoindex/applyRefactor` (v0.8)

Apply a previously previewed refactor plan, reindex, and return write summary.

**Params:** `{ "plan": RefactorPlan, "preview_only"?: boolean }`

**Result:** `{ "files_written": number, "reindex_warning"?: string }`

## Structured errors

Custom method failures return `LspErrorPayload` in the JSON-RPC error `data` field. Full catalog: [errors.md](errors.md).

| Field | Description |
|-------|-------------|
| `code` | Machine-readable code (`NOT_INDEXED`, `ENTITY_NOT_FOUND`, `PATCH_INVALID`, `UNSUPPORTED_FORMAT`, `INDEX_FAILED`, `QUERY_FAILED`, `MANCHESTER_INVALID`, `APPLIED_NOT_INDEXED`, `REASONER_FAILED`, `EXPLANATION_FAILED`, …) |
| `message` | Human-readable message |
| `recoverable` | Whether the client can retry |
| `user_action` | Suggested user action (optional) |

## Not implemented yet (see LSP_SPEC)

- `ontoindex/getSemanticDiff`

Use the CLI or Rust crates for semantic diff until that LSP method lands.
