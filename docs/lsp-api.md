# OntoIndex LSP API (v0.4)

This document describes **what ships today** in `ontoindex-lsp`. For the **v1.0 target** (including `parseManchester`, `getExplanation`, `runRobot`), see [LSP_SPEC.md](design/LSP_SPEC.md).

## Wire format (v0.4)

LSP JSON uses **snake_case** for enums serialized from Rust (`EntityKind`, `ParseStatus`, `OntologyFormat`), e.g. `"kind": "class"`, `"parse_status": "ok"`. SQL virtual tables use the same snake_case strings via `as_str()` on core enums (e.g. `ParseStatus::as_str()` → `"ok"`, `EntityKind::as_str()` → `"class"`, `axiom_kind` → `"sub_class_of"`).

**Source of truth:**

- Types: [`protocol.rs` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/crates/ontoindex-lsp/src/protocol.rs)
- JSON Schema (v0.4 subset): [`docs/lsp-protocol.schema.json`](lsp-protocol.schema.json)
- Handlers: [`handlers.rs` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/crates/ontoindex-lsp/src/handlers.rs)
- Extension client: [`client.ts` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/extension/src/lsp/client.ts)

## Transport

- stdio (VS Code language client)

## Standard LSP capabilities (v0.4)

| Capability | Status |
|------------|--------|
| `textDocument/hover` | Implemented (basic entity info) |
| `textDocument/documentSymbol` | Implemented |
| `workspace/symbol` | Implemented |
| `textDocument/definition` | Implemented |
| Diagnostics | **Implemented** — server pushes `textDocument/publishDiagnostics` after each reindex |
| Completion | Planned |
| Rename | Planned |

## Custom methods (v0.4)

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
| `hierarchy` | `ClassHierarchy` | `edges`, `parents`, `children` maps |
| `diagnostics` | `DiagnosticSummary[]` | Lint summaries for explorer |

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
| `axioms` | Human-readable axiom strings |
| `source` | Optional `{ path, line, column }` |
| `editable` | `true` for Turtle write-back in v0.4 |
| `document_path` | Filesystem path to declaring file |

**Errors:** `NOT_INDEXED`, `ENTITY_NOT_FOUND`

### `ontoindex/applyAxiomPatch` (v0.4)

Apply Turtle patch operations. See [authoring.md](authoring.md).

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

See [patch-reference.md](patch-reference.md) for CLI `ApplyPatchResult` examples and [errors.md](errors.md) for failure codes.

**Errors:** `PATCH_INVALID`, `UNSUPPORTED_FORMAT`, `NOT_INDEXED`

## Structured errors

Custom method failures return `LspErrorPayload` in the JSON-RPC error `data` field. Full catalog: [errors.md](errors.md).

| Field | Description |
|-------|-------------|
| `code` | Machine-readable code (`NOT_INDEXED`, `ENTITY_NOT_FOUND`, `PATCH_INVALID`, `UNSUPPORTED_FORMAT`, `INDEX_FAILED`, …) |
| `message` | Human-readable message |
| `recoverable` | Whether the client can retry |
| `user_action` | Suggested user action (optional) |

## Not implemented yet (see LSP_SPEC)

- `ontoindex/query`, `ontoindex/sparql`
- `ontoindex/getGraph`, `ontoindex/getSemanticDiff`, `ontoindex/runReasoner`

Use the CLI or Rust crates for SQL/SPARQL until these LSP methods land.
