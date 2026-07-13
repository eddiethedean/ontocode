import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { isUsageJumpLineInDocument, usageJumpLineIndex } from "./refactorPreviewLogic";

describe("usage jump line guard", () => {
  it("converts 1-based usage lines to 0-based indices", () => {
    assert.equal(usageJumpLineIndex(1), 0);
    assert.equal(usageJumpLineIndex(5), 4);
    assert.equal(usageJumpLineIndex(undefined), 0);
  });

  it("rejects out-of-range lines before lineAt (#228)", () => {
    assert.equal(isUsageJumpLineInDocument(0, 3), true);
    assert.equal(isUsageJumpLineInDocument(2, 3), true);
    assert.equal(isUsageJumpLineInDocument(3, 3), false);
    assert.equal(isUsageJumpLineInDocument(99, 10), false);
  });
});
