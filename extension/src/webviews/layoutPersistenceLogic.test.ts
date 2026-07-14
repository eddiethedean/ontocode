import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
  DEFAULT_REOPEN,
  PERSPECTIVES,
  graphRestoreState,
  isAllowedPanelRestoreCommand,
  resolvePanelRestoreState,
  sanitizePanelRestoreState,
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

  it("rejects non-allowlisted restore commands (#309)", () => {
    assert.equal(isAllowedPanelRestoreCommand("workbench.action.terminal.new"), false);
    assert.equal(isAllowedPanelRestoreCommand("vscode.open"), false);
    assert.equal(isAllowedPanelRestoreCommand("ontocode.showEntityInspector"), true);
    assert.equal(isAllowedPanelRestoreCommand("ontocode.openEntity"), true);
    assert.equal(isAllowedPanelRestoreCommand("ontocode.evil;rm"), false);
    assert.equal(
      sanitizePanelRestoreState({
        command: "workbench.action.terminal.new",
        args: [],
      }),
      undefined
    );
  });

  it("falls back to default when saved restore command is not allowlisted", () => {
    const restore = resolvePanelRestoreState(
      {
        ontocodeInspector: {
          command: "workbench.action.terminal.new",
          args: ["--dangerous"],
        },
      },
      "ontocodeInspector"
    );
    assert.deepEqual(restore, DEFAULT_REOPEN.ontocodeInspector);
  });

  it("graphRestoreState maps graphKind to restore commands", () => {
    assert.deepEqual(graphRestoreState({ graphKind: "class" }, "Class Graph"), {
      command: "ontocode.openClassGraph",
      title: "Class Graph",
    });
    assert.deepEqual(graphRestoreState({ graphKind: "property" }, "Property Graph"), {
      command: "ontocode.openPropertyGraph",
      title: "Property Graph",
    });
    assert.deepEqual(graphRestoreState({ graphKind: "import" }, "Import Graph"), {
      command: "ontocode.openImportGraph",
      title: "Import Graph",
    });
    assert.deepEqual(
      graphRestoreState(
        { graphKind: "neighborhood", rootIri: "http://ex.org#Person" },
        "Neighborhood"
      ),
      {
        command: "ontocode.openNeighborhoodGraph",
        args: ["http://ex.org#Person"],
        title: "Neighborhood",
      }
    );
    assert.equal(
      isAllowedPanelRestoreCommand(
        graphRestoreState({ graphKind: "neighborhood", rootIri: "x" }).command
      ),
      true
    );
  });
});
