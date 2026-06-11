# OBO & ROBOT Interop Specification (v1.0)

> **Status:** target design (P0 for biomedical compete per [PROTEGE_PARITY.md](PROTEGE_PARITY.md))
>
> OntoCode does **not** reimplement ROBOT — it wraps the official CLI.

## 1. Purpose

Enable biomedical ontology maintainers to use OntoCode as a **primary IDE** alongside standard OBO/ROBOT release pipelines.

## 2. OBO format (P0)

### Parser / writer

- Read and write OBO Format 1.4 (`.obo`)
- Map OBO ids to IRIs in catalog (`obo_id`, `iri`, `namespace`)
- Support `synonymtypedef`, `property_value`, `xref` in catalog annotations table

### UI

- Explorer shows OBO shorthand ids where applicable
- Manchester / completion resolves OBO ids in biomedical workspaces
- Syntax highlighting for `.obo` files in VS Code

### Milestone

**v0.7b** — OBO format support before v1.0.

## 3. ROBOT interop (P0)

Thin CLI wrappers in `ontoindex-robot` crate (or `ontoindex-cli` subcommand):

```bash
ontoindex robot validate ./ontology
ontoindex robot merge --inputs a.owl b.owl --output merged.owl
ontoindex robot report ./ontology --report report.tsv
```

### Requirements

- Detect `robot` on `PATH`; clear error if missing with install link
- Pass through exit codes for CI
- Optional `ontocode.robotPath` setting (workspace-trusted, like `lspPath`)

### Documentation

- Side-by-side: OntoCode diagnostics vs ROBOT `validate` / `report`
- When to use OntoCode-only CI vs ROBOT-only vs both

## 4. Example workspace

`examples/obo-workflow/` — minimal mixed OBO + OWL repo demonstrating:

- Edit in OntoCode
- `ontoindex validate` + `ontoindex robot validate` in CI
- `ontoindex robot merge` for release

## 5. Non-goals

- Reimplementing ROBOT merge/template/report logic
- ODK Makefile replacement (document interop only)
- OBO Foundry automated compliance (P1 plugin)

## 6. Parity tracking

See [PROTEGE_PARITY.md](PROTEGE_PARITY.md) — OBO & biomedical section.
