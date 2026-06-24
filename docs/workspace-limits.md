# Workspace resource limits

OntoIndex enforces limits to keep local indexing predictable and to reduce DoS risk when
opening untrusted ontology repositories.

Constants live in [`limits.rs` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/crates/ontoindex-core/src/limits.rs).

## Recommended workspace size

| Guideline | Value |
|-----------|-------|
| Ontology files | Up to **10,000** per workspace (`MAX_SCAN_FILES`) |
| Single file size | Up to **50 MB** on disk or in an LSP open buffer (`MAX_FILE_BYTES`) |
| Open editor buffers (LSP) | Up to **256** tracked paths (`MAX_OPEN_DOCUMENTS`) |
| Entities | Up to **1,000,000** per workspace (`MAX_ENTITIES`) |
| RDF triples | Up to **20,000,000** total (`MAX_TOTAL_TRIPLES`) |

Workspaces above these limits may fail indexing with a clear error. For very large
terminologies (e.g. full OBO), prefer CLI batch workflows and expect v0.4+ incremental
indexing improvements.

## Query limits

| Limit | Value |
|-------|-------|
| SQL/SPARQL query string | **1 MB** (`MAX_QUERY_BYTES`) |
| SQL result rows | **100,000** (truncated) |
| SPARQL result rows | **100,000** (error) |

## `ontoindex validate` exit codes

| Outcome | Exit code |
|---------|-----------|
| No diagnostic **errors** (warnings/info allowed) | **0** |
| One or more diagnostic **errors** | **non-zero** |

Warnings and info diagnostics are printed to stderr but do not fail CI.
