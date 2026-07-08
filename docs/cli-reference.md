# CLI reference (OntoCore v0.14)

The `ontocore` binary indexes ontology workspaces and exposes query, validation, patch, and reasoning commands.

Install:

```bash
cargo install ontocore-cli --locked
```

From a git clone, use `cargo run --` instead of `ontocore`.

## Global output formats

Several commands accept `--format text|json|csv` (where noted). Default is `text`.

## Commands

### `index`

Scan and index ontology files in a workspace. Prints catalog statistics only — use for CI scripts and machine-readable output.

```bash
ontocore index [workspace]
ontocore index ./ontologies --format json
```

Default workspace: `.` (current directory).

**Exit:** 0 on success; non-zero on index failure.

### `inspect`

Index the workspace and print catalog statistics **plus a diagnostic summary** (counts and up to 10 sample messages). Use for a quick human health check; run `validate` for full diagnostic listing and CI gating.

```bash
ontocore inspect fixtures
ontocore inspect /path/to/ontologies --format json
```

**Expected output (text, `fixtures/`):** ontology/class/property counts (e.g. multiple classes including `Person`).

### `query`

Run a SQL-like query against virtual tables. See [SQL reference](sql-reference.md).

```bash
ontocore query fixtures "SELECT short_name, labels FROM classes"
ontocore query . "SELECT code, message FROM diagnostics WHERE severity = 'error'" --format json
```

**Expected output (text, `fixtures/`):** tab-separated columns plus rows (e.g. `Person` in `short_name`).

**Exit:** 0 on success; non-zero on parse/unsupported SQL/I/O errors.

### `sparql`

Run SPARQL against indexed triples. See [SPARQL reference](sparql-reference.md).

```bash
ontocore sparql fixtures "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10"
```

**Exit:** 0 on success; non-zero on failure. Results truncate at 100,000 rows.

### `validate`

Index workspace and fail if diagnostic **errors** exist (warnings allowed).

```bash
ontocore validate .
ontocore validate /path/to/ontologies
```

**Expected output (text, clean workspace):** diagnostic summary with **exit code 0** (warnings allowed).

**Exit:** 0 when no errors; non-zero otherwise. Stable for CI — [ci-integration.md](ci-integration.md).

### `patch`

Apply **Turtle (`.ttl`) and OBO (`.obo`)** patch operations from a JSON file. See [patch reference](patch-reference.md) and [patch examples](examples/patches.md).
For a one-page format matrix (index/query vs write-back), see [Supported formats](supported-formats.md).

```bash
ontocore patch ./ontology.ttl patches.json --preview
ontocore patch ./ontology.ttl patches.json
```

**Exit:** 0 on success; non-zero on invalid patch or unsupported format.

### `robot`

