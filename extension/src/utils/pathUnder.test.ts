import assert from "node:assert/strict";
import path from "node:path";
import { describe, it } from "node:test";
import { isPathUnderFolder } from "./pathUnder";

describe("isPathUnderFolder", () => {
  it("matches the folder itself and descendants", () => {
    const folder = path.join(path.sep, "workspace", "proj");
    assert.equal(isPathUnderFolder(folder, folder), true);
    assert.equal(
      isPathUnderFolder(path.join(folder, "ontologies", "a.ttl"), folder),
      true
    );
  });

  it("rejects sibling folders that share a path prefix (#151)", () => {
    const folder = path.join(path.sep, "workspace", "proj");
    const sibling = path.join(path.sep, "workspace", "proj-other", "b.ttl");
    assert.equal(isPathUnderFolder(sibling, folder), false);
  });

  it("rejects paths outside the folder", () => {
    const folder = path.join(path.sep, "workspace", "proj");
    assert.equal(
      isPathUnderFolder(path.join(path.sep, "workspace", "other", "a.ttl"), folder),
      false
    );
  });
});
