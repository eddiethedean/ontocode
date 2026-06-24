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

`workspace_uri` is optional; the server uses the initialized workspace folder when omitted. Legacy clients may send `workspaceUri` (camelCase); support will be removed in v0.4.

**Result:** `IndexWorkspaceResult`

```json
{
  "stats": { "class_count": 2, "ontology_count": 1, "...": "..." },
  "indexed_at": 1718000000
}
```

### `ontoindex/getCatalogSnapshot`

Return documents, entities, and class hierarchy for the explorer UI.

**Params:** `null`

**Result:** `CatalogSnapshot`

```json
{
  "documents": [ "... OntologyDocument ..." ],
  "entities": [ "... Entity ..." ],
  "hierarchy": { "edges": [], "parents": {}, "children": {} },
  "diagnostics": [ "... DiagnosticSummary ..." ]
}
```

**Errors:** `NOT_INDEXED` if the workspace has not been indexed.

### `ontoindex/getEntity`

Return detailed entity information for the inspector.

**Params:** `GetEntityParams`

```json
{ "iri": "http://example.org/people#Person" }
```

**Result:** `GetEntityResult` with `EntityDetail` (entity, parents, children, axioms, optional source, `editable`, `document_path`).

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

**Result:** `ApplyPatchResult` — `applied`, optional `preview_text`, `entity_detail` after apply.

**Errors:** `PATCH_INVALID`, `UNSUPPORTED_FORMAT`, `NOT_INDEXED`

## Structured errors

Custom method failures return `LspErrorPayload` (Rust type; not `ontoindex_core::OntoIndexError`):

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
