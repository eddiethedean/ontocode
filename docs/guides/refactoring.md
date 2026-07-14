# Workspace refactoring

OntoCode v0.8 adds **workspace-wide refactoring** for Turtle (`.ttl`) ontologies: find usages, rename IRIs, migrate namespaces, move entities between files, and extract modules. All operations support **preview before apply**.

Canonical capability matrix: [What ships today](../SHIPPED.md).

## Supported operations

| Operation | VS Code | CLI |
|-----------|---------|-----|
| Find usages of an IRI | Entity Inspector, explorer context menu | `ontocore refactor usages` |
| Rename entity IRI | **Rename Entity IRI** command | `ontocore refactor rename` |
| Migrate namespace base | **Migrate Namespace** command | `ontocore refactor migrate-namespace` |
| Move entity to another file | **Move Entity** command | `ontocore refactor move` |
| Extract module (subset of entities) | **Extract Module** command | `ontocore refactor extract` |
| Merge entities | **Merge Entities** command | — (IDE only) |
| Replace entity references | **Replace Entity** command | — (IDE only) |

**Format policy:** refactoring applies to **Turtle (`.ttl`) only**. RDF/XML, OBO, and other formats are indexed but not modified.

## VS Code workflow

### Find usages

1. Select an entity in the **Classes**, **Properties**, or **Individuals** view.
2. Right-click → **Find Entity Usages**, or use the **Find Usages** action in the Entity Inspector.
3. Review the usage list (file, line, kind, context).

### Merge or replace entities

1. Command Palette → **OntoCode: Merge Entities** or **OntoCode: Replace Entity**.
2. Enter survivor/duplicate IRIs (merge) or source/target IRIs (replace).
3. Review the **Refactor Preview** panel and click **Apply**.

Rename / merge / replace also run from the CLI (`ontocore refactor …`). They rewrite Turtle plus RDF/XML, OWL/XML, and OBO where remaps apply (non-Turtle files may be skipped with warnings).

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
| **OntoCode: Merge Entities** | Merge two entities (survivor + duplicate) across Turtle files |
| **OntoCode: Replace Entity** | Replace references to one IRI with another (preview + apply) |

Use **`--preview`** before apply in production repos.

After apply, the workspace re-indexes automatically. If re-index fails, you may see `reindex_warning` — run **OntoCode: Index Workspace**.

## CLI examples

Replace `fixtures` with your ontology root (or use `cargo run --` from a git clone).

### Merge or replace entities

```bash
ontocore refactor merge fixtures \
  --keep 'http://example.org/people#Person' \
  --merge 'http://example.org/people#Human' \
  --preview

ontocore refactor replace fixtures \
  --from 'http://example.org/people#OldName' \
  --to 'http://example.org/people#NewName' \
  --preview
```

| Flag | Description |
|------|-------------|
| `--keep` / `--merge` | Survivor and duplicate IRIs for merge |
| `--from` / `--to` | Source and target IRIs for replace |
| `--preview` | Print plan without writing files |
| `--format` | `text` (default) or `json` |

### Find usages

```bash
ontocore refactor usages fixtures 'http://example.org/people#Person'
ontocore refactor usages fixtures 'http://example.org/people#Person' --format json
```

### Rename IRI (preview then apply)

```bash
ontocore refactor rename fixtures \
  --from 'http://example.org/people#Person' \
  --to 'http://example.org/people#Human' \
  --preview

ontocore refactor rename fixtures \
  --from 'http://example.org/people#Person' \
  --to 'http://example.org/people#Human'
```

### Migrate namespace

```bash
ontocore refactor migrate-namespace fixtures \
  --from 'http://example.org/people#' \
  --to 'http://example.org/v2/people#' \
  --preview
```

### Move entity

```bash
ontocore refactor move fixtures 'http://example.org/people#Student' \
  --to ./people/students.ttl \
  --preview
```

### Extract module

```bash
ontocore refactor extract fixtures \
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
| `ontocore/findUsages` | List usages for an IRI |
| `ontocore/previewRefactor` | Build a `RefactorPlan` without writing |
| `ontocore/applyRefactor` | Apply a plan (re-previews and verifies match) |

Refactor requests use a tagged `kind` field: `rename_iri`, `migrate_namespace`, `move_entity`, `extract_module`. See [LSP API](../lsp-api.md) and [lsp-protocol.schema.json](../lsp-protocol.schema.json).

## Safety and limitations

| Topic | Notes |
|-------|-------|
| Preview | Always preview multi-file changes before apply in production repos |
| Open buffers | LSP applies to open editor buffers first, then disk (same as patches) |
| Multi-root workspace | All folders indexed (v0.10+); refactor applies across indexed Turtle in every root |
| Non-Turtle files | Refactor apply is **Turtle-only** — `.obo`, `.owl`, etc. are skipped (use inspector/patch for OBO) |
| Extract module | Direct-reference closure; may not capture all indirect imports |
| Git | Review diffs before commit; use [semantic diff](../ontocode/semantic-diff.md) for PR summaries |

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
