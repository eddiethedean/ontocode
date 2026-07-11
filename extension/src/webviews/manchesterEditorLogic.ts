import { PatchOp } from "../lsp/protocol";

export type ManchesterAxiomKind =
  | "sub_class_of"
  | "equivalent_class"
  | "disjoint_class";

export function buildManchesterPatch(
  axiomKind: ManchesterAxiomKind,
  entityIri: string,
  manchester: string,
  mode: "add" | "remove" | "set",
  otherIri?: string
): PatchOp {
  if (axiomKind === "disjoint_class") {
    if (!otherIri) {
      throw new Error("disjoint_class requires other_iri");
    }
    if (mode === "remove") {
      return {
        op: "remove_disjoint_class",
        entity_iri: entityIri,
        other_iri: otherIri,
      };
    }
    return {
      op: "add_disjoint_class",
      entity_iri: entityIri,
      other_iri: otherIri,
    };
  }
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

/**
 * When the webview changes axiom kind after open, ignore frozen edit-mode
 * options so we do not remove/set the wrong axiom (#29).
 */
export function resolveManchesterApplyMode(
  requestedKind: string,
  openedKind: string | undefined,
  openedMode: "add" | "edit" | undefined,
  openedInitialExpression: string | undefined
): { mode: "add" | "edit"; initialExpression?: string } {
  const kind = openedKind ?? "sub_class_of";
  if (requestedKind !== kind) {
    return { mode: "add", initialExpression: undefined };
  }
  return {
    mode: openedMode ?? "add",
    initialExpression: openedInitialExpression,
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
  if (axiomKind === "disjoint_class") {
    const other = expression.trim();
    if (mode === "edit" && initialExpression?.trim()) {
      return [
        buildManchesterPatch(
          axiomKind,
          entityIri,
          initialExpression,
          "remove",
          initialExpression.trim()
        ),
        buildManchesterPatch(axiomKind, entityIri, other, "add", other),
      ];
    }
    return [
      buildManchesterPatch(axiomKind, entityIri, other, "add", other),
    ];
  }
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
