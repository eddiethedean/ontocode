# Semantic diff cookbook

Compare ontology catalogs between git refs, directories, or snapshots. See [Semantic diff guide](../ontocode/semantic-diff.md).

## Git worktree compare

Requires a **git repository** at the ontology root:

```bash
cd /path/to/your/ontology-repo
ontocore diff HEAD..WORKTREE
ontocore diff main..feature --format markdown
ontocore diff --breaking-only main..HEAD
```

## PR summary (v0.13+)

```bash
ontocore diff main..feature --pr-summary
ontocore diff --format pr-summary HEAD..WORKTREE
```

## Directory compare (no git)

```bash
ontocore diff --left-ref ./baseline --right-ref ./candidate --format json
```

## Indexed catalog (LSP / VS Code only)

To compare the **indexed in-memory catalog** against `WORKTREE`, use LSP `ontocore/semanticDiff` with `INDEXED` or `CATALOG` (legacy alias `WORKSPACE`). The CLI does not resolve indexed-catalog refs — use `WORKTREE` or directory paths.

## From a git clone

```bash
cargo run --example semantic_diff -p ontocode
```

## Related

- [CLI reference — diff](../cli-reference.md)
- [LSP API — semanticDiff](../lsp-api.md)
