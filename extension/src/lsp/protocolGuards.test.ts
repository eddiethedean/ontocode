import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { fixtureCatalogSnapshot } from "../test/fixtureSnapshot";
import {
  isCatalogSnapshot,
  isIndexWorkspaceResult,
  isDiffPayload,
  assertSemanticDiffResult,
} from "../lsp/protocolGuards";

describe("isCatalogSnapshot", () => {
  it("accepts a well-formed snapshot", () => {
    assert.equal(
      isCatalogSnapshot({
        documents: [],
        entities: [],
        hierarchy: { edges: [], parents: {}, children: {} },
      }),
      true
    );
  });

  it("rejects missing hierarchy", () => {
    assert.equal(
      isCatalogSnapshot({
        documents: [],
        entities: [],
      }),
      false
    );
  });

  it("accepts fixture snapshot wire format", () => {
    assert.equal(isCatalogSnapshot(fixtureCatalogSnapshot), true);
  });

  it("rejects PascalCase entity kinds", () => {
    assert.equal(
      isCatalogSnapshot({
        ...fixtureCatalogSnapshot,
        entities: [{ ...fixtureCatalogSnapshot.entities[0]!, kind: "Class" }],
      }),
      false
    );
  });
});

describe("isIndexWorkspaceResult", () => {
  it("accepts index workspace payloads", () => {
    assert.equal(
      isIndexWorkspaceResult({
        stats: { class_count: 2, error_count: 0 },
        indexed_at: 1,
      }),
      true
    );
  });

  it("rejects payloads without stats", () => {
    assert.equal(isIndexWorkspaceResult({ indexed_at: 1 }), false);
  });
});

const validDiff = {
  entity_changes: [{ kind: "added", iri: "http://ex#C" }],
  axiom_changes: [
    {
      change: "added",
      subject: "http://ex#C",
      predicate: "rdfs:subClassOf",
      object: "owl:Thing",
      axiom_kind: "sub_class_of",
    },
  ],
  annotation_changes: [],
  import_changes: [],
  inference_changes: [],
  breaking_changes: [],
};

describe("isDiffPayload", () => {
  it("accepts well-formed diff payloads", () => {
    assert.equal(isDiffPayload(validDiff), true);
  });

  it("rejects malformed diff payloads", () => {
    assert.equal(isDiffPayload(null), false);
    assert.equal(isDiffPayload({}), false);
  });
});

describe("assertSemanticDiffResult", () => {
  it("accepts wrapped diff responses", () => {
    const result = assertSemanticDiffResult({ diff: validDiff });
    assert.equal(result.diff.entity_changes.length, 1);
  });

  it("accepts flattened diff responses", () => {
    const result = assertSemanticDiffResult(validDiff);
    assert.equal(result.diff.axiom_changes.length, 1);
  });

  it("rejects invalid responses", () => {
    assert.throws(() => assertSemanticDiffResult(null));
    assert.throws(() => assertSemanticDiffResult({ diff: {} }));
  });
});
