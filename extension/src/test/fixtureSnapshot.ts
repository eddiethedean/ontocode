import { CatalogSnapshot } from "../lsp/protocol";

export const fixtureCatalogSnapshot: CatalogSnapshot = {
  documents: [
    {
      id: "doc-1",
      path: "fixtures/example.ttl",
      format: "turtle",
      base_iri: "http://example.org/people",
      parse_status: "ok",
    },
  ],
  entities: [
    {
      iri: "http://example.org/people#Person",
      short_name: "Person",
      kind: "class",
      ontology_id: "doc-1",
      labels: ["Person"],
      comments: ["A human being."],
      deprecated: false,
    },
    {
      iri: "http://example.org/people#Organization",
      short_name: "Organization",
      kind: "class",
      ontology_id: "doc-1",
      labels: ["Organization"],
      comments: [],
      deprecated: false,
    },
    {
      iri: "http://example.org/people#worksFor",
      short_name: "worksFor",
      kind: "object_property",
      ontology_id: "doc-1",
      labels: ["works for"],
      comments: [],
      deprecated: false,
    },
    {
      iri: "http://example.org/people#age",
      short_name: "age",
      kind: "data_property",
      ontology_id: "doc-1",
      labels: ["age"],
      comments: [],
      deprecated: false,
    },
    {
      iri: "http://example.org/people#note",
      short_name: "note",
      kind: "annotation_property",
      ontology_id: "doc-1",
      labels: ["note"],
      comments: [],
      deprecated: false,
    },
    {
      iri: "http://example.org/people#alice",
      short_name: "alice",
      kind: "individual",
      ontology_id: "doc-1",
      labels: ["Alice"],
      comments: [],
      deprecated: false,
    },
    {
      iri: "http://example.org/people#acme",
      short_name: "acme",
      kind: "individual",
      ontology_id: "doc-1",
      labels: ["ACME Corp"],
      comments: [],
      deprecated: false,
    },
  ],
  hierarchy: {
    edges: [
      {
        child: "http://example.org/people#Person",
        parent: "http://www.w3.org/2002/07/owl#Thing",
      },
    ],
    parents: {
      "http://example.org/people#Person": [
        "http://www.w3.org/2002/07/owl#Thing",
      ],
    },
    children: {
      "http://www.w3.org/2002/07/owl#Thing": [
        "http://example.org/people#Person",
      ],
    },
  },
  diagnostics: [],
};
