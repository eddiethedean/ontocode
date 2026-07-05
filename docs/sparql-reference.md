# SPARQL reference (OntoCore v0.11)

> **Status:** Documents behavior in **OntoCore v0.11.0**. Pre-1.0 APIs may change.
> Canonical feature list: [What ships today](SHIPPED.md).

Run SPARQL queries over the **indexed triple store** built from workspace ontology files.

```bash
ontocore sparql /path/to/workspace "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10"
```

From a git clone:

```bash
cargo run -- sparql fixtures "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 5"
```

**Source of truth:** [`sparql.rs` on GitHub](https://github.com/eddiethedean/ontocode/blob/main/crates/ontocore-query/src/sparql.rs) (Oxigraph SPARQL engine).

## Behavior

- Queries run against triples parsed from all indexed files in the workspace
- The workspace is re-indexed on each CLI invocation (same as `query` and `validate`)
- Result formats: text (default), JSON (`--format json`)
- SQL virtual tables are separate — use `ontocore query` for catalog tables like `classes`

## Prefixes and IRIs

- Use **full IRIs** in queries, or declare prefixes in the SPARQL query:

```sparql
PREFIX ex: <http://example.org/people#>
SELECT ?label WHERE { ex:Person rdfs:label ?label }
```

- Prefixes from Turtle `@prefix` declarations in source files are **not** automatically injected into SPARQL — declare them in the query string.

## Examples

```bash
# All triples (limited)
ontocore sparql fixtures "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 5"

# Labels for a known IRI
ontocore sparql fixtures "SELECT ?label WHERE { <http://example.org/people#Person> rdfs:label ?label }"

# Count classes (approximate via rdf:type)
ontocore sparql fixtures "SELECT (COUNT(?c) AS ?count) WHERE { ?c a owl:Class }"

# JSON output
ontocore sparql fixtures "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 3" --format json
```

More examples: [query cookbook on GitHub](https://github.com/eddiethedean/ontocode/blob/main/examples/queries.md).

## Limits

| Limit | Value |
|-------|-------|
| Query string size | 1 MiB (`MAX_QUERY_BYTES`) |
| Result rows | 100,000 (silently truncated; LSP sets `truncated: true`) |

See [workspace-limits.md](workspace-limits.md).

## LSP

SPARQL is available via the CLI, Rust API, LSP (`ontocore/sparql`), and the VS Code **Query Workbench** (v0.5+). See [lsp-api.md](lsp-api.md).

## Related

- SQL-like catalog queries: [sql-reference.md](sql-reference.md)
- Query cookbook: [query cookbook on GitHub](https://github.com/eddiethedean/ontocode/blob/main/examples/queries.md)
