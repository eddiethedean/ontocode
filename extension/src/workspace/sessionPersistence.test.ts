import { describe, it } from "node:test";
import assert from "node:assert/strict";
import type { WorkspaceSessionSnapshot } from "./types";
import { isAllowedPanelRestoreCommand } from "../webviews/layoutPersistenceLogic";
import { isNotIndexedError } from "../utils/lspErrors";

describe("workspace session snapshot shape", () => {
  it("accepts bounded navigation and panel restore payloads", () => {
    const snapshot: WorkspaceSessionSnapshot = {
      openOntologyUris: ["file:///ws/a.ttl"],
      activeOntologyId: "http://example.org/a",
      focus: { kind: "entity", id: "http://example.org/a#Person", source: "inspector" },
      navigation: [
        { kind: "entity", id: "http://example.org/a#Org", source: "explorer" },
        { kind: "entity", id: "http://example.org/a#Person", source: "inspector" },
      ],
      navigationIndex: 1,
      panelRestore: {
        ontocodeInspector: {
          command: "ontocode.showEntityInspector",
          args: ["http://example.org/a#Person"],
        },
      },
    };
    const roundTrip = JSON.parse(JSON.stringify(snapshot)) as WorkspaceSessionSnapshot;
    assert.equal(roundTrip.openOntologyUris.length, 1);
    assert.equal(roundTrip.navigationIndex, 1);
    assert.equal(
      roundTrip.panelRestore.ontocodeInspector.command,
      "ontocode.showEntityInspector"
    );
  });

  it("treats non-ontocode panel restore commands as disallowed (#309)", () => {
    assert.equal(isAllowedPanelRestoreCommand("workbench.action.terminal.new"), false);
    assert.equal(isAllowedPanelRestoreCommand("ontocode.showEntityInspector"), true);
  });

  it("detects NOT_INDEXED catalog errors (#294)", () => {
    assert.equal(
      isNotIndexedError(new Error("NOT_INDEXED: workspace not indexed yet")),
      true
    );
    assert.equal(isNotIndexedError(new Error("OTHER: boom")), false);
    assert.equal(isNotIndexedError("NOT_INDEXED"), false);
  });
});
