# Troubleshooting

Common problems and fixes for OntoCode (VS Code) and OntoCore (CLI/LSP).

For quick answers, see also [FAQ](faq.md).

## VS Code: explorer empty or stale

1. Run **OntoCode: Index Workspace** from the Command Palette.
2. Check **View → Output → OntoCore Language Server** for errors.
3. Confirm the folder contains supported files (`.ttl`, `.owl`, `.rdf`, `.jsonld`, `.nt`, `.nq`, `.trig`).
4. **Multi-root workspace:** since v0.10 all folders are indexed — confirm each root contains ontology files and check **Output → OntoCore Language Server** for per-root errors.

## VS Code: language server failed to start

1. **Trust** the workspace (Restricted Mode blocks custom `ontocode.lspPath`).
2. Uninstall duplicate OntoCode extension versions.
3. Check **Output → OntoCore Language Server** for the exact error.
4. Set `ontocode.lspPath` to a local `ontocore-lsp` binary (`cargo install ontocore-lsp`) — trusted workspaces only.
5. See [Install VS Code](vscode-install.md#troubleshooting).

## VS Code: cannot edit in inspector

- Write-back is **Turtle (`.ttl`) only**. RDF/XML, OWL XML, and JSON-LD are read-only in the inspector.
- Entity must be declared in an indexed `.ttl` file in the workspace.

## VS Code: patch or Manchester apply did not stick

Patches write the **`.ttl` file on disk** first, then update the language server’s open-buffer copy (and return a workspace edit for the editor):

1. If the editor still shows old text, accept or re-apply the workspace edit, or reload the file from disk.
2. If you had **unsaved** local edits that conflicted, review the file on disk — the server applied against its buffer/disk snapshot.
3. If re-index fails after a successful write, the catalog may be stale. Run **OntoCode: Index Workspace**.
4. Check **View → Output → OntoCore Language Server** and [errors reference](errors.md).

## CLI: `ontocore query ./fixtures` fails

The `fixtures/` directory exists only in a **git clone**, not after `cargo install`:

```bash
ontocore query /path/to/your/ontologies "SELECT * FROM classes"
```

## CLI: `validate` exits non-zero

Exit code **0** only when there are no diagnostic **errors** (warnings are allowed). Query errors:

```bash
ontocore query /path/to/ontologies "SELECT code, severity, message FROM diagnostics WHERE severity = 'error'"
```

See [CI integration](ci-integration.md).

## Queries return no rows or wrong data

1. Re-index: `ontocore validate` or VS Code **Index Workspace**.
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

## Graphs, OBO, ROBOT, and semantic diff

| Problem | What to try |
|---------|-------------|
| Semantic diff: `no git repository` | Open a git checkout; or use CLI `ontocore diff --left ./a --right ./b` |
| Semantic diff panel empty | Trust workspace; run **Index Workspace**; see [Semantic diff](ontocode/semantic-diff.md) |
| Graph commands missing | Run **Index Workspace** first — [Graph view](ontocode/graph-view.md) |
| Cannot edit `.obo` in inspector | OBO is read-only in VS Code; use Turtle write-back or external tools — [OBO guide](guides/obo-workflow.md) |
| `robot` not found | Install Java + ROBOT; set `ontocode.robotPath` — [ROBOT guide](guides/robot-interop.md) |

## Reasoner

| Problem | What to try |
|---------|-------------|
| `dl` or `auto` profile fails | Check ontology constructs and profile warnings; try `el` for EL-only ontologies |
| Inferred hierarchy not visible | Run **OntoCode: Run Reasoner**, then **Set Hierarchy Mode** → inferred or combined |
| Explanation panel empty | Explanations need an unsatisfiable class; run reasoner first |
| Classify exits non-zero in CI | Ontology has unsatisfiable classes — inspect JSON `unsatisfiable` list |

See [Reasoner guide](guides/reasoner.md).

## Still stuck?

- [FAQ](faq.md)
- [Errors reference](errors.md)
- [Report an issue](https://github.com/eddiethedean/ontocode/issues)
