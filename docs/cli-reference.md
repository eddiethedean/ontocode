# CLI reference (OntoCore v0.26)

The `ontocore` binary indexes ontology workspaces and exposes query, validation, patch, and reasoning commands.

Install (pin latest tagged release):

```bash
cargo install ontocore-cli --locked --version 0.26.0
```

From a git clone, use `cargo run --` instead of `ontocore`.

## Commands at a glance

| Command | Purpose |
|---------|---------|
| `new` | Scaffold Turtle or OBO ontology file |
| `index` | Index workspace (catalog stats) |
| `inspect` | Index + diagnostic summary |
| `validate` | Run diagnostics (CI gate) |
| `query` | Catalog SQL |
| `sparql` | SPARQL over indexed triples |
| `patch` | Apply JSON patch ops |
| `classify` | Reasoner classification |
| `explain` | Unsatisfiability explanation |
| `realize` | Realize individuals |
| `check-instance` | Instance check |
| `dl-query` | Manchester DL Query |
| `diff` | Semantic diff |
| `docs` | Export Markdown/HTML docs |
| `refactor …` | Rename / merge / replace / move / extract / … |
| `robot …` | ROBOT CLI wrappers |
| `plugins …` | Plugin list/info/enable/disable/run |
| `workflow` | External workflow plugin step |

## Global output formats

Several commands accept `--format text|json|csv` (where noted). Default is `text`.

## Commands

### `new`

Create a new Turtle or OBO ontology file with `owl:Ontology` header metadata.

```bash
ontocore new ./people.ttl --ontology-iri 'http://example.org/people'
ontocore new ./terms.obo --ontology-iri 'http://purl.obolibrary.org/obo/demo.owl' --force
```

| Flag | Description |
|------|-------------|
| `path` | Output path (`.ttl` or `.obo`) |
| `--ontology-iri` | Ontology IRI (required) |
| `--version-iri` | Optional version IRI |
| `--force` | Overwrite existing file |

**Exit:** 0 on success; non-zero if the path exists (without `--force`) or IRI validation fails.

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

### `dl-query`

Run a Protégé-style DL Query (Manchester class expression). See [DL Query](guides/dl-query.md).

```bash
ontocore dl-query "Person and hasPet some Dog" fixtures --profile dl
ontocore dl-query "Person" . --mode asserted --format json
```

| Argument / flag | Default | Description |
|-----------------|---------|-------------|
| `expression` | *(required)* | Manchester class expression (prefer full IRIs) |
| `workspace` | `.` | Workspace directory |
| `--profile` | `dl` | Reasoner profile |
| `--mode` | `inferred` | `inferred` or `asserted` |
| `--format` | `text` | `text` or `json` |

**Exit:** 0 on success; non-zero on parse/reasoner error.

### `validate`

Index workspace and fail if diagnostic **errors** exist (warnings allowed).

```bash
ontocore validate .
ontocore validate /path/to/ontologies
```

**Expected output (text, clean workspace):** diagnostic summary with **exit code 0** (warnings allowed).

**Exit:** 0 when no errors; non-zero otherwise. Stable for CI — [ci-integration.md](ci-integration.md).

### `patch`

Apply **Turtle (`.ttl`), OBO (`.obo`), RDF/XML (`.owl`/`.rdf`), and OWL/XML (`.owx`)** patch operations from a JSON file. See [patch reference](patch-reference.md) and [patch examples](examples/patches.md).
For a one-page format matrix (index/query vs write-back), see [Supported formats](supported-formats.md). XML caveats: [OWL/XML write-back](guides/owl-xml-workflow.md).

```bash
ontocore patch ./ontology.ttl patches.json --preview
ontocore patch ./ontology.ttl patches.json
ontocore patch ./ontology.owl patches.json
```

**Exit:** 0 on success; non-zero on invalid patch or unsupported format/op.

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
```

**Expected output (json):** `consistent: true`, `unsatisfiable: []`, `profile_used`, `duration_ms` when ontology is consistent.

| Flag | Default | Description |
|------|---------|-------------|
| `--profile` | `el` | `el`, `rl`, `rdfs`, `dl`, `auto` (OntoLogos 1.0) |
| `--auto-profile` | enabled | Emit profile-detection warnings (default on; the current clap flag does not expose a separate disable switch) |
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

### `realize`

Realize individuals (inferred types) for a workspace (ABox). Default profile is `rl`.

```bash
ontocore realize ./ontologies --profile rl
ontocore realize fixtures --profile dl --format json
```

| Flag | Default | Description |
|------|---------|-------------|
| `--profile` | `rl` | `el`, `rl`, `rdfs`, `dl`, `auto` |
| `--format` | `text` | `text` or `json` |

**Expected output (text):** one line per individual with `types=[…]` and `most_specific=[…]`.

**Exit:** 0 on success; non-zero on reasoner error.

See [Reasoner guide](guides/reasoner.md) and [realize cookbook](examples/realize.md).

### `check-instance`

Check whether an individual is an instance of a class.

```bash
ontocore check-instance fixtures \
  --individual 'http://example.org/people#alice' \
  --class 'http://example.org/people#Person' \
  --profile rl

ontocore check-instance . \
  --individual 'http://example.org/people#alice' \
  --class 'http://example.org/people#Person' \
  --format json
