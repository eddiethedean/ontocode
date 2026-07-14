import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { fixtureCatalogSnapshot } from "../test/fixtureSnapshot";
import {
  childEntitiesForClass,
  classRootEntities,
  diagnosticLabel,
  entityDisplayLabel,
  filterEntitiesByKind,
  groupDiagnosticsBySeverity,
  propertyGroupsPresent,
} from "../treeviews/explorerLogic";

describe("explorerLogic", () => {
  it("filters entities by kind", () => {
    const classes = filterEntitiesByKind(fixtureCatalogSnapshot.entities, "class");
    assert.ok(classes.length >= 4);
    assert.ok(classes.some((c) => c.short_name === "Person"));
    assert.ok(classes.some((c) => c.short_name === "Thing"));
    assert.ok(classes.some((c) => c.short_name === "Patient"));
  });

  it("finds class roots from real fixture hierarchy", () => {
    const roots = classRootEntities(fixtureCatalogSnapshot);
    assert.deepEqual(
      roots.map((e) => e.short_name).sort(),
      ["A", "Animal", "B", "Cat", "ClinicPerson", "Dog", "Thing"]
    );
  });

  it("falls back to all classes when SubClassOf forms a pure cycle (#222)", () => {
    const aIri = "http://example.org/cycle#A";
    const bIri = "http://example.org/cycle#B";
    const snapshot = {
      ...fixtureCatalogSnapshot,
      entities: [
        {
          iri: aIri,
          short_name: "A",
          kind: "class",
          ontology_id: "http://example.org/cycle",
          labels: ["A"],
          comments: [],
          deprecated: false,
        },
        {
          iri: bIri,
          short_name: "B",
          kind: "class",
          ontology_id: "http://example.org/cycle",
          labels: ["B"],
          comments: [],
          deprecated: false,
        },
      ],
      hierarchy: {
        edges: [
          { child: aIri, parent: bIri },
          { child: bIri, parent: aIri },
        ],
        parents: {
          [aIri]: [bIri],
          [bIri]: [aIri],
        },
        children: {
          [aIri]: [bIri],
          [bIri]: [aIri],
        },
      },
    };
    const roots = classRootEntities(snapshot as typeof fixtureCatalogSnapshot);
    assert.deepEqual(
      roots.map((e) => e.short_name).sort(),
      ["A", "B"]
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
      "http://example.org/people#Thing"
    );
    assert.deepEqual(
      children.map((e) => e.short_name).sort(),
      ["Organization", "Person"]
    );
  });

  it("prefers labels for display names", () => {
    const person = fixtureCatalogSnapshot.entities.find(
      (e) => e.short_name === "Person"
    );
    assert.ok(person);
    assert.equal(entityDisplayLabel(person!), "Person");
  });

  it("groups diagnostics by severity", () => {
    const groups = groupDiagnosticsBySeverity([
      {
        code: "broken_import",
        severity: "error",
        message: "missing import",
        file: "a.ttl",
        line: 2,
      },
      {
        code: "orphan_class",
        severity: "warning",
        message: "no parent",
        file: "a.ttl",
      },
    ]);
    assert.equal(groups.get("error")?.length, 1);
    assert.equal(groups.get("warning")?.length, 1);
    assert.match(diagnosticLabel(groups.get("error")![0]!), /missing import/);
  });
});
