import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { acceptInspectorRevealRequest } from "./inspectorReveal";

describe("acceptInspectorRevealRequest", () => {
  it("accepts first navigation when activeRequestId is 0", () => {
    assert.equal(acceptInspectorRevealRequest(0, 1), true);
  });

  it("accepts newer navigation while inspector is open", () => {
    assert.equal(acceptInspectorRevealRequest(1, 2), true);
    assert.equal(acceptInspectorRevealRequest(3, 5), true);
  });

  it("rejects stale async reveal after a newer navigation", () => {
    assert.equal(acceptInspectorRevealRequest(5, 3), false);
    assert.equal(acceptInspectorRevealRequest(2, 1), false);
  });

  it("accepts reveal without requestId", () => {
    assert.equal(acceptInspectorRevealRequest(4, undefined), true);
  });
});
