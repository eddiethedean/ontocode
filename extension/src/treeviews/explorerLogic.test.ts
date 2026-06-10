import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { fixtureCatalogSnapshot } from "../test/fixtureSnapshot";
import {
  childEntitiesForClass,
  classRootEntities,
  entityDisplayLabel,
  filterEntitiesByKind,
  propertyGroupsPresent,
} from "../treeviews/explorerLogic";

describe("explorerLogic", () => {
  it("filters entities by kind", () => {
    const classes = filterEntitiesByKind(fixtureCatalogSnapshot.entities, "class");
    assert.equal(classes.length, 2);
    assert.equal(classes[0]?.short_name, "Person");
  });

  it("finds class roots excluding subclass children", () => {
    const roots = classRootEntities(fixtureCatalogSnapshot);
    assert.deepEqual(
      roots.map((e) => e.short_name).sort(),
      ["Organization"]
    );
  });

  it("lists property groups present in the snapshot", () => {
    const groups = propertyGroupsPresent(fixtureCatalogSnapshot);
    assert.deepEqual(
      groups.map((g) => g.kind).sort(),
      ["annotation_property", "data_property", "object_property"]
    );
  });

  it("returns child classes for a parent IRI", () => {
    const children = childEntitiesForClass(
      fixtureCatalogSnapshot,
      "http://www.w3.org/2002/07/owl#Thing"
    );
    assert.deepEqual(children.map((e) => e.short_name), ["Person"]);
  });

  it("prefers labels for display names", () => {
    assert.equal(
      entityDisplayLabel(fixtureCatalogSnapshot.entities[0]!),
      "Person"
    );
  });
});
