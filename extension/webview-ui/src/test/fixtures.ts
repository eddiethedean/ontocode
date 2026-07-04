import type {
  DiffPayload,
  EntityDetailPayload,
  GraphPayload,
  HostMessage,
  ManchesterCompletions,
  RefactorPlanPayload,
  SavedQuery,
  TabularQueryResult,
} from "../messages";
import { vscodePostMessage } from "./vscodeMock";

export const entityDetail: EntityDetailPayload = {
  entity: {
    iri: "http://example.org#Person",
    short_name: "Person",
    kind: "class",
    labels: ["Person", "Human being"],
    comments: ["A person entity"],
    deprecated: false,
    obo_id: "EX:001",
  },
  parents: ["http://example.org#Agent"],
  children: ["http://example.org#Student"],
  axioms: [
    {
      kind: "sub_class_of",
      display: "SubClassOf Agent",
      manchester: "Agent",
      parent_iri: "http://example.org#Agent",
      editable: true,
    },
  ],
  editable: true,
  document_path: "/workspace/ontology.ttl",
};

export const classOptions = [
  "http://example.org#Agent",
  "http://example.org#Person",
  "http://example.org#Thing",
];

export const graphPayload: GraphPayload = {
  graph_kind: "class",
  truncated: false,
  nodes: [
    { id: "http://example.org#Person", label: "Person", kind: "class" },
    { id: "http://example.org#Agent", label: "Agent", kind: "class" },
  ],
  edges: [
    {
      source: "http://example.org#Person",
      target: "http://example.org#Agent",
      kind: "subClassOf",
      inferred: false,
    },
  ],
};

export const refactorPlan: RefactorPlanPayload = {
  warnings: ["Review import statements"],
  changes: [
    {
      path: "/workspace/ontology.ttl",
      original_text: "ex:OldName a owl:Class .",
      preview_text: "ex:NewName a owl:Class .",
      hunks: [],
    },
  ],
};

export const savedQueries: SavedQuery[] = [
  { name: "All classes", mode: "sql", text: "SELECT * FROM classes" },
];

export const queryHistory: SavedQuery[] = [
  { name: "Recent", mode: "sparql", text: "SELECT ?s WHERE { ?s ?p ?o }" },
];

export const queryResult: TabularQueryResult = {
  columns: ["short_name", "labels"],
  rows: [
    { short_name: "Person", labels: "Person" },
    { short_name: "Agent", labels: "Agent" },
  ],
};

export const manchesterCompletions: ManchesterCompletions = {
  classes: ["ex:Person", "ex:Agent"],
  object_properties: ["ex:hasPart"],
  data_properties: ["ex:age"],
  datatypes: ["xsd:string"],
};

export const diffPayload: DiffPayload = {
  entity_changes: [{ kind: "added", iri: "http://example.org#Person" }],
  axiom_changes: [
    {
      change: "added",
      subject: "http://example.org#Person",
      predicate: "rdfs:subClassOf",
      object: "owl:Thing",
      axiom_kind: "sub_class_of",
    },
  ],
  annotation_changes: [
    {
      change: "added",
      subject: "http://example.org#Person",
      predicate: "rdfs:label",
      object: '"Person"',
    },
  ],
  import_changes: [
    {
      change: "added",
      ontology_id: "http://example.org",
      import_iri: "http://example.org/imports/other",
    },
  ],
  inference_changes: [
    {
      class_iri: "http://example.org#Person",
      change: "added",
      detail: "Inferred subclass of Agent",
    },
  ],
  breaking_changes: [
    {
      reason: "removed_entity",
      message: "Removed class breaks downstream references",
      entity_iri: "http://example.org#Old",
    },
  ],
  summary_counts: {
    entities: 1,
    axioms: 1,
    annotations: 1,
    imports: 1,
    inferences: 1,
    breaking: 1,
  },
};

export function postHostMessage(message: HostMessage): void {
  window.dispatchEvent(new MessageEvent("message", { data: message }));
}

export function lastPostedMessage(): unknown {
  const calls = vscodePostMessage.mock.calls;
  return calls.length > 0 ? calls[calls.length - 1][0] : undefined;
}

export function postedMessages(): unknown[] {
  return vscodePostMessage.mock.calls.map((call) => call[0]);
}

export function postedMessagesOfType(type: string): unknown[] {
  return postedMessages().filter((m) => (m as { type?: string }).type === type);
}

/** Build a diff payload with N entity changes for truncation tests. */
export function largeEntityDiff(count: number): DiffPayload {
  return {
    entity_changes: Array.from({ length: count }, (_, i) => ({
      kind: "added",
      iri: `http://example.org#Class${i}`,
    })),
    axiom_changes: [],
    annotation_changes: [],
    import_changes: [],
    inference_changes: [],
    breaking_changes: [],
  };
}

export const multiFileRefactorPlan: RefactorPlanPayload = {
  changes: [
    {
      path: "/workspace/ontology.ttl",
      original_text: "ex:A a owl:Class .",
      preview_text: "ex:B a owl:Class .",
      hunks: [],
    },
    {
      path: "/workspace/imports.ttl",
      original_text: "ex:Old a owl:Class .",
      preview_text: "ex:New a owl:Class .",
      hunks: [],
    },
  ],
};

export const graphWithInferredEdge: GraphPayload = {
  ...graphPayload,
  edges: [
    ...graphPayload.edges,
    {
      source: "http://example.org#Person",
      target: "http://example.org#Student",
      kind: "subClassOf",
      inferred: true,
    },
  ],
};
