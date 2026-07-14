import { describe, it } from "node:test";
import assert from "node:assert/strict";
import {
  clearSelfWritesForTests,
  isSelfWrite,
  noteSelfWrite,
  SELF_WRITE_TTL_MS,
} from "./selfWriteGuard";
import {
  findOpenDocumentForEntry,
  noOpenDocumentSaveOutcome,
} from "./saveCoordinatorLogic";
import {
  decideCommitBookkeeping,
  shouldPopStackAfterApply,
} from "./transactionApplyPolicy";

describe("selfWriteGuard (#293)", () => {
  it("marks paths as self-writes within the TTL", () => {
    clearSelfWritesForTests();
    const path = "/ws/demo.ttl";
    const t0 = 1_000_000;
    noteSelfWrite(path, SELF_WRITE_TTL_MS, t0);
    assert.equal(isSelfWrite(path, t0 + 100), true);
    assert.equal(isSelfWrite(path, t0 + SELF_WRITE_TTL_MS + 1), false);
  });

  it("ignores unrelated paths", () => {
    clearSelfWritesForTests();
    noteSelfWrite("/ws/a.ttl", 1000, 100);
    assert.equal(isSelfWrite("/ws/b.ttl", 150), false);
  });
});

describe("transactionApplyPolicy (#296 #297)", () => {
  it("keeps stack entry when apply fails (#296)", () => {
    assert.equal(
      shouldPopStackAfterApply({ applied: false, editor_synced: false }),
      false
    );
  });

  it("pops and bookkeeps when applied but editor sync cancelled (#297)", () => {
    assert.equal(
      shouldPopStackAfterApply({
        applied: true,
        editor_synced: false,
        undo_patches: [{ op: "delete_entity", entity_iri: "x" }],
      }),
      true
    );
    const decision = decideCommitBookkeeping({
      applied: true,
      editor_synced: false,
      undo_patches: [{ op: "delete_entity", entity_iri: "x" }],
    });
    assert.equal(decision.markDirty, true);
    assert.equal(decision.pushUndo, true);
    assert.equal(decision.throwNotApplied, false);
  });

  it("throws bookkeeping path when not applied", () => {
    const decision = decideCommitBookkeeping({
      applied: false,
      editor_synced: false,
    });
    assert.equal(decision.throwNotApplied, true);
    assert.equal(decision.markDirty, false);
    assert.equal(decision.pushUndo, false);
  });
});

describe("saveCoordinatorLogic (#299)", () => {
  it("finds open documents by fsPath when URI strings differ", () => {
    const docs = [
      {
        uriString: "file:///ws/demo.ttl",
        fsPath: "/ws/demo.ttl",
        isDirty: true,
      },
    ];
    const found = findOpenDocumentForEntry(
      docs,
      "file:///private/ws/demo.ttl",
      "/ws/demo.ttl"
    );
    assert.ok(found);
    assert.equal(found.fsPath, "/ws/demo.ttl");
  });

  it("does not claim saved when no open document matched", () => {
    const outcome = noOpenDocumentSaveOutcome();
    assert.equal(outcome.claimSaved, false);
    assert.equal(outcome.markClean, true);
  });
});