Run [ROBOT](http://robot.obolibrary.org/) CLI subcommands. Requires Java and `robot` on `PATH` (or `--robot-path`). See [ROBOT interop guide](guides/robot-interop.md).

```bash
ontocore robot validate path/to/ontology.owl
ontocore robot merge --inputs a.owl --inputs b.owl --output merged.owl
ontocore robot report path/to/ontology.owl --report report.tsv
```

| Subcommand | Description |
|------------|-------------|
| `validate` | `robot validate` on a file |
| `merge` | Merge multiple ontology files |
| `report` | Generate a ROBOT report |

Optional `--robot-path` overrides the `robot` executable (same as VS Code `ontocode.robotPath`).

**Exit:** matches ROBOT exit code (0 on success).

### `classify`

Run OWL classification via OntoLogos 1.0.0.

```bash
ontocore classify ./ontologies --profile el
ontocore classify . --profile rl --format json
ontocore classify . --profile el --no-auto-profile
```

**Expected output (json):** `consistent: true`, `unsatisfiable: []`, `profile_used`, `duration_ms` when ontology is consistent.

| Flag | Default | Description |
|------|---------|-------------|
| `--profile` | `el` | `el`, `rl`, `rdfs`, `dl`, `auto` (OntoLogos 1.0) |
| `--auto-profile` | `true` | Emit profile-detection warnings |
| `--format` | `text` | `text` or `json` |

**Exit:** 0 when consistent (no unsatisfiable classes); non-zero on unsatisfiable classes or reasoner error.

JSON includes: `profile_used`, `consistent`, `unsatisfiable`, `warnings`, `duration_ms`, inferred hierarchy fields.

See [Reasoner guide](guides/reasoner.md) and [workspace-limits.md](workspace-limits.md).

### `explain`

Explain unsatisfiability for a class IRI (requires OntoLogos explain support).

```bash
ontocore explain ./ontologies --class 'http://example.org#Invalid' --profile el
ontocore explain . --class 'http://example.org#Invalid' --format json
```

| Flag | Default | Description |
|------|---------|-------------|
| `--class` | *(required)* | Class IRI |
| `--profile` | `el` | Reasoner profile |
| `--format` | `text` | `text` or `json` |

**Exit:** 0 when explanation produced; non-zero if class not found or explanation unavailable.

### `refactor`

Workspace-wide Turtle refactoring. See [Refactoring guide](guides/refactoring.md).

#### `refactor usages`

List usages of an entity IRI across the workspace.

```bash
ontocore refactor usages fixtures 'http://example.org/people#Person'
ontocore refactor usages . 'http://example.org/people#Person' --format json
```

**Exit:** 0 on success; non-zero on index failure.

#### `refactor rename`

Rename an entity IRI in all Turtle files.

```bash
ontocore refactor rename fixtures \
  --from 'http://example.org/people#Person' \
  --to 'http://example.org/people#Human' \
  --preview

ontocore refactor rename fixtures \
  --from 'http://example.org/people#Person' \
  --to 'http://example.org/people#Human'
```

| Flag | Description |
|------|-------------|
| `--from` | Current entity IRI (required) |
| `--to` | New entity IRI (required) |
| `--preview` | Print plan without writing files |
| `--format` | `text` (default) or `json` |

#### `refactor migrate-namespace`

Replace a namespace base IRI across Turtle files (`@prefix` and term IRIs).

```bash
ontocore refactor migrate-namespace fixtures \
  --from 'http://example.org/people#' \
  --to 'http://example.org/v2/people#' \
  --preview
```

| Flag | Description |
|------|-------------|
| `--from` | Current namespace base (required) |
| `--to` | New namespace base (required) |
| `--preview` | Print plan without writing |
| `--format` | `text` or `json` |

#### `refactor move`

Move an entity block to another Turtle file.

```bash
ontocore refactor move fixtures 'http://example.org/people#Student' \
  --to ./students.ttl \
  --preview
```

| Flag | Description |
|------|-------------|
| `--to` | Target `.ttl` path (required) |
| `--preview` | Print plan without writing |
| `--format` | `text` or `json` |

#### `refactor extract`

Extract selected entities into a new module file.

```bash
ontocore refactor extract fixtures \
  --entities 'http://example.org/people#Person,http://example.org/people#Student' \
  --out ./core.ttl \
  --leave-stub \
  --preview
```

| Flag | Description |
|------|-------------|
| `--entities` | Comma-separated entity IRIs (required) |
| `--out` | Output `.ttl` path (required) |
| `--leave-stub` | Leave import stubs in source files |
| `--preview` | Print plan without writing |
| `--format` | `text` or `json` |

**Exit (rename / migrate / move / extract):** 0 on success; non-zero on invalid request, path jail violation, or I/O failure. With `--preview`, files are not written.

### `diff`

Semantic catalog diff between git refs, directories, or indexed snapshots. See [Semantic diff guide](ontocode/semantic-diff.md).

```bash
ontocore diff HEAD..WORKTREE
ontocore diff --left-ref main --right-ref feature --format markdown
ontocore diff --left-ref HEAD --right-ref WORKTREE --breaking-only
ontocore diff --left-ref ./baseline --right-ref ./candidate
ontocore diff main..feature --pr-summary
```

| Flag | Description |
|------|-------------|
| `GIT_RANGE` | Optional positional range (`main..feature`) |
| `--left-ref` | Left git ref, directory path, or `WORKTREE` |
| `--right-ref` | Right git ref, directory path, or `WORKTREE` |
| `--repo` | Git repository root (default: current directory) |
| `--reasoner` | Include reasoner unsatisfiability changes |
| `--format` | `text` (default), `json`, `markdown`, or `pr-summary` (PR-ready Markdown, v0.13+) |
| `--pr-summary` | Shorthand for `--format pr-summary` |
| `--breaking-only` | Filter to likely breaking changes |

**Ref tokens (CLI vs LSP):** The CLI compares **git refs** and **directories on disk** only. Use `WORKTREE` for the working-tree catalog built from disk. To compare against the **indexed in-memory catalog** (LSP / VS Code), use `ontocore/semanticDiff` with `INDEXED` or `CATALOG` (legacy alias: `WORKSPACE`). Passing `WORKSPACE` / `INDEXED` / `CATALOG` to the CLI returns an error directing you to the LSP or `WORKTREE`.

**Exit:** 0 on success; non-zero on git/parse/I/O errors.

**Troubleshooting:** If you see `provide --left-ref or a git range`, pass a range or both refs. Do **not** use `--left` / `--right` — those flags do not exist.

### `docs`

Export Markdown or HTML documentation from an indexed workspace. See [Documentation export guide](guides/docs-export.md).

```bash
ontocore docs ./fixtures --format markdown --output /tmp/onto-docs
ontocore docs . --format html --output ./docs-out \
  --ontology-id http://example.org/people
```

| Flag | Default | Description |
|------|---------|-------------|
| `workspace` | `.` | Workspace directory to index |
| `--output` / `-o` | *(required)* | Output directory |
| `--format` | `markdown` | `markdown` or `html` |
| `--ontology-id` | — | Limit export to one ontology IRI or document id |
| `--plugin` | — | Exporter plugin id (e.g. `ontocode.markdown-export`); omit for built-in docs export |

Markdown `index.md` includes **Class hierarchy** and **Property index** sections (v0.13+).

**Exit:** 0 on success; non-zero on index, plugin export, or I/O failure.

### `plugins`

Discover and run workspace plugins from `.ontocore/plugins/*.toml`. See [Plugin authoring guide](guides/plugins.md).

#### `plugins list`

```bash
ontocore plugins list [workspace]
ontocore plugins list . --format json
```

**Result (text):** one line per plugin (`id`, `kind`, `version`). **Result (json):** array of plugin descriptors.

**Exit:** 0 on success; non-zero on discovery/host failure.

#### `plugins run`

```bash
ontocore plugins run <plugin_id> [--action validate|export|workflow] [--step <name>] [workspace]
ontocore plugins run ontocode.naming-validator --action validate .
```

| Flag | Default | Description |
|------|---------|-------------|
| `plugin_id` | *(required)* | Plugin id from manifest |
| `--action` | `validate` | `validate`, `export`, or `workflow` |
| `--step` | — | Workflow step when `--action workflow` |
| `--format` | `text` | `text` or `json` |

**Exit:** 0 on success; non-zero on host/action failure or plugin-reported failure.

### `workflow`

Run an external workflow plugin subprocess (e.g. owlmake scaffold).

```bash
ontocore workflow --plugin owlmake --step qc [workspace]
```

| Flag | Default | Description |
|------|---------|-------------|
| `--plugin` | *(required)* | Plugin id from `.ontocore/plugins/` |
| `--step` | `qc` | Workflow step: `build`, `qc`, `release`, or `report` |
| `workspace` | `.` | Workspace directory |

**Exit:** 0 when the workflow reports success; non-zero when the plugin fails or subprocess exits unsuccessfully.

## Related

- [Plugin authoring guide](guides/plugins.md)
- [Refactoring guide](guides/refactoring.md)
- [Examples: refactoring](examples/refactoring.md)
- [Getting started](getting-started.md)
- [CI integration](ci-integration.md)
- [Errors reference](errors.md)
- [What ships today](SHIPPED.md)