```

| Flag | Default | Description |
|------|---------|-------------|
| `--individual` | *(required)* | Individual IRI |
| `--class` | *(required)* | Class IRI |
| `--profile` | `rl` | Reasoner profile |
| `--format` | `text` | `text` or `json` |

**Exit:** 0 when entailed; **non-zero** when not entailed or on reasoner error.

**LSP note:** Realization is a **CLI / CI** command today (`ontocore realize`). Instance checks are available via LSP `ontocore/checkInstance`. There is no LSP `realize` method — use the CLI in automation.

### SWRL (no CLI subcommand)

!!! note "No `ontocore swrl` command"
    Author and validate SWRL via **patches**, the IDE Rule Browser, and LSP (`ontocore/listSwrlRules`, `ontocore/validateSwrlRule`, `ontocore/parseSwrlRule`). When rules are present, classify may materialize SWRL consequents. See [SWRL examples](examples/swrl.md) and [patch SWRL ops](patch-reference.md).

### `refactor`

Workspace refactoring (rename / merge / replace across formats where remaps apply; move / extract / import ops are Turtle-first). See [Refactoring guide](guides/refactoring.md).

#### `refactor usages`

List usages of an entity IRI across the workspace.

```bash
ontocore refactor usages fixtures 'http://example.org/people#Person'
ontocore refactor usages . 'http://example.org/people#Person' --format json
```

**Exit:** 0 on success; non-zero on index failure.

#### `refactor rename`

Rename an entity IRI across indexed ontology files (Turtle + format remaps for RDF/XML, OWL/XML, OBO).

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

#### `refactor merge`

Merge one entity into another (rewrite references to the keep IRI; drop the merge declaration).

```bash
ontocore refactor merge fixtures \
  --keep 'http://example.org/people#Person' \
  --merge 'http://example.org/people#Human' \
  --preview
```

| Flag | Description |
|------|-------------|
| `--keep` | Survivor entity IRI (required) |
| `--merge` | Duplicate entity IRI to fold in (required) |
| `--preview` | Print plan without writing files |
| `--format` | `text` or `json` |

#### `refactor replace`

Replace references to one entity with another (keeps source declaration when the target already exists).

```bash
ontocore refactor replace fixtures \
  --from 'http://example.org/people#OldName' \
  --to 'http://example.org/people#NewName' \
  --preview
```

| Flag | Description |
|------|-------------|
| `--from` | Source entity IRI (required) |
| `--to` | Target entity IRI (required) |
| `--preview` | Print plan without writing files |
| `--format` | `text` or `json` |

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
  --locality \
  --preview
```

| Flag | Description |
|------|-------------|
| `--entities` | Comma-separated entity IRIs (required) |
| `--out` | Output `.ttl` path (required) |
| `--leave-stub` | Leave import stubs in source files |
| `--locality` | Close seed signature under bottom-locality heuristic before extract |
| `--preview` | Print plan without writing |
| `--format` | `text` or `json` |

#### `refactor merge-ontologies`

Merge one or more Turtle ontology files into a target Turtle file.

```bash
ontocore refactor merge-ontologies fixtures \
  --sources ./a.ttl --sources ./b.ttl \
  --target ./combined.ttl \
  --preview
```

#### `refactor flatten-imports` / `cleanup-imports`

Inline imported Turtle axioms (flatten) or remove unused `owl:imports` (cleanup heuristic).

```bash
ontocore refactor flatten-imports fixtures --file ./root.ttl --preview
ontocore refactor cleanup-imports fixtures --file ./root.ttl --preview
```

**Exit (rename / merge / replace / migrate / move / extract / ontology ops):** 0 on success; non-zero on invalid request, path jail violation, or I/O failure. With `--preview`, files are not written.

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

Discover, inspect, enable/disable, and run workspace plugins from `.ontocore/plugins/*.toml`. See [Plugin authoring guide](guides/plugins.md) (SDK 1.0).

#### `plugins list`

```bash
ontocore plugins list [workspace]
ontocore plugins list . --format json
```

**Result (text):** one line per plugin (`id`, `kind`, `version`). **Result (json):** array of plugin descriptors (includes lifecycle fields such as `state`, `enabled`, `depends_on`, `activation` when present).

**Exit:** 0 on success; non-zero on discovery/host failure.

#### `plugins info`

```bash
ontocore plugins info <plugin_id> [workspace]
ontocore plugins info ontocode.naming-validator . --format json
```

Shows lifecycle and dependency info: `state`, `activation`, `enabled`, `depends_on`, `manifest_path`.

**Exit:** 0 on success; non-zero if the plugin id is unknown or host discovery fails.

#### `plugins enable` / `plugins disable`

```bash
ontocore plugins enable <plugin_id> [workspace]
ontocore plugins disable <plugin_id> [workspace]
```

`enable` activates the plugin (and dependents per activation policy). `disable` cascade-deactivates dependents.

**Exit:** 0 on success; non-zero on host/policy failure.

#### `plugins run`

```bash
ontocore plugins run <plugin_id> [--action <name>] [--step <name>] [--query <text>] [--iri <iri>] [workspace]
ontocore plugins run ontocode.naming-validator --action validate .
ontocore plugins run my.query --action query.run --query 'SELECT short_name FROM classes' .
```

| Flag | Default | Description |
|------|---------|-------------|
| `plugin_id` | *(required)* | Plugin id from manifest |
| `--action` | `validate` | `validate`, `export`, `workflow`, `reasoner.classify`, `query.run`, `refactor.preview`, `graph.build` |
| `--step` | — | Workflow step when `--action workflow` |
| `--query` | — | Query text for `query.run` |
| `--iri` | — | Focus/root IRI for `refactor.preview` or `graph.build` |
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
- [Install CLI & CI (detail)](install-cli-ci.md)
- [CI integration](ci-integration.md)
- [Errors reference](errors.md)
- [What ships today](SHIPPED.md)
