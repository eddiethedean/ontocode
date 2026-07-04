import { describe, it } from "node:test";
import assert from "node:assert/strict";
import { isWebviewMessage, parseApplyPatchMessage } from "./messages";

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
    assert.equal(isWebviewMessage({ type: "copyMarkdown" }), true);
  });

  it("rejects invalid payloads", () => {
    assert.equal(isWebviewMessage(null), false);
    assert.equal(isWebviewMessage({}), false);
    assert.equal(isWebviewMessage({ type: 1 }), false);
    assert.equal(isWebviewMessage({ type: "ready" }), false);
  });
});

describe("parseApplyPatchMessage", () => {
  const entity = "http://example.org/Person";
  const patch = { op: "add_label", entity_iri: entity, value: "X" };

  it("requires boolean previewOnly", () => {
    assert.equal(
      parseApplyPatchMessage(
        { type: "applyPatch", patches: [patch] } as never,
        entity
      ),
      null
    );
  });

  it("rejects patches for other entities", () => {
    assert.equal(
      parseApplyPatchMessage(
        {
          type: "applyPatch",
          patches: [{ op: "delete_entity", entity_iri: "http://other" }],
          previewOnly: false,
        },
        entity
      ),
      null
    );
  });

  it("accepts valid applyPatch", () => {
    const parsed = parseApplyPatchMessage(
      { type: "applyPatch", patches: [patch], previewOnly: true },
      entity
    );
    assert.ok(parsed);
    assert.equal(parsed?.previewOnly, true);
  });
});
