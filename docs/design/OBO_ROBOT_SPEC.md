# OBO & ROBOT Interop Specification (v1.0)

> **Status:** target design (P0 for biomedical compete per [PROTEGE_PARITY.md](PROTEGE_PARITY.md))
>
> OntoCore does **not** reimplement ROBOT or ODK — it wraps the official ROBOT CLI today and integrates external workflow plugins (such as [owlmake](https://github.com/INCATools/owlmake)) for future Java-free pipelines.

## 1. Purpose

Enable biomedical ontology maintainers to use OntoCode as a **primary IDE** alongside standard OBO/ROBOT/ODK release pipelines.

## 2. Strategy summary

| Layer | What it does | Status |
|-------|----------------|--------|
| **`ontocore-robot`** | Thin wrapper around the official ROBOT Java CLI | **Shipped** (v0.7) |
| **OntoCore diagnostics** | Built-in lint and parse checks | **Shipped** |
| **owlmake** (external) | Rust-native portable ROBOT/ODK-style workflows | **Reference plugin** (v1.0 integration target) |
| **OntoCore core** | Does **not** reimplement ROBOT merge/template/report or ODK Makefile logic | By design |

OntoCore should integrate with the ontology toolchain, not absorb it. See [PLUGIN_SPEC.md](PLUGIN_SPEC.md).

## 3. OBO format (P0)

### Parser / writer

- Read and write OBO Format 1.4 (`.obo`) via [`fastobo`](https://crates.io/crates/fastobo) + [`fastobo-owl`](https://crates.io/crates/fastobo-owl) ([DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md))
- Validate with [`fastobo-validator`](https://crates.io/crates/fastobo-validator) where applicable
- Map OBO ids to IRIs in catalog (`obo_id`, `iri`, `namespace`)
- Support `synonymtypedef`, `property_value`, `xref` in catalog annotations table

### UI

- Explorer shows OBO shorthand ids where applicable
- Manchester / completion resolves OBO ids in biomedical workspaces
- Syntax highlighting for `.obo` files in VS Code

### Milestone

**v0.7b** — OBO format support before v1.0.

## 4. ROBOT interop (P0) — current path

Thin CLI wrappers in `ontocore-robot` crate (`ontocore robot` subcommand):

```bash
ontocore robot validate ./ontology
ontocore robot merge --inputs a.owl b.owl --output merged.owl
ontocore robot report ./ontology --report report.tsv
```

### Requirements

- Detect `robot` on `PATH`; clear error if missing with install link
- Pass through exit codes for CI
- Optional `ontocode.robotPath` setting (workspace-trusted, like `lspPath`)

### Documentation

- Side-by-side: OntoCode diagnostics vs ROBOT `validate` / `report`
- When to use OntoCode-only CI vs ROBOT-only vs both

## 5. owlmake — future external integration path

[owlmake](https://github.com/INCATools/owlmake) provides **Rust-native, portable** ROBOT/ODK-style workflow execution. It is a **reference external workflow plugin**, not a core OntoCore crate.

```text
OntoCode IDE
     │
     ▼
OntoCore (index, diagnostics, LSP)
     │
     ├── ontocore-robot ──► ROBOT CLI (Java)     ← shipped today
     │
     └── owlmake plugin ──► Rust-native workflows ← v1.0 integration target
```

### Integration goals (v1.0)

| Goal | Description |
|------|-------------|
| Import existing ODK projects | Recognize `src/ontology/`, catalog files, import structure |
| Run project QC | Execute QC steps; surface results as OntoCore diagnostics |
| Run release workflows | Trigger build/release pipelines from IDE or CLI |
| Inspect build outputs | Index and browse release artifacts in OntoCode |
| Surface workflow errors | Map owlmake/ROBOT failures to Problems panel diagnostics |

### Clarifications

- **`ontocore-robot`** wraps the current ROBOT CLI — this remains the supported path when Java + `robot` are available.
- **owlmake** may provide future **Java-free** workflow execution for teams that want portable Rust tooling.
- **OntoCore must not** reimplement all of ROBOT/ODK internally; plugins and CLI wrappers delegate to established semantics.
- OntoCode **surfaces** workflow actions; OntoCore **hosts** plugin APIs; owlmake **implements** workflow automation.

See [PLUGIN_SPEC.md](PLUGIN_SPEC.md) for `WorkflowPlugin` / `BuildPlugin` interfaces.

## 6. OBO/ODK project workflow goals (v1.0)

| Workflow | OntoCode / OntoCore role |
|----------|--------------------------|
| Open ODK repo | Index standard layout; show imports and modules in explorer |
| Edit source `.ttl` / `.obo` | Turtle write-back; OBO read-only in VS Code until v1.0 |
| Run QC | `ontocore validate` + ROBOT `report` + owlmake QC plugin |
| Run release | owlmake or ROBOT via plugin/CLI; inspect outputs in workspace |
| CI gates | `ontocore validate`, `ontocore classify`, `ontocore robot validate` |

## 7. Example workspace

`examples/obo-workflow/` — minimal mixed OBO + OWL repo demonstrating:

- Edit in OntoCode
- `ontocore validate` + `ontocore robot validate` in CI
- `ontocore robot merge` for release

## 8. Non-goals

- Reimplementing ROBOT merge/template/report logic inside OntoCore
- Replacing ODK Makefile or owlmake with built-in OntoCore code
- OBO Foundry automated compliance as a core feature (P1 validator plugin)

## 9. Parity tracking

See [PROTEGE_PARITY.md](PROTEGE_PARITY.md) — OBO & biomedical section.
