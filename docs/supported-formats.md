# Supported ontology formats

> **Latest tagged release: v0.22.0** — canonical capability matrix: [What ships today](SHIPPED.md).

This page is the canonical reference for **what OntoCode/OntoCore can do with each file format** today.

## Quick summary

- **Write-back (edit in OntoCode / patch)**: **Turtle (`.ttl`)**, **OBO (`.obo`)**, **RDF/XML (`.owl` / `.rdf`)**, **OWL/XML (`.owx`)**
- **Read-only (index/query/browse)**: **JSON-LD**, **N-Triples**, **N-Quads**, **TriG**

## Capability matrix

| Format | File extensions | Index & browse | Query (SQL/SPARQL) | Entity Inspector edit (write-back) | Patch JSON write-back |
|--------|------------------|----------------|--------------------|-------------------------------------|------------------------|
| Turtle | `.ttl` | Yes | Yes | **Yes** | **Yes** |
| OBO | `.obo` | Yes | Yes (via indexed catalog) | **Yes** (v0.13+) | **Yes** (v0.12+) |
| RDF/XML | `.rdf`, `.owl` | Yes | Yes | **Yes** (v0.21+) | **Yes** (v0.21+) |
| OWL/XML | `.owx` | Yes | Yes | **Yes** (v0.21+) | **Yes** (v0.21+) |
| JSON-LD | `.jsonld` | Yes | Yes | No (read-only) | No |
| N-Triples | `.nt` | Yes | Yes | No (read-only) | No |
| N-Quads | `.nq` | Yes | Yes | No (read-only) | No |
| TriG | `.trig` | Yes | Yes | No (read-only) | No |

> **OBO versioning:** patch engine write-back since **v0.12**; Entity Inspector write-back since **v0.13**.  
> **XML write-back (v0.21):** Horned full-document re-serialize; semantic fidelity, not byte-identical formatting ([ADR-0021](design/adr/0021-deterministic-xml-serializers.md)).

## Why some formats remain read-only

OntoCode write-back focuses on formats with a safe round-trip path:

- **Turtle** — primary span-preserving write-back.
- **OBO** — stanza-preserving write-back for common term workflows.
- **RDF/XML / OWL/XML** — semantic re-serialize via Horned (v0.21).

JSON-LD and line-oriented RDF still lack write-back adapters.

See also:

- [First success tutorial](guides/first-success.md)
- [Capabilities by format](guides/capabilities-by-format.md)
- [Authoring guide](authoring.md)
- [OBO authoring](ontocode/obo-authoring.md)
- [OWL/XML workflow](guides/owl-xml-workflow.md)
- [Patch reference](patch-reference.md)
