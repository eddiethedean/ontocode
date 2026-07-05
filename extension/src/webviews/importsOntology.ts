import type { Entity, OntologyDocument } from "../lsp/protocol";

export function normalizeOntologyIri(iri: string): string {
  return iri.replace(/[#/]+$/, "");
}

/** Mirror `document_matches_entity` in ontocore-core. */
export function entityBelongsToDocument(
  entity: Entity,
  doc: OntologyDocument
): boolean {
  if (entity.ontology_id === doc.id) {
    return true;
  }
  if (doc.base_iri) {
    if (normalizeOntologyIri(doc.base_iri) === normalizeOntologyIri(entity.ontology_id)) {
      return true;
    }
    if (entity.iri.startsWith(doc.base_iri)) {
      return true;
    }
  }
  return false;
}

/** Resolve the owl:Ontology subject IRI declared in a Turtle document. */
export function resolveOntologyIri(
  doc: OntologyDocument,
  entities: Entity[]
): string | undefined {
  const candidates = entities
    .filter((e) => e.kind === "ontology" && entityBelongsToDocument(e, doc))
    .sort((a, b) => a.iri.localeCompare(b.iri));
  return candidates[0]?.iri;
}

export const MISSING_ONTOLOGY_HEADER_MESSAGE =
  "This file has no owl:Ontology declaration. Add an ontology header before managing imports.";
