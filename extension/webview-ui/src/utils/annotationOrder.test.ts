import { describe, expect, it } from "vitest"
import {
  cmpAnnotationPropertyIri,
  sortAnnotationsByPredicate,
} from "./annotationOrder"

describe("annotationOrder", () => {
  const label = "http://www.w3.org/2000/01/rdf-schema#label"
  const comment = "http://www.w3.org/2000/01/rdf-schema#comment"

  it("orders rdfs:label before rdfs:comment", () => {
    expect(cmpAnnotationPropertyIri(label, comment)).toBeLessThan(0)
  })

  it("sorts annotation list stably by predicate", () => {
    const sorted = sortAnnotationsByPredicate([
      { predicate: comment, value: "c" },
      { predicate: "http://example.org/z", value: "z" },
      { predicate: label, value: "l" },
    ])
    expect(sorted.map((a) => a.predicate)).toEqual([
      label,
      comment,
      "http://example.org/z",
    ])
  })
})
