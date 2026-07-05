import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
  hasPatchFailureDiagnostics,
  patchFailureMessage,
} from "./patchFeedback";
import type { ApplyPatchResult } from "./protocol";

describe("patchFeedback", () => {
  it("uses first diagnostic message when present", () => {
    const result: ApplyPatchResult = {
      applied: false,
      diagnostics: [{ severity: "error", message: "entity not found" }],
    };
    assert.equal(patchFailureMessage(result), "entity not found");
    assert.equal(hasPatchFailureDiagnostics(result), true);
  });

  it("falls back when diagnostics are empty", () => {
    const result: ApplyPatchResult = { applied: false };
    assert.equal(
      patchFailureMessage(result),
      "OntoCode: patch was not applied"
    );
    assert.equal(hasPatchFailureDiagnostics(result), false);
  });
});
