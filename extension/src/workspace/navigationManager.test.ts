import { describe, it, beforeEach, afterEach } from "node:test";
import assert from "node:assert/strict";
import { focusRelay } from "../focus/focusRelay";
import { navigationManager } from "./navigationManager";
import { selectionManager } from "./selectionManager";

describe("navigationManager", () => {
  let unsubscribe: (() => void) | undefined;

  beforeEach(() => {
    navigationManager.resetForTests();
    selectionManager.resetForTests();
    focusRelay.resetForTests();
    unsubscribe = selectionManager.subscribeToFocus();
  });

  afterEach(() => {
    unsubscribe?.();
    unsubscribe = undefined;
    navigationManager.resetForTests();
    selectionManager.resetForTests();
    focusRelay.resetForTests();
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

  it("preserves forward history when focus re-applies current slot (#292)", () => {
    navigationManager.push({ kind: "entity", id: "http://ex/a", source: "test" });
    navigationManager.push({ kind: "entity", id: "http://ex/b", source: "test" });
    void navigationManager.runHistoryNavigation(() => {
      navigationManager.back();
      // Simulate openEntity re-focus during Back (commands do this).
      focusRelay.setEntityFocus("http://ex/a", "explorer");
    });
    assert.equal(navigationManager.getStack().length, 2);
    assert.equal(navigationManager.canGoForward(), true);
    const forward = navigationManager.forward();
    assert.equal(forward?.id, "http://ex/b");
  });
});
