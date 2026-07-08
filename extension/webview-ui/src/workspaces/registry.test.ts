import { describe, it, expect } from "vitest";
import { getWorkspace, getWorkspaceByPanelKind, listWorkspaces } from "./registry";

describe("WorkspaceRegistry", () => {
  it("returns registered workspace by id", () => {
    const ws = getWorkspace("entity");
    expect(ws).toBeDefined();
    expect(ws?.title).toBe("Entity Inspector");
    expect(ws?.panelKind).toBe("inspector");
  });

  it("resolves by panel kind", () => {
    const ws = getWorkspaceByPanelKind("queryWorkbench");
    expect(ws?.id).toBe("query");
  });

  it("lists all workspaces", () => {
    const all = listWorkspaces();
    expect(all.length).toBeGreaterThanOrEqual(4);
    expect(all.map((w) => w.id)).toContain("graph");
  });
});
