# OBO authoring (v0.12+)

Edit **OBO Format 1.4** (`.obo`) files in VS Code via the Entity Inspector and `ontocore patch`. Requires OntoCode **v0.12.0+** or `ontocore-cli` **0.12.0+**.

**Related:** [OBO workflows](../guides/obo-workflow.md) · [Patch reference](../patch-reference.md) · [What ships today](../SHIPPED.md)

## Quick start

1. Open a workspace with `.obo` files and **Trust** the folder.
2. Select a term in the explorer or open a `.obo` file.
3. Open the **Entity Inspector** — edit name, synonyms, definition, `is_a` parent, namespace, or deprecated flag.
4. **Preview** then **Apply** (same flow as Turtle).

## Inspector vs raw file

| Approach | When to use |
|----------|-------------|
| **Entity Inspector** | Term metadata, parents, synonyms — guided forms |
| **Direct `.obo` edit** | Bulk edits, version control diffs, ROBOT pipelines |
| **`ontocore patch`** | CI, scripts, LSP integrators |

RDF/XML, OWL/XML, and JSON-LD remain **read-only** in the inspector.

## Patch operations

OBO patches use **`term_id`** (e.g. `GO:0008150`), not `entity_iri`. Ops are applied by `ontocore-obo`.

| `op` | Purpose |
|------|---------|
| `set_name` | Replace term name |
| `add_synonym` / `remove_synonym` | Synonyms with scope (`EXACT`, `RELATED`, …) |
| `add_def` / `remove_def` | Definition text |
| `add_xref` / `remove_xref` | Database cross-references |
| `set_namespace` | Term namespace |
| `set_deprecated` | `is_obsolete` flag |
| `add_is_a` / `remove_is_a` | Parent terms |

Full schema: [Patch reference — OBO operations](../patch-reference.md#obo-operations-obo).

## CLI example

```bash
ontocore patch ./terms.obo --preview '[
  {"op": "set_name", "term_id": "EX:001", "value": "example class"},
  {"op": "add_is_a", "term_id": "EX:001", "parent_id": "EX:000"}
]'
ontocore patch ./terms.obo patches.json
```

Copy-paste samples: [examples/patches.md](../examples/patches.md#obo-patches-v012)

## LSP

Method: `ontocore/applyAxiomPatch` with `document_uri` pointing at a `.obo` file. Same envelope as Turtle; ops use `term_id`. See [LSP API](../lsp-api.md#ontocoreapplyaxiompatch).

## Limits (v0.12)

- No Manchester or OWL axiom editing on OBO files — use Turtle/OWL for complex logical axioms.
- Logical definitions and some xref encodings may require Protégé or ROBOT for full parity.
- Mixed workspaces (`.obo` + `.ttl`) index together; write-back applies per file format.

## Troubleshooting

| Problem | Fix |
|---------|-----|
| Inspector read-only | Confirm v0.12+; entity must be in an indexed `.obo` file |
| `unsupported format` | Patch target must be `.obo` with OBO ops |
| Term not found | Check `term_id` matches the file (e.g. `GO:0008150`) |

See [Troubleshooting](../troubleshooting.md) and [OBO workflows](../guides/obo-workflow.md).
