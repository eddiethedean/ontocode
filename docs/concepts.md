# Ontology concepts

Short glossary for engineers new to OWL/RDF who are evaluating OntoCode.

## Core terms

| Term | Meaning |
|------|---------|
| **IRI** | Internationalized Resource Identifier — the canonical ID for a class, property, or individual (e.g. `http://example.org/people#Person`) |
| **Prefix** | Short name for a namespace in Turtle (`@prefix ex: <http://example.org/people#> .`) — lets you write `ex:Person` instead of the full IRI |
| **Turtle (`.ttl`)** | Human-readable RDF syntax; the only format OntoCode can write back today |
| **Class** | A category or type (e.g. `Person`, `Organization`) |
| **Object property** | A relationship between individuals (e.g. `hasParent`) |
| **Data property** | A relationship from an individual to a literal value (e.g. `hasAge`) |
| **Individual** | A named instance of one or more classes |
| **Axiom** | A logical statement, such as `SubClassOf` or `EquivalentClasses` |
| **Annotation** | Descriptive metadata (labels, comments) that does not affect reasoning |

## Minimal Turtle example

```turtle
@prefix ex: <http://example.org/people#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .

<http://example.org/people> a owl:Ontology .

ex:Person a owl:Class ;
    rdfs:label "Person" ;
    rdfs:subClassOf ex:Thing .

ex:Thing a owl:Class ;
    rdfs:label "Thing" .
```

- `ex:Person` is a **class** with label `"Person"` and parent `ex:Thing`
- `rdfs:subClassOf` is a simple **axiom** (Person is a subclass of Thing)
- OntoCode indexes this file and shows `Person` under **Classes** in the explorer

Download this pattern from [fixtures/example.ttl](https://github.com/eddiethedean/ontocode/blob/main/fixtures/example.ttl) or the [first success tutorial](guides/first-success.md).

## Manchester syntax

OntoCode supports subclass, equivalent, and disjoint axioms via the Manchester editor — see [Manchester editor](ontocode/manchester-editor.md).

## Reasoning profiles

| Profile | Typical use |
|---------|-------------|
| **EL** | OWL EL ontologies (default in OntoCode) |
| **RL** | OWL RL materialization |
| **RDFS** | RDFS entailment |
| **DL** | Full OWL 2 DL via OntoLogos 1.0 (`dl` profile) |

## Asserted vs inferred hierarchy

- **Asserted** — parent/child edges written explicitly in your `.ttl` files
- **Inferred** — additional subsumption edges computed by the reasoner
- **Combined** — asserted plus inferred edges in the explorer tree

Run **OntoCode: Run Reasoner** before switching to inferred or combined mode.

## OntoCode vs OntoCore

- **OntoCode** — VS Code IDE (UI)
- **OntoCore** — Rust semantic workspace engine (CLI, LSP, `ontocore` / `ontocore-*` crates)

## Next steps

- [First success tutorial](guides/first-success.md)
- [What ships today](SHIPPED.md)
- [Protégé coexistence](guides/protege-coexistence.md)
