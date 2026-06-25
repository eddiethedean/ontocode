import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { isWebviewMessage } from "./messages";

describe("isWebviewMessage", () => {
  it("accepts typed webview messages", () => {
    assert.equal(
      isWebviewMessage({ type: "ready", panel: "inspector" }),
      true
    );
    assert.equal(
      isWebviewMessage({ type: "applyPatch", patches: [], previewOnly: false }),
      true
    );
  });

  it("rejects invalid payloads", () => {
    assert.equal(isWebviewMessage(null), false);
    assert.equal(isWebviewMessage({}), false);
    assert.equal(isWebviewMessage({ type: 1 }), false);
  });
});
