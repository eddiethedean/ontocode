import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { formatSemanticDiffMarkdown } from "./semanticDiffMarkdown";
import type { DiffPayload } from "./messages";

const sampleDiff: DiffPayload = {
  entity_changes: [{ kind: "added", iri: "http://ex#A" }],
  axiom_changes: [
    {
      change: "added",
      subject: "http://ex#Child",
      predicate: "rdfs:subClassOf",
      object: "http://ex#Parent",
      axiom_kind: "SubClassOf",
    },
  ],
  annotation_changes: [],
  import_changes: [{ change: "added", ontology_id: "o", import_iri: "http://ex#import" }],
  inference_changes: [],
  breaking_changes: [],
};

describe("formatSemanticDiffMarkdown", () => {
  it("includes entity axiom and import sections not only breaking", () => {
    const md = formatSemanticDiffMarkdown(sampleDiff);
    assert.match(md, /## Entity changes/);
    assert.match(md, /http:\/\/ex#A/);
    assert.match(md, /## Axiom changes/);
    assert.match(md, /SubClassOf/);
    assert.match(md, /## Import changes/);
    assert.doesNotMatch(md, /## Breaking changes/);
  });
});
