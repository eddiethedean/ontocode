import { describe, it, expect } from "vitest";
import { isDiffPayload, isHostMessage } from "./messages";

const validDiff = {
  entity_changes: [{ kind: "added", iri: "http://ex#C" }],
  axiom_changes: [
    {
      change: "added",
      subject: "http://ex#C",
      predicate: "rdfs:subClassOf",
      object: "owl:Thing",
      axiom_kind: "sub_class_of",
    },
  ],
  annotation_changes: [],
  import_changes: [],
  inference_changes: [],
  breaking_changes: [],
};

describe("isDiffPayload", () => {
  it("accepts well-formed diff payloads", () => {
    expect(isDiffPayload(validDiff)).toBe(true);
  });

  it("accepts diffs with all change types populated", () => {
    expect(
      isDiffPayload({
        entity_changes: [{ kind: "removed", iri: "http://ex#C" }],
        axiom_changes: [
          {
            change: "removed",
            subject: "http://ex#C",
            predicate: "rdfs:subClassOf",
            object: "owl:Thing",
            axiom_kind: "sub_class_of",
          },
        ],
        annotation_changes: [
          {
            change: "added",
            subject: "http://ex#C",
            predicate: "rdfs:label",
            object: '"C"',
          },
        ],
        import_changes: [
          { change: "removed", ontology_id: "http://ex", import_iri: "http://other" },
        ],
        inference_changes: [
          { class_iri: "http://ex#C", change: "added", detail: "inferred" },
        ],
        breaking_changes: [{ reason: "removed_entity", message: "breaking" }],
      })
    ).toBe(true);
  });

  it("rejects malformed diff payloads", () => {
    expect(isDiffPayload(null)).toBe(false);
    expect(isDiffPayload(undefined)).toBe(false);
    expect(isDiffPayload({})).toBe(false);
    expect(isDiffPayload({ ...validDiff, entity_changes: "bad" })).toBe(false);
    expect(isDiffPayload({ ...validDiff, entity_changes: [{}] })).toBe(false);
    expect(isDiffPayload({ ...validDiff, axiom_changes: [{ change: "added" }] })).toBe(false);
    expect(isDiffPayload({ ...validDiff, annotation_changes: [{ change: "x" }] })).toBe(false);
    expect(isDiffPayload({ ...validDiff, import_changes: [{ change: "x" }] })).toBe(false);
    expect(isDiffPayload({ ...validDiff, inference_changes: [{ change: "x" }] })).toBe(false);
    expect(isDiffPayload({ ...validDiff, breaking_changes: [{ reason: "x" }] })).toBe(false);
  });
});

describe("isHostMessage", () => {
  it("accepts init and loading messages", () => {
    expect(isHostMessage({ type: "init", panel: "smoke" })).toBe(true);
    expect(isHostMessage({ type: "loading" })).toBe(true);
    expect(isHostMessage({ type: "preview", text: "ttl" })).toBe(true);
  });

  it("accepts semanticDiffData with valid diff", () => {
    expect(isHostMessage({ type: "semanticDiffData", diff: validDiff })).toBe(true);
  });

  it("accepts error messages with string message", () => {
    expect(isHostMessage({ type: "error", message: "failed" })).toBe(true);
  });

  it("accepts loadEntity messages", () => {
    expect(
      isHostMessage({
        type: "loadEntity",
        detail: {
          entity: {
            iri: "http://ex#C",
            short_name: "C",
            kind: "class",
            labels: [],
            comments: [],
            deprecated: false,
          },
          parents: [],
          children: [],
          axioms: [],
          editable: true,
        },
        classOptions: [],
      })
    ).toBe(true);
  });

  it("accepts graph, refactor, query, and manchester messages", () => {
    expect(
      isHostMessage({
        type: "graphData",
        graph: {
          nodes: [],
          edges: [],
          truncated: false,
          graph_kind: "class",
        },
      })
    ).toBe(true);
    expect(
      isHostMessage({
        type: "loadRefactorPlan",
        plan: { changes: [] },
      })
    ).toBe(true);
    expect(
      isHostMessage({
        type: "queryInit",
        saved: [],
        history: [],
        sqlTables: [],
      })
    ).toBe(true);
    expect(
      isHostMessage({
        type: "queryResult",
        runId: 1,
        result: { columns: [], rows: [] },
      })
    ).toBe(true);
    expect(
      isHostMessage({
        type: "manchesterInit",
        entityIri: "http://ex#C",
        axiomKind: "sub_class_of",
        expression: "",
        completions: {
          classes: [],
          object_properties: [],
          data_properties: [],
          datatypes: [],
        },
      })
    ).toBe(true);
    expect(
      isHostMessage({
        type: "manchesterValidation",
        seq: 1,
        result: {
          normalized: "",
          turtle_fragment: "",
          tree: null,
          diagnostics: [],
        },
      })
    ).toBe(true);
  });

  it("rejects invalid payloads", () => {
    expect(isHostMessage(null)).toBe(false);
    expect(isHostMessage(undefined)).toBe(false);
    expect(isHostMessage({})).toBe(false);
    expect(isHostMessage({ type: 123 })).toBe(false);
    expect(isHostMessage({ type: "semanticDiffData", diff: {} })).toBe(false);
    expect(isHostMessage({ type: "error" })).toBe(false);
    expect(isHostMessage({ type: "error", message: 1 })).toBe(false);
    expect(isHostMessage({ notType: "init" })).toBe(false);
  });
});
