import {
  dlQueryToTabular,
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

test("upsertSavedQuery accepts dl mode", () => {
  const saved = upsertSavedQuery([], {
    name: "q-dl",
    mode: "dl",
    text: "Person and hasPet some Animal",
    dlMode: "asserted",
  });
  assert.equal(saved[0]?.mode, "dl");
  assert.equal(saved[0]?.dlMode, "asserted");
});

test("mergeHistory preserves dlMode", () => {
  const history = mergeHistory(
    [],
    { name: "dl", mode: "dl", text: "Person", dlMode: "asserted" },
    5
  );
  assert.equal(history[0]?.dlMode, "asserted");
});

test("dlQueryToTabular flattens tabs", () => {
  const tabular = dlQueryToTabular({
    expression: "Person",
    normalized: "Person",
    query_class_iri: "http://example.org#Person",
    instances: ["http://example.org#Alice"],
    subclasses: ["http://example.org#Student"],
    superclasses: ["http://example.org#Agent"],
    equivalents: [],
    profile: "dl",
    mode: "inferred",
    duration_ms: 1,
  });
  assert.deepEqual(tabular.columns, ["kind", "iri"]);
  assert.equal(tabular.rows.length, 3);
  assert.equal(tabular.rows[0]?.kind, "instance");
});
