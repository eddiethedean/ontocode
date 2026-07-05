# ADR-0019 — OBO Write-Back via fastobo

## Status
Accepted (v0.11 groundwork; full write-back targeted v1.0)

## Context

OntoCore indexes OBO Format 1.4 (`.obo`) for biomedical workflows. v0.7 shipped read-only OBO support with a minimal line parser. v1.0 Protégé parity requires reliable read/write, rich metadata (synonyms, definitions, xrefs), and patch-based editing consistent with Turtle write-back ([ADR-0006](0006-patch-based-write-back.md)).

The canonical Rust stack for OBO is [`fastobo`](https://crates.io/crates/fastobo) + [`fastobo-owl`](https://crates.io/crates/fastobo-owl) per [OBO_ROBOT_SPEC.md](../OBO_ROBOT_SPEC.md).

## Decision

### v0.11 (this release)

1. **Read path:** Replace the minimal OBO line parser with `fastobo::from_str` → existing `ParsedOntology` / catalog model. Surface synonyms, definitions, and property values in catalog annotations and entity detail.
2. **Write-back:** Document patch schema below; **do not** ship OBO inspector editing or `fastobo` serialize-to-disk in v0.11. Turtle remains the only editable format in VS Code.
3. **Boundaries:** Turtle patches (`ontocore-owl::patch`) continue to own Turtle write-back. OBO patches are a separate op namespace applied by a future `ontocore-obo` layer (v1.0).

### v1.0 (planned)

- `fastobo` + `fastobo-owl` for round-trip OBO read/write
- Inspector and LSP `applyAxiomPatch` on `.obo` files
- Optional `fastobo-validator` in CI / `ontocore validate`

## OBO patch op schema (v1.0 target)

JSON array; each object has `"op"` (snake_case). Term subject is always an OBO id (e.g. `GO:0008150`) or resolved IRI.

| `op` | Required fields | Description |
|------|-----------------|-------------|
| `set_name` | `term_id`, `value` | Replace `name` clause |
| `add_synonym` | `term_id`, `value`, `scope` | Add synonym (`EXACT`, `RELATED`, `BROAD`, `NARROW`) |
| `remove_synonym` | `term_id`, `value` | Remove matching synonym |
| `add_def` | `term_id`, `value` | Add or replace `def` |
| `remove_def` | `term_id` | Remove definition |
| `add_xref` | `term_id`, `xref` | Add xref (`DB:ID` or structured) |
| `remove_xref` | `term_id`, `xref` | Remove xref |
| `set_namespace` | `term_id`, `namespace` | Set term namespace |
| `set_deprecated` | `term_id`, `value` | Set `is_obsolete` |
| `add_is_a` | `term_id`, `parent_id` | Add `is_a` parent |
| `remove_is_a` | `term_id`, `parent_id` | Remove `is_a` parent |

Example:

```json
[
  {
    "op": "add_synonym",
    "term_id": "EX:001",
    "value": "example term",
    "scope": "EXACT"
  },
  {
    "op": "add_def",
    "term_id": "EX:001",
    "value": "An example class."
  }
]
```

## Turtle vs OBO write-back

| Concern | Turtle | OBO |
|---------|--------|-----|
| Patch crate | `ontocore-owl` | Future `ontocore-obo` (v1.0) |
| LSP apply | `ontocore/applyAxiomPatch` on `.ttl` | Same method, format dispatch (v1.0) |
| Imports | `owl:imports` patch ops (`add_import`, `remove_import`) | N/A (OBO uses separate files / ontology headers) |
| Manchester | Yes | No (OBO uses structural clauses) |

## Consequences

**Positive:**

- Single canonical OBO parser (`fastobo`) reduces drift from Protégé / ROBOT
- Patch schema is documented before v1.0 implementation
- v0.11 enriches explorer/inspector metadata without risky write path

**Negative:**

- Two patch vocabularies until v1.0 unifies LSP dispatch
- Golden snapshots may change when migrating parser (mitigated by CI golden tests)

## References

- [OBO_ROBOT_SPEC.md](../OBO_ROBOT_SPEC.md)
- [PROTEGE_PARITY.md](../PROTEGE_PARITY.md)
- [patch-reference.md](../../patch-reference.md)
