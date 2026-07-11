import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
  PERSPECTIVES,
  resolvePanelRestoreState,
  type PanelRestoreState,
} from "./layoutPersistenceLogic";

describe("layoutPersistenceLogic", () => {
  it("exposes default Modeling/Reasoning/Review perspectives", () => {
    assert.deepEqual(
      PERSPECTIVES.map((p) => p.name),
      ["Modeling", "Reasoning", "Review"]
    );
  });

  it("falls back to default reopen commands when no saved state", () => {
    const restore = resolvePanelRestoreState(undefined, "ontocodeQueryWorkbench");
    assert.equal(restore?.command, "ontocode.openQueryWorkbench");
  });

  it("prefers saved restore state over defaults", () => {
    const saved: PanelRestoreState = {
      command: "ontocode.showExplanation",
      args: ["http://example.org#A", "el"],
      title: "Explanation: A",
    };
    assert.deepEqual(
      resolvePanelRestoreState({ ontocodeExplanation: saved }, "ontocodeExplanation"),
      saved
    );
  });
});
