import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { entityKindLabel, shortLabel } from "../utils/iri";

describe("shortLabel", () => {
  it("extracts local name after hash", () => {
    assert.equal(shortLabel("http://example.org/people#Person"), "Person");
  });

  it("extracts local name after slash", () => {
    assert.equal(shortLabel("http://example.org/people/Person"), "Person");
  });

  it("returns full iri when no delimiter", () => {
    assert.equal(shortLabel("Person"), "Person");
  });
});

describe("entityKindLabel", () => {
  it("maps known kinds", () => {
    assert.equal(entityKindLabel("class"), "Class");
    assert.equal(entityKindLabel("object_property"), "Object Property");
    assert.equal(entityKindLabel("individual"), "Individual");
  });

  it("returns unknown kinds unchanged", () => {
    assert.equal(entityKindLabel("custom_kind"), "custom_kind");
  });
});
