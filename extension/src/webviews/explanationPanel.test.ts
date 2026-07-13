import assert from "node:assert/strict";
import { describe, it, beforeEach } from "node:test";
import { focusRelay } from "../focus/focusRelay";

describe("explanationPanel catalog fingerprint", () => {
  beforeEach(() => {
    focusRelay.resetForTests();
  });

  it("panel-owned fingerprint updates after explanation content (#219)", () => {
    focusRelay.setCatalogFingerprint({ indexedAt: 1, contentHash: "old" });
    focusRelay.setCatalogFingerprint({
      indexedAt: 2,
      contentHash: "fresh",
    });
    const fp = focusRelay.getCatalogFingerprint();
    assert.equal(fp?.indexedAt, 2);
    assert.equal(fp?.contentHash, "fresh");
  });

  it("reindex clears stale content hash (#219)", () => {
    focusRelay.setCatalogFingerprint({ indexedAt: 1, contentHash: "old" });
    focusRelay.setCatalogFingerprint({ indexedAt: 2 });
    const fp = focusRelay.getCatalogFingerprint();
    assert.equal(fp?.indexedAt, 2);
    assert.equal(fp?.contentHash, undefined);
  });
});
