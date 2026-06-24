import { buildManchesterPatch } from "./manchesterEditorLogic";
import assert from "node:assert/strict";
import { test } from "node:test";

test("buildManchesterPatch for complex subclass", () => {
  const patch = buildManchesterPatch(
    "sub_class_of",
    "http://ex/Patient",
    "ex:hasRecord some ex:MedicalRecord",
    "add"
  );
  assert.equal(patch.op, "add_complex_sub_class_of");
});

test("buildManchesterPatch for equivalent", () => {
  const patch = buildManchesterPatch(
    "equivalent_class",
    "http://ex/Staff",
    "ex:Person and ex:Employee",
    "set"
  );
  assert.equal(patch.op, "set_equivalent_class");
});
