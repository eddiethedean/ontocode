# ADR-0003 — Use Oxigraph for RDF and SPARQL Infrastructure

## Status
Proposed

## Context
OntoIndex needs RDF parsing, SPARQL support, and RDF graph primitives.

## Decision
Use Oxigraph as the primary RDF/SPARQL infrastructure.

## Consequences
OntoIndex avoids competing directly as a triplestore and instead builds ontology-aware indexing and analytics on top.