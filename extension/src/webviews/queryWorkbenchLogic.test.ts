import {
  exportResultCsv,
  mergeHistory,
  shouldDeliverQueryResult,
  upsertSavedQuery,
} from "./queryWorkbenchLogic";
import assert from "node:assert/strict";
import { test } from "node:test";

test("exportResultCsv escapes commas", () => {
  const csv = exportResultCsv({
    columns: ["a", "b"],
    rows: [{ a: "hello, world", b: "x" }],
  });
  assert.ok(csv.includes('"hello, world"'));
});

test("exportResultCsv quotes cells with newlines", () => {
  const csv = exportResultCsv({
    columns: ["label"],
    rows: [{ label: "line1\nline2" }],
  });
  assert.ok(csv.includes('"line1\nline2"'));
});

test("mergeHistory caps entries", () => {
  const history = mergeHistory(
    [{ name: "old", mode: "sql", text: "SELECT 1" }],
    { name: "new", mode: "sql", text: "SELECT 2" },
    1
  );
  assert.equal(history.length, 1);
  assert.equal(history[0]?.text, "SELECT 2");
});

test("shouldDeliverQueryResult matches active run id", () => {
  assert.equal(shouldDeliverQueryResult(3, 3), true);
  assert.equal(shouldDeliverQueryResult(3, 0), false);
});

test("upsertSavedQuery replaces same name", () => {
  const saved = upsertSavedQuery(
    [{ name: "q1", mode: "sql", text: "A" }],
    { name: "q1", mode: "sparql", text: "B" }
  );
  assert.equal(saved.length, 1);
  assert.equal(saved[0]?.mode, "sparql");
});
