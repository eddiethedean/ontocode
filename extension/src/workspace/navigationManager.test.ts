import { describe, it, beforeEach } from "node:test";
import assert from "node:assert/strict";
import { navigationManager } from "./navigationManager";

describe("navigationManager", () => {
  beforeEach(() => {
    navigationManager.resetForTests();
  });

  it("pushes entries and supports back/forward", () => {
    navigationManager.push({ kind: "entity", id: "http://ex/a", source: "test" });
    navigationManager.push({ kind: "entity", id: "http://ex/b", source: "test" });
    assert.equal(navigationManager.getIndex(), 1);
    assert.equal(navigationManager.canGoBack(), true);
    const back = navigationManager.back();
    assert.equal(back?.id, "http://ex/a");
    assert.equal(navigationManager.canGoForward(), true);
    const forward = navigationManager.forward();
    assert.equal(forward?.id, "http://ex/b");
  });

  it("deduplicates consecutive identical entries", () => {
    navigationManager.push({ kind: "entity", id: "http://ex/a", source: "test" });
    navigationManager.push({ kind: "entity", id: "http://ex/a", source: "test" });
    assert.equal(navigationManager.getStack().length, 1);
  });

  it("truncates forward history on new push", () => {
    navigationManager.push({ kind: "entity", id: "http://ex/a", source: "test" });
    navigationManager.push({ kind: "entity", id: "http://ex/b", source: "test" });
    navigationManager.back();
    navigationManager.push({ kind: "entity", id: "http://ex/c", source: "test" });
    assert.equal(navigationManager.getStack().length, 2);
    assert.equal(navigationManager.canGoForward(), false);
  });
});
