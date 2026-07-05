import assert from "node:assert/strict";
import { describe, it } from "node:test";
import type { Entity, OntologyDocument } from "../lsp/protocol";
import {
  entityBelongsToDocument,
  MISSING_ONTOLOGY_HEADER_MESSAGE,
  resolveOntologyIri,
} from "./importsOntology";

function doc(overrides: Partial<OntologyDocument> = {}): OntologyDocument {
  return {
    id: "doc-1",
    path: "/ws/example.ttl",
    format: "turtle",
    base_iri: "http://example.org/foo#",
    imports: [],
    parse_status: "ok",
    ...overrides,
  };
}

function entity(overrides: Partial<Entity> = {}): Entity {
  return {
    iri: "http://example.org/foo#Bar",
    short_name: "Bar",
    kind: "class",
    ontology_id: "http://example.org/foo#",
    labels: [],
    comments: [],
    deprecated: false,
    ...overrides,
  };
}

describe("resolveOntologyIri", () => {
  it("returns owl:Ontology subject IRI when declared", () => {
    const document = doc({
      path: "/ws/people.ttl",
      base_iri: "http://example.org/people",
    });
    const entities = [
      entity({
        iri: "http://example.org/people",
        kind: "ontology",
        short_name: "people",
        ontology_id: "http://example.org/people",
      }),
      entity({
        iri: "http://example.org/people#Person",
        kind: "class",
        ontology_id: "http://example.org/people",
      }),
    ];
    assert.equal(resolveOntologyIri(document, entities), "http://example.org/people");
  });

  it("returns undefined when file lacks owl:Ontology header", () => {
    const document = doc({ base_iri: "http://example.org/foo#" });
    const entities = [
      entity({
        iri: "http://example.org/foo#Bar",
        kind: "class",
        ontology_id: "http://example.org/foo#",
      }),
    ];
    assert.equal(resolveOntologyIri(document, entities), undefined);
  });

  it("does not fall back to base_iri namespace fallback", () => {
    const document = doc({ base_iri: "http://example.org/foo#" });
    assert.equal(resolveOntologyIri(document, []), undefined);
  });
});

describe("entityBelongsToDocument", () => {
  it("matches by normalized ontology base IRI", () => {
    const document = doc({ base_iri: "http://example.org/people" });
    const ont = entity({
      iri: "http://example.org/people",
      kind: "ontology",
      ontology_id: "http://example.org/people",
    });
    assert.equal(entityBelongsToDocument(ont, document), true);
  });
});

describe("MISSING_ONTOLOGY_HEADER_MESSAGE", () => {
  it("is non-empty guidance", () => {
    assert.match(MISSING_ONTOLOGY_HEADER_MESSAGE, /owl:Ontology/);
  });
});
