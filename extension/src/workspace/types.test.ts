import { describe, it } from "node:test";
import assert from "node:assert/strict";
import type { OntologyDocument } from "../lsp/protocol";
import {
  entryFromDocument,
  inferRole,
  isEditableFormat,
} from "./types";

function doc(
  overrides: Partial<OntologyDocument> & Pick<OntologyDocument, "id" | "path" | "format">
): OntologyDocument {
  return {
    parse_status: "ok",
    ...overrides,
  };
}

describe("workspace types", () => {
  it("isEditableFormat accepts turtle and obo only", () => {
    assert.equal(isEditableFormat("turtle"), true);
    assert.equal(isEditableFormat("OBO"), true);
    assert.equal(isEditableFormat("owl"), false);
  });

  it("inferRole marks imported documents as import", () => {
    const root = doc({
      id: "root",
      path: "/ws/root.ttl",
      format: "turtle",
      base_iri: "http://example.org/root",
    });
    const imported = doc({
      id: "imported",
      path: "/ws/imported.ttl",
      format: "turtle",
      base_iri: "http://example.org/imported",
    });
    root.imports = [imported.path];
    assert.equal(inferRole(root, [root, imported]), "root");
    assert.equal(inferRole(imported, [root, imported]), "import");
  });

  it("entryFromDocument marks imports read-only", () => {
    const root = doc({
      id: "root",
      path: "/ws/root.ttl",
      format: "turtle",
    });
    const imported = doc({
      id: "imported",
      path: "/ws/imported.ttl",
      format: "turtle",
    });
    root.imports = [imported.path];
    const entry = entryFromDocument(imported, [root, imported], {
      uri: "file:///ws/imported.ttl",
      dirty: false,
      active: false,
    });
    assert.equal(entry.role, "import");
    assert.equal(entry.editable, false);
  });

  it("entryFromDocument marks turtle root editable", () => {
    const root = doc({
      id: "root",
      path: "/ws/root.ttl",
      format: "turtle",
    });
    const entry = entryFromDocument(root, [root], {
      uri: "file:///ws/root.ttl",
      dirty: true,
      active: true,
      version: 2,
    });
    assert.equal(entry.editable, true);
    assert.equal(entry.dirty, true);
    assert.equal(entry.active, true);
    assert.equal(entry.version, 2);
  });
});
