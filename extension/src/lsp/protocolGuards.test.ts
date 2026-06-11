import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { fixtureCatalogSnapshot } from "../test/fixtureSnapshot";
import {
  isCatalogSnapshot,
  isIndexWorkspaceResult,
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
