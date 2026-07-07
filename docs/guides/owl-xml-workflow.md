# OWL/XML and RDF/XML workflow (read-only catalog)

> **Status:** Shipped in **v0.12.0** — browse and query in VS Code; **no write-back** to `.owl` or `.owx` files.

OntoCore indexes RDF/XML (`.owl`) and OWL/XML (`.owx`) via [Horned-OWL](https://crates.io/crates/horned-owl). The Entity Inspector shows entities from these files but marks them **read-only** (`editable: false`).

Canonical capability matrix: [What ships today](../SHIPPED.md).

## What works today

| Capability | `.owl` (RDF/XML) | `.owx` (OWL/XML) |
|------------|------------------|------------------|
| Workspace indexing | Yes | Yes |
| Explorer / SQL / SPARQL | Yes | Yes |
| Entity Inspector (view) | Yes | Yes |
| Entity Inspector (edit) | No | No |
| Patch / `ontocore patch` write-back | No | No |
| Manchester editor apply | No | No |
| Refactoring apply | No (Turtle only) | No |

## Recommended workflows

### Browse-only in place

Keep authoritative sources as `.owl` / `.owx` in the repo. Use OntoCode to browse, query, reason, and run semantic diff without editing those files.

### Edit via Turtle or OBO

1. Maintain editable copies as `.ttl` or `.obo` in the same workspace (or a sibling package).
2. Use `owl:imports` or catalog files so the index links related modules.
3. Apply patches and inspector edits on Turtle/OBO only — see [Authoring](../authoring.md).

### Round-trip with Protégé or ROBOT

For teams that must publish RDF/XML or OWL/XML:

1. Author in Turtle/OBO with OntoCode.
2. Export or convert with [ROBOT](robot-interop.md) or Protégé before release.
3. Re-index after conversion; treat XML as **release artifacts**, not the live edit surface.

Protégé coexistence patterns: [Protégé coexistence](protege-coexistence.md).

## VS Code behavior

1. Open a workspace containing `.owl` or `.owx` files.
2. **Trust** the workspace and run **OntoCode: Index Workspace**.
3. Select an entity — the inspector shows axioms but edit controls are disabled.
4. For edits, open the corresponding `.ttl` or `.obo` file, or create one and link via imports.

## CLI

```bash
ontocore validate /path/to/ontologies
ontocore query /path/to/ontologies "SELECT * FROM classes WHERE file_path LIKE '%.owl'"
ontocore classify /path/to/ontologies --profile el
```

Patch apply to `.owl` / `.owx` returns an **unsupported format** error — see [Errors reference](../errors.md).

## Migration from v0.11

v0.12 added Horned-OWL catalog support for OWL/XML and clarified RDF/XML read-only policy. See [Migration v0.11 → v0.12](../migration/v0.12.md).

## Related

| Topic | Guide |
|-------|-------|
| Format policy | [Authoring](../authoring.md#format-policy) |
| OBO write-back | [OBO authoring](../ontocode/obo-authoring.md) |
| Protégé gaps | [Protégé decision guide](protege-decision.md) |
