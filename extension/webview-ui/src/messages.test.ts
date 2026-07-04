import { describe, it, expect } from "vitest";
import { isDiffPayload, isHostMessage } from "./messages";

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
    expect(isDiffPayload(validDiff)).toBe(true);
  });

  it("rejects malformed diff payloads", () => {
    expect(isDiffPayload(null)).toBe(false);
    expect(isDiffPayload({})).toBe(false);
    expect(isDiffPayload({ ...validDiff, entity_changes: "bad" })).toBe(false);
  });
});

describe("isHostMessage", () => {
  it("accepts host messages", () => {
    expect(isHostMessage({ type: "init", panel: "smoke" })).toBe(true);
    expect(isHostMessage({ type: "loading" })).toBe(true);
    expect(isHostMessage({ type: "semanticDiffData", diff: validDiff })).toBe(true);
    expect(
      isHostMessage({
        type: "loadEntity",
        detail: {
          entity: {
            iri: "http://ex#C",
            short_name: "C",
            kind: "class",
            labels: [],
            comments: [],
            deprecated: false,
          },
          parents: [],
          children: [],
          axioms: [],
          editable: true,
        },
        classOptions: [],
      })
    ).toBe(true);
  });

  it("rejects invalid payloads", () => {
    expect(isHostMessage(null)).toBe(false);
    expect(isHostMessage({})).toBe(false);
    expect(isHostMessage({ type: "semanticDiffData", diff: {} })).toBe(false);
    expect(isHostMessage({ type: "error" })).toBe(false);
  });
});
