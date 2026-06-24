import { PatchOp } from "../lsp/protocol";

export type ManchesterAxiomKind = "sub_class_of" | "equivalent_class";

export function buildManchesterPatch(
  axiomKind: ManchesterAxiomKind,
  entityIri: string,
  manchester: string,
  mode: "add" | "remove" | "set"
): PatchOp {
  if (axiomKind === "equivalent_class") {
    if (mode === "remove") {
      return {
        op: "remove_equivalent_class",
        entity_iri: entityIri,
        manchester,
      };
    }
    if (mode === "set") {
      return {
        op: "set_equivalent_class",
        entity_iri: entityIri,
        manchester,
      };
    }
    return { op: "add_equivalent_class", entity_iri: entityIri, manchester };
  }
  if (mode === "remove") {
    return {
      op: "remove_complex_sub_class_of",
      entity_iri: entityIri,
      manchester,
    };
  }
  return {
    op: "add_complex_sub_class_of",
    entity_iri: entityIri,
    manchester,
  };
}

/** Build patch list for apply (handles SubClassOf edit as remove+add). */
export function buildManchesterPatches(
  axiomKind: ManchesterAxiomKind,
  entityIri: string,
  expression: string,
  mode: "add" | "edit",
  initialExpression?: string
): PatchOp[] {
  if (mode === "edit" && axiomKind === "sub_class_of") {
    const patches: PatchOp[] = [];
    if (initialExpression?.trim()) {
      patches.push(
        buildManchesterPatch(
          axiomKind,
          entityIri,
          initialExpression,
          "remove"
        )
      );
    }
    patches.push(buildManchesterPatch(axiomKind, entityIri, expression, "add"));
    return patches;
  }
  const patchMode = mode === "edit" ? "set" : "add";
  return [buildManchesterPatch(axiomKind, entityIri, expression, patchMode)];
}
