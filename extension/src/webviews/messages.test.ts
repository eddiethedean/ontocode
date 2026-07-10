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
      isWebviewMessage({
        type: "applyPatch",
        patches: [{ op: "add_label", entity_iri: "http://ex/P", value: "X" }],
        previewOnly: false,
      }),
      true
    );
    assert.equal(isWebviewMessage({ type: "copyMarkdown" }), true);
  });

  it("rejects applyPatch with empty patches", () => {
    assert.equal(
      isWebviewMessage({ type: "applyPatch", patches: [], previewOnly: false }),
      false
    );
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

  it("rejects patches missing entity_iri when expected", () => {
    assert.equal(
      parseApplyPatchMessage(
        {
          type: "applyPatch",
          patches: [{ op: "add_label", value: "X" }],
          previewOnly: false,
        },
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

  it("accepts OBO patches when term_id matches expectedOboId", () => {
    const oboPatch = { op: "set_name", term_id: "EX:001", value: "renamed" };
    const parsed = parseApplyPatchMessage(
      { type: "applyPatch", patches: [oboPatch], previewOnly: false },
      "http://example.org/EX_001",
      "EX:001"
    );
    assert.ok(parsed);
    assert.equal(parsed?.patches[0]?.term_id, "EX:001");
  });

  it("rejects OBO patches when term_id does not match expectedOboId", () => {
    const oboPatch = { op: "set_name", term_id: "EX:002", value: "renamed" };
    const parsed = parseApplyPatchMessage(
      { type: "applyPatch", patches: [oboPatch], previewOnly: false },
      "http://example.org/EX_001",
      "EX:001"
    );
    assert.equal(parsed, null);
  });

  it("rejects OBO patches in entity context without expectedOboId", () => {
    const oboPatch = { op: "add_synonym", term_id: "EX:001", value: "alias", scope: "EXACT" };
    const parsed = parseApplyPatchMessage(
      { type: "applyPatch", patches: [oboPatch], previewOnly: false },
      entity
    );
    assert.equal(parsed, null);
  });

  it("accepts OBO patches without entity context when no expected ids", () => {
    const oboPatch = { op: "set_name", term_id: "EX:001", value: "renamed" };
    assert.equal(
      isWebviewMessage({
        type: "applyPatch",
        patches: [oboPatch],
        previewOnly: true,
      }),
      true
    );
    const parsed = parseApplyPatchMessage(
      { type: "applyPatch", patches: [oboPatch], previewOnly: true },
      undefined
    );
    assert.ok(parsed);
  });

  it("accepts import patches without entity context", () => {
    const importPatch = {
      op: "add_import",
      ontology_iri: "http://example.org/ont",
      import_iri: "http://example.org/other",
    };
    const parsed = parseApplyPatchMessage(
      { type: "applyPatch", patches: [importPatch], previewOnly: false },
      undefined
    );
    assert.ok(parsed);
    assert.equal(parsed?.patches[0]?.op, "add_import");
  });
});
