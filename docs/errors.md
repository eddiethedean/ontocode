# Errors reference (OntoCore v0.12)

Unified catalog of error codes, exit behavior, and failure modes for OntoCore **v0.12.0**.

## CLI exit codes

| Command | Exit 0 | Exit non-zero |
|---------|--------|---------------|
| `ontocore validate` | No diagnostic **errors** (warnings/info allowed) | One or more diagnostic **errors** |
| `ontocore query` | Query succeeded | Parse error, unsupported SQL, I/O failure |
| `ontocore sparql` | Query succeeded (results may be truncated at row cap) | Parse error, I/O failure |
| `ontocore patch` | Patch applied or preview succeeded | Invalid patch, unsupported format, I/O failure |
| `ontocore classify` | Consistent ontology (no unsatisfiable classes) | Unsatisfiable classes, reasoner error, I/O failure |
| `ontocore explain` | Explanation produced | Class not found, explanation unavailable, reasoner error |
| `ontocore refactor` (subcommands) | Preview or apply succeeded | Invalid request, path outside workspace, I/O failure |

`validate` and `classify` exit semantics are stable for CI — see [workspace-limits.md](workspace-limits.md) and [ci-integration.md](ci-integration.md).

Other CLI commands return non-zero on failure with a human-readable message on stderr.

## LSP custom method errors (`LspErrorPayload`)

Custom `ontocore/*` method failures return JSON-RPC errors with `data` containing:

| Field | Type | Description |
|-------|------|-------------|
| `code` | string | Machine-readable code |
| `message` | string | Human-readable message |
| `recoverable` | boolean | Whether the client can retry |
| `user_action` | string? | Suggested user action |

### LSP error codes

| Code | When | Typical `user_action` |
|------|------|------------------------|
| `NOT_INDEXED` | Catalog methods called before first index | Run OntoCode: Index Workspace |
| `ENTITY_NOT_FOUND` | `getEntity` IRI not in catalog | Check IRI spelling / re-index |
| `PATCH_INVALID` | Patch JSON invalid or entity missing | Check patch parameters and entity IRIs |
| `UNSUPPORTED_FORMAT` | Write-back on non-Turtle file | Save as Turtle (.ttl) for write-back |
| `INDEX_FAILED` | Indexing failed (parse, limits, I/O) | Check ontology files for parse errors |
| `QUERY_FAILED` | SQL or SPARQL query failed | Check query syntax and [sql-reference](sql-reference.md) |
| `MANCHESTER_INVALID` | Manchester expression parse failed | Fix expression; see [Manchester editor](ontocode/manchester-editor.md) |
| `APPLIED_NOT_INDEXED` | Patch written to buffer/disk but reindex failed | Run Index Workspace; file may already be updated (`recoverable: true`) |
| `REASONER_FAILED` | `runReasoner` failed (profile, parse, OntoLogos error) | Try another profile or fix ontology axioms |
| `EXPLANATION_FAILED` | `getExplanation` failed | Run reasoner first or choose another class |
| `REFACTOR_FAILED` | Refactor preview/apply failed | Check IRIs, Turtle-only scope, and preview plan |
| `GRAPH_FAILED` | `getGraph` failed | Re-index workspace or reduce neighborhood depth |
| `ROBOT_FAILED` | `runRobot` external process failed | Check `ontocode.robotPath` and ROBOT install |

Successful patch/refactor apply may include `reindex_warning` in the result when disk write succeeded but reindex failed.

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "error": {
    "code": -32000,
    "message": "Workspace has not been indexed yet",
    "data": {
      "code": "NOT_INDEXED",
      "message": "Workspace has not been indexed yet",
      "recoverable": true,
      "user_action": "Run OntoCode: Index Workspace"
    }
  }
}
```

## Patch diagnostics (`PatchDiagnostic`)

When `ontocore patch` or `ontocore/applyAxiomPatch` fails validation, the result may include:

```json
{
  "applied": false,
  "diagnostics": [
    { "severity": "error", "message": "entity not found: http://example.org/unknown" }
  ]
}
```

| Field | Description |
|-------|-------------|
| `severity` | `error` or `warning` |
| `message` | Human-readable description |

LSP `applyAxiomPatch` returns the same `ApplyPatchResult` fields (`applied`, `preview_text`, `diagnostics`, `document_path`) plus optional `entity_detail` on success.

**Buffer-first apply (v0.5):** When called from VS Code, patches apply to the **open document buffer** first, then disk, then trigger reindex. If reindex fails after a successful write, the server returns `APPLIED_NOT_INDEXED` — the buffer/disk may already contain the patch.

## Lint diagnostic codes (`diagnostics` table)

| Code | Severity | Meaning |
|------|----------|---------|
| `parse_error` | error | File failed to parse |
| `broken_import` | error/warning | Import IRI could not be resolved |
| `undefined_prefix` | warning | Unknown prefix in file |
| `duplicate_label` | warning | Same label on multiple entities |
| `missing_label` | info/warning | Entity has no rdfs:label |
| `orphan_class` | info | Class with no declared parents |
| `owl_bridge_failed` | warning | Horned-OWL bridge failed; catalog uses parser-only semantics for that file |
| `io_read_error` | error | Diagnostic engine could not read file from disk |

Query diagnostics: `SELECT code, severity, message, file FROM diagnostics WHERE severity = 'error'`

## Workspace limit failures

| Limit | Constant | Typical failure |
|-------|----------|-----------------|
| Files scanned | `MAX_SCAN_FILES` (10,000) | Scanner error during index |
| File size | `MAX_FILE_BYTES` (50 MB) | File skipped or index error |
| Open LSP buffers | `MAX_OPEN_DOCUMENTS` (256) | Document not tracked |
| Entities | `MAX_ENTITIES` (1,000,000) | Catalog build error |
| Triples total | `MAX_TOTAL_TRIPLES` (20M) | Index error |
| Triples per file | `MAX_TRIPLES_PER_FILE` (5M) | Per-file index error |
| Query size | `MAX_QUERY_BYTES` (1 MiB) | Query rejected |
| SQL rows | `MAX_SQL_RESULT_ROWS` (100k) | **Silent truncation** (`truncated: true` in LSP) |
| SPARQL rows | `MAX_SPARQL_RESULT_ROWS` (100k) | **Silent truncation** (`truncated: true` in LSP) |

Full limits: [workspace-limits.md](workspace-limits.md).

## Rust library errors

Integrators using `ontocore-*` crates directly should handle:

| Crate | Error type | Common causes |
|-------|------------|---------------|
| `ontocore-parser` | `ParseError` | Invalid RDF syntax |
| `ontocore-catalog` | `CatalogError` | Index build failure |
| `ontocore-query` | `QueryError` | Unsupported SQL, SPARQL parse error |
| `ontocore-owl` | `OwlError` | Patch apply failure |
| `ontocore-reasoner` | `ReasonerError` | Classify/explain failure, unsupported profile |

Example: [`examples/error_handling.rs`](https://github.com/eddiethedean/ontocode/blob/main/examples/error_handling.rs) on GitHub.

## Related docs

- [LSP API](lsp-api.md) — custom methods and result types
- [patch-reference.md](patch-reference.md) — patch operations
- [sql-reference.md](sql-reference.md) — virtual tables and SQL subset
- [faq.md](faq.md) — common troubleshooting
- [troubleshooting.md](troubleshooting.md) — step-by-step fixes
