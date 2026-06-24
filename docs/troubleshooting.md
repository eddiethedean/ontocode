# Troubleshooting

Common problems and fixes for OntoCode (VS Code) and OntoIndex (CLI/LSP).

For quick answers, see also [FAQ](faq.md).

## VS Code: explorer empty or stale

1. Run **OntoCode: Index Workspace** from the Command Palette.
2. Check **View → Output → OntoIndex Language Server** for errors.
3. Confirm the folder contains supported files (`.ttl`, `.owl`, `.rdf`, `.jsonld`, `.nt`, `.nq`, `.trig`).
4. **Multi-root workspace:** only the **first** folder is indexed — open the ontology project as a single-root folder or put it first.

## VS Code: language server failed to start

1. **Trust** the workspace (Restricted Mode blocks custom `ontocode.lspPath`).
2. Uninstall duplicate OntoCode extension versions.
3. Check **Output → OntoIndex Language Server** for the exact error.
4. Set `ontocode.lspPath` to a local `ontoindex-lsp` binary (`cargo install ontoindex-lsp`) — trusted workspaces only.
5. See [Install VS Code](vscode-install.md#troubleshooting).

## VS Code: cannot edit in inspector

- Write-back is **Turtle (`.ttl`) only**. RDF/XML, OWL XML, and JSON-LD are read-only in the inspector.
- Entity must be declared in an indexed `.ttl` file in the workspace.

## VS Code: patch or Manchester apply did not stick

Patches apply to the **open editor buffer** first, then disk:

1. If you have **unsaved** edits, the patch merges into the buffer — save or review the buffer content.
2. If re-index fails after a successful write, the LSP may return `APPLIED_NOT_INDEXED` — file/buffer updated but catalog stale. Run **Index Workspace**.
3. Check **Output → OntoIndex Language Server** and [errors reference](errors.md).

## CLI: `ontoindex query ./fixtures` fails

The `fixtures/` directory exists only in a **git clone**, not after `cargo install`:

```bash
ontoindex query /path/to/your/ontologies "SELECT * FROM classes"
```

## CLI: `validate` exits non-zero

Exit code **0** only when there are no diagnostic **errors** (warnings are allowed). Query errors:

```bash
ontoindex query /path/to/ontologies "SELECT code, severity, message FROM diagnostics WHERE severity = 'error'"
```

See [CI integration](ci-integration.md).

## Queries return no rows or wrong data

1. Re-index: `ontoindex validate` or VS Code **Index Workspace**.
2. Confirm SQL table name and column names — [SQL reference](sql-reference.md).
3. SPARQL runs over indexed triples — prefix declarations must be valid in source files.

## Results truncated at 100,000 rows

Both SQL and SPARQL cap results at **100,000 rows** with silent truncation. In the Query Workbench or LSP responses, check `truncated: true`. Narrow your query with `WHERE` or SPARQL `LIMIT`.

See [workspace limits](workspace-limits.md).

## Patch JSON errors

| Symptom | Likely cause |
|---------|--------------|
| `entity not found` | Wrong IRI or entity not in target `.ttl` file; check `@prefix` declarations |
| `unsupported format` | Patch target is not Turtle (`.ttl`) |
| `applied: false` with diagnostics | Invalid patch op or Manchester expression — see [patch reference](patch-reference.md) |

## Workspace too large

Indexing may fail above [workspace limits](workspace-limits.md) (file count, size, triple caps). For very large terminologies, use CLI batch workflows on a subset.

## Reasoner

| Problem | What to try |
|---------|-------------|
| `dl` or `auto` profile fails | Full DL requires OntoLogos 1.0 — use `el`, `rl`, or `rdfs` |
| Inferred hierarchy not visible | Run **OntoCode: Run Reasoner**, then **Set Hierarchy Mode** → inferred or combined |
| Explanation panel empty | Explanations need an unsatisfiable class; run reasoner first |
| Classify exits non-zero in CI | Ontology has unsatisfiable classes — inspect JSON `unsatisfiable` list |

See [Reasoner guide](guides/reasoner.md).

## Still stuck?

- [FAQ](faq.md)
- [Errors reference](errors.md)
- [Report an issue](https://github.com/eddiethedean/ontocode/issues)
