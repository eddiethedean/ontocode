import { describe, it, expect } from "vitest";
import { isHostMessage } from "./messages";

describe("isHostMessage", () => {
  it("accepts host messages", () => {
    expect(isHostMessage({ type: "init", panel: "smoke" })).toBe(true);
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
  });
});
