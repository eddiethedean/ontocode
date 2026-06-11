import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { resolveEntityIri } from "./resolveEntityIri";

describe("resolveEntityIri", () => {
  it("accepts IRI string from tree item command arguments", () => {
    assert.equal(
      resolveEntityIri("http://example.org/people#Person"),
      "http://example.org/people#Person"
    );
  });

  it("accepts tree item from view context menu", () => {
    assert.equal(
      resolveEntityIri({ iri: "http://example.org/people#Person", label: "Person" }),
      "http://example.org/people#Person"
    );
  });

  it("rejects empty and missing values", () => {
    assert.equal(resolveEntityIri(""), undefined);
    assert.equal(resolveEntityIri(undefined), undefined);
    assert.equal(resolveEntityIri({}), undefined);
  });
});
