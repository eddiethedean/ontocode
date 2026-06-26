# Workspace refactoring

OntoCode v0.8 adds **workspace-wide refactoring** for Turtle (`.ttl`) ontologies: find usages, rename IRIs, migrate namespaces, move entities between files, and extract modules. All operations support **preview before apply**.

Canonical capability matrix: [What ships today](../SHIPPED.md).

## Supported operations

| Operation | VS Code | CLI |
|-----------|---------|-----|
| Find usages of an IRI | Entity Inspector, explorer context menu | `ontoindex refactor usages` |
| Rename entity IRI | **Rename Entity IRI** command | `ontoindex refactor rename` |
| Migrate namespace base | **Migrate Namespace** command | `ontoindex refactor migrate-namespace` |
| Move entity to another file | **Move Entity** command | `ontoindex refactor move` |
| Extract module (subset of entities) | **Extract Module** command | `ontoindex refactor extract` |

**Format policy:** refactoring applies to **Turtle (`.ttl`) only**. RDF/XML, OBO, and other formats are indexed but not modified.

## VS Code workflow

### Find usages

1. Select an entity in the **Classes**, **Properties**, or **Individuals** view.
2. Right-click → **Find Entity Usages**, or use the **Find Usages** action in the Entity Inspector.
3. Review the usage list (file, line, kind, context).

### Rename, migrate, move, or extract

1. Select an entity (for rename/move) or run the command from the Command Palette.
2. Enter parameters (new IRI, namespace base, target file, or entity set for extract).
3. The **Refactor Preview** panel opens with per-file diffs.
4. Review warnings (if any) and click **Apply** to write changes, or cancel.

Standard LSP **Rename** (`F2` on an IRI in a `.ttl` file) also triggers IRI rename when the symbol is a known entity.

| Command | When to use |
|---------|-------------|
| **OntoCode: Find Entity Usages** | Locate all references before a manual edit |
| **OntoCode: Rename Entity IRI** | Change one entity's IRI across the workspace |
| **OntoCode: Migrate Namespace** | Replace a namespace base IRI (`@prefix` + term IRIs) |
| **OntoCode: Move Entity** | Move an entity block to another `.ttl` file |
| **OntoCode: Extract Module** | Copy selected entities into a new module file |

After apply, the workspace re-indexes automatically. If re-index fails, you may see `reindex_warning` — run **OntoCode: Index Workspace**.

## CLI examples

Replace `fixtures` with your ontology root (or use `cargo run --` from a git clone).

### Find usages

```bash
ontoindex refactor usages fixtures 'http://example.org/people#Person'
ontoindex refactor usages fixtures 'http://example.org/people#Person' --format json
```

### Rename IRI (preview then apply)

```bash
ontoindex refactor rename fixtures \
  --from 'http://example.org/people#Person' \
  --to 'http://example.org/people#Human' \
  --preview

ontoindex refactor rename fixtures \
  --from 'http://example.org/people#Person' \
  --to 'http://example.org/people#Human'
```

### Migrate namespace

```bash
ontoindex refactor migrate-namespace fixtures \
  --from 'http://example.org/people#' \
  --to 'http://example.org/v2/people#' \
  --preview
```

### Move entity

```bash
ontoindex refactor move fixtures 'http://example.org/people#Student' \
  --to ./people/students.ttl \
  --preview
```

### Extract module

```bash
ontoindex refactor extract fixtures \
  --entities 'http://example.org/people#Person,http://example.org/people#Student' \
  --out ./people/core.ttl \
  --leave-stub \
  --preview
```

`--leave-stub` keeps import stubs in the original files. Extract uses direct-reference closure for related axioms.

Full flag reference: [CLI reference](../cli-reference.md#refactor).

## LSP integrators

| Method | Purpose |
|--------|---------|
| `ontoindex/findUsages` | List usages for an IRI |
| `ontoindex/previewRefactor` | Build a `RefactorPlan` without writing |
| `ontoindex/applyRefactor` | Apply a plan (re-previews and verifies match) |

Refactor requests use a tagged `kind` field: `rename_iri`, `migrate_namespace`, `move_entity`, `extract_module`. See [LSP API](../lsp-api.md) and [lsp-protocol.schema.json](../lsp-protocol.schema.json).

## Safety and limitations

| Topic | Notes |
|-------|-------|
| Preview | Always preview multi-file changes before apply in production repos |
| Open buffers | LSP applies to open editor buffers first, then disk (same as patches) |
| Multi-root workspace | Only the **first** folder is indexed and refactored |
| Non-Turtle files | Skipped — no write-back on `.owl`, `.obo`, etc. |
| Extract module | Direct-reference closure; may not capture all indirect imports |
| Git | Review diffs before commit; no semantic diff yet (v0.9 target) |

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| Preview shows no changes | Confirm entity exists in a `.ttl` file; run **Index Workspace** |
| Apply failed / plan mismatch | Re-run preview — files changed since preview was generated |
| `reindex_warning` after apply | File written but catalog stale — **Index Workspace** |
| Namespace migration overwrote renames | Migrate updates `@prefix` and IRIs under the base — preview carefully |

More: [Troubleshooting](../troubleshooting.md) · [Errors reference](../errors.md)

## Related

- [Authoring](../authoring.md) — single-entity Turtle patches
- [Patch reference](../patch-reference.md) — patch JSON for CI automation
- [CLI reference](../cli-reference.md)
- [Examples: refactoring](../examples/refactoring.md)
