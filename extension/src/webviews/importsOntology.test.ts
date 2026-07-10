import assert from "node:assert/strict";
import { describe, it } from "node:test";
import type { Entity, OntologyDocument } from "../lsp/protocol";
import {
  AMBIGUOUS_ONTOLOGY_HEADER_MESSAGE,
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

  it("prefers ontology matching document base_iri when multiple exist", () => {
    const document = doc({
      id: "doc-multi",
      base_iri: "http://example.org/primary",
    });
    const entities = [
      entity({
        iri: "http://example.org/aaa-secondary",
        kind: "ontology",
        short_name: "aaa-secondary",
        ontology_id: "doc-multi",
      }),
      entity({
        iri: "http://example.org/primary",
        kind: "ontology",
        short_name: "primary",
        ontology_id: "doc-multi",
      }),
    ];
    assert.equal(resolveOntologyIri(document, entities), "http://example.org/primary");
  });

  it("returns undefined when multiple ontologies and none match base_iri", () => {
    const document = doc({
      id: "doc-multi",
      base_iri: "http://example.org/unrelated",
    });
    const entities = [
      entity({
        iri: "http://example.org/aaa",
        kind: "ontology",
        short_name: "aaa",
        ontology_id: "doc-multi",
      }),
      entity({
        iri: "http://example.org/zzz",
        kind: "ontology",
        short_name: "zzz",
        ontology_id: "doc-multi",
      }),
    ];
    assert.equal(resolveOntologyIri(document, entities), undefined);
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
    assert.match(AMBIGUOUS_ONTOLOGY_HEADER_MESSAGE, /multiple/);
  });
});
