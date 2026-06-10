# ADR-0002 — Use Horned-OWL for OWL Modeling

## Status
Proposed

## Context
OntoIndex needs OWL 2 ontology manipulation and should avoid inventing its own OWL model.

## Decision
Use Horned-OWL as the primary OWL abstraction layer where possible.

## Consequences
This reduces risk around OWL semantics but may require adapters to normalize data into OntoIndex tables.