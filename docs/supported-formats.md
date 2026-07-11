# Supported ontology formats (v0.17)

This page is the canonical reference for **what OntoCode/OntoCore can do with each file format** today.

If you are deciding whether to adopt OntoCode, cross-check this page with **[What ships today](SHIPPED.md)** and **[Known limitations](known-limitations.md)**.

## Quick summary

- **Write-back (edit in OntoCode / patch)**: **Turtle (`.ttl`)** and **OBO (`.obo`)**
- **Read-only (index/query/browse)**: OWL/RDF serializations including **OWL/XML** (`.owl`, `.owx`) and **RDF/XML**

## Capability matrix

| Format | File extensions | Index & browse | Query (SQL/SPARQL) | Entity Inspector edit (write-back) | Patch JSON write-back |
|--------|------------------|----------------|--------------------|-------------------------------------|------------------------|
| Turtle | `.ttl` | Yes | Yes | **Yes** | **Yes** |
| OBO | `.obo` | Yes | Yes (via indexed catalog) | **Yes** (v0.13+) | **Yes** (v0.12+) |
| OWL/XML | `.owl`, `.owx` | Yes | Yes | No (read-only) | No |
| RDF/XML | `.rdf`, `.xml` | Yes | Yes | No (read-only) | No |
| JSON-LD | `.jsonld` | Yes | Yes | No (read-only) | No |
| N-Triples | `.nt` | Yes | Yes | No (read-only) | No |
| N-Quads | `.nq` | Yes | Yes | No (read-only) | No |
| TriG | `.trig` | Yes | Yes | No (read-only) | No |

## Why some formats are read-only

OntoCode focuses write-back on formats where it can safely round-trip edits with predictable diffs today:

- **Turtle** is the primary write-back target.
- **OBO** write-back is supported for common OBO term workflows.

For OWL/RDF XML and JSON-LD, OntoCode can index, browse, and query, but does not yet guarantee safe in-place write-back.

See also:

- [First success tutorial](guides/first-success.md)
- [Authoring guide](authoring.md)
- [OBO authoring](ontocode/obo-authoring.md)
- [OWL/XML workflow](guides/owl-xml-workflow.md)
- [Patch reference](patch-reference.md)

