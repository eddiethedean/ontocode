import { describe, expect, it } from "vitest"
import { annotationTextParts, extractLinks } from "./annotationLinks"

describe("annotationLinks", () => {
  it("extracts DOI and PMID", () => {
    const links = extractLinks("See DOI: 10.1000/xyz and PMID: 12345")
    expect(links).toHaveLength(2)
    expect(links[0].kind).toBe("DOI")
    expect(links[0].url).toBe("https://doi.org/10.1000/xyz")
    expect(links[1].kind).toBe("PubMedId")
    expect(links[1].url).toBe("http://www.ncbi.nlm.nih.gov/pubmed/12345")
  })

  it("prefers OMIMPS over OMIM", () => {
    const links = extractLinks("OMIMPS: 99")
    expect(links).toHaveLength(1)
    expect(links[0].kind).toBe("OMIMPS")
  })

  it("splits text into link parts", () => {
    const parts = annotationTextParts("before PMID: 1 after")
    expect(parts).toEqual([
      { type: "text", value: "before " },
      {
        type: "link",
        value: "PMID: 1",
        url: "http://www.ncbi.nlm.nih.gov/pubmed/1",
        kind: "PubMedId",
      },
      { type: "text", value: " after" },
    ])
  })
})
