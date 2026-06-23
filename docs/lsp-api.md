# OntoIndex LSP API (implemented in v0.2)

This document describes **what ships today** in `ontoindex-lsp`. For the **v1.0 target** (including `applyAxiomPatch`, `parseManchester`, `getExplanation`, `runRobot`), see [LSP_SPEC.md](design/LSP_SPEC.md) — those methods are planned, not implemented in v0.2.

## Wire format (v0.2)

LSP JSON uses **snake_case** for enums serialized from Rust (`EntityKind`, `ParseStatus`, `OntologyFormat`), e.g. `"kind": "class"`, `"parse_status": "ok"`. The SQL CLI uses the same snake_case strings via `EntityKind::as_str()` — not PascalCase serde names.

**Source of truth:**

- Types: [`crates/ontoindex-lsp/src/protocol.rs`](../crates/ontoindex-lsp/src/protocol.rs)
- Handlers: [`crates/ontoindex-lsp/src/handlers.rs`](../crates/ontoindex-lsp/src/handlers.rs)
- Extension client: [`extension/src/lsp/client.ts`](../extension/src/lsp/client.ts)

## Transport

- stdio (VS Code language client)

## Standard LSP capabilities (v0.2)

| Capability | Status |
|------------|--------|
| `textDocument/hover` | Implemented (basic entity info) |
| `textDocument/documentSymbol` | Implemented |
| `workspace/symbol` | Implemented |
| `textDocument/definition` | Implemented |
| Diagnostics | **Implemented** (v0.3) — server pushes `textDocument/publishDiagnostics` after each reindex |
| Completion | Planned |
| Rename | Planned |

## Custom methods (v0.2)

All custom methods use the `ontoindex/` prefix.

### `ontoindex/indexWorkspace`

Rebuild the workspace catalog.

**Params:** `IndexWorkspaceParams`

```json
{ "workspaceUri": "file:///path/to/workspace" }
```

`workspaceUri` is optional; the server uses the initialized workspace folder when omitted.

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

**Result:** `GetEntityResult` with `EntityDetail` (entity, parents, children, axioms, optional source location).

**Errors:** `NOT_INDEXED`, `ENTITY_NOT_FOUND`

## Structured errors

Custom method failures return `LspErrorPayload` (Rust type; not `ontoindex_core::OntoIndexError`):

| Field | Description |
|-------|-------------|
| `code` | Machine-readable code (`NOT_INDEXED`, `ENTITY_NOT_FOUND`, `INDEX_FAILED`, …) |
| `message` | Human-readable message |
| `recoverable` | Whether the client can retry |
| `user_action` | Suggested user action (optional) |

## Not implemented yet (see LSP_SPEC)

- `ontoindex/query`, `ontoindex/sparql`
- `ontoindex/getGraph`, `ontoindex/getSemanticDiff`, `ontoindex/runReasoner`

Use the CLI or Rust crates for SQL/SPARQL until these LSP methods land.
