# Ontology concepts

Short glossary for engineers new to OWL/RDF who are evaluating OntoCode.

## Core terms

| Term | Meaning |
|------|---------|
| **IRI** | Internationalized Resource Identifier — the canonical ID for a class, property, or individual (e.g. `http://example.org/people#Person`) |
| **Turtle (`.ttl`)** | Human-readable RDF syntax; the only format OntoCode can write back today |
| **Class** | A category or type (e.g. `Person`, `Organization`) |
| **Object property** | A relationship between individuals (e.g. `hasParent`) |
| **Data property** | A relationship from an individual to a literal value (e.g. `hasAge`) |
| **Individual** | A named instance of one or more classes |
| **Axiom** | A logical statement, such as `SubClassOf` or `EquivalentClasses` |
| **Annotation** | Descriptive metadata (labels, comments) that does not affect reasoning |

## Manchester syntax

A text notation for OWL class expressions (e.g. `ex:hasRecord some ex:MedicalRecord`). OntoCode v0.7 supports an MVP subset via the Manchester editor — see [Manchester guide](guides/manchester-editor.md).

## Reasoning profiles

| Profile | Typical use |
|---------|-------------|
| **EL** | OWL EL ontologies (default in OntoCode v0.7) |
| **RL** | OWL RL materialization |
| **RDFS** | RDFS entailment |
| **DL** | Full OWL 2 DL — requires OntoLogos 1.0 (not shipped in v0.7) |

## Asserted vs inferred hierarchy

- **Asserted** — parent/child edges written explicitly in your `.ttl` files
- **Inferred** — additional subsumption edges computed by the reasoner
- **Combined** — asserted plus inferred edges in the explorer tree

Run **OntoCode: Run Reasoner** before switching to inferred or combined mode.

## OntoCode vs OntoIndex

- **OntoCode** — VS Code extension (UI)
- **OntoIndex** — Rust engine (CLI, LSP, crates)

## Next steps

- [First success tutorial](guides/first-success.md)
- [What ships today](SHIPPED.md)
- [Protégé coexistence](guides/protege-coexistence.md)
