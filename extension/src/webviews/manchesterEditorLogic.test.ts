import {
  buildManchesterPatch,
  buildManchesterPatches,
  resolveManchesterApplyMode,
} from "./manchesterEditorLogic";
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

test("buildManchesterPatches for subclass edit removes then adds", () => {
  const patches = buildManchesterPatches(
    "sub_class_of",
    "http://ex/Patient",
    "ex:hasRecord some ex:NewRecord",
    "edit",
    "ex:hasRecord some ex:MedicalRecord"
  );
  assert.equal(patches.length, 2);
  assert.equal(patches[0].op, "remove_complex_sub_class_of");
  assert.equal(patches[1].op, "add_complex_sub_class_of");
});

test("buildManchesterPatches for equivalent edit is single set", () => {
  const patches = buildManchesterPatches(
    "equivalent_class",
    "http://ex/Staff",
    "ex:Person and ex:Employee",
    "edit",
    "ex:Person"
  );
  assert.equal(patches.length, 1);
  assert.equal(patches[0].op, "set_equivalent_class");
});

test("resolveManchesterApplyMode forces add when axiom kind changes", () => {
  const resolved = resolveManchesterApplyMode(
    "equivalent_class",
    "sub_class_of",
    "edit",
    "ex:Agent"
  );
  assert.equal(resolved.mode, "add");
  assert.equal(resolved.initialExpression, undefined);
});

test("resolveManchesterApplyMode keeps edit options when kind matches", () => {
  const resolved = resolveManchesterApplyMode(
    "sub_class_of",
    "sub_class_of",
    "edit",
    "ex:Agent"
  );
  assert.equal(resolved.mode, "edit");
  assert.equal(resolved.initialExpression, "ex:Agent");
});
