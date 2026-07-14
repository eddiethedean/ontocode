import {
  CatalogSnapshot,
  ClassHierarchy,
  DiagnosticSummary,
  Entity,
  HierarchyMode,
  SubclassEdge,
} from "../lsp/protocol";

export function hierarchyModeFromConfig(mode: string | undefined): HierarchyMode {
  if (mode === "inferred" || mode === "combined") {
    return mode;
  }
  return "asserted";
}

export function activeHierarchy(snapshot: CatalogSnapshot, mode: HierarchyMode): ClassHierarchy {
  if (mode === "asserted") {
    return snapshot.hierarchy;
  }
  if (!snapshot.reasoner) {
    return snapshot.hierarchy;
  }
  if (mode === "inferred") {
    return hierarchyFromEdges(snapshot.reasoner.inferred.edges);
  }
  return snapshot.reasoner.inferred.combined;
}

export function hierarchyFromEdges(edges: SubclassEdge[]): ClassHierarchy {
  const parents: Record<string, string[]> = {};
  const children: Record<string, string[]> = {};
  for (const edge of edges) {
    parents[edge.child] = parents[edge.child] ?? [];
    parents[edge.child].push(edge.parent);
    children[edge.parent] = children[edge.parent] ?? [];
    children[edge.parent].push(edge.child);
  }
  return { edges, parents, children };
}

export function isInferredOnlyEdge(
  snapshot: CatalogSnapshot,
  child: string,
  parent: string
): boolean {
  if (!snapshot.reasoner) {
    return false;
  }
  return snapshot.reasoner.inferred.edges.some(
    (e) => e.child === child && e.parent === parent
  );
}

export function isUnsatisfiable(snapshot: CatalogSnapshot, iri: string): boolean {
  return snapshot.reasoner?.unsatisfiable.includes(iri) ?? false;
}

export function filterEntitiesByKind(entities: Entity[], kind: string): Entity[] {
  return entities.filter((e) => e.kind === kind);
}

export function diagnosticLabel(diag: DiagnosticSummary): string {
  const loc =
    diag.line != null
      ? `:${diag.line}${diag.column != null ? `:${diag.column}` : ""}`
      : "";
  const base = diag.entity_iri ?? diag.code;
  return `${base}${loc} — ${diag.message}`;
}

export function groupDiagnosticsBySeverity(
  diagnostics: DiagnosticSummary[]
): Map<string, DiagnosticSummary[]> {
  const groups = new Map<string, DiagnosticSummary[]>();
  for (const diag of diagnostics) {
    const key = diag.severity;
    const list = groups.get(key) ?? [];
    list.push(diag);
    groups.set(key, list);
  }
  return groups;
}

export function classRootEntities(snapshot: CatalogSnapshot, mode: HierarchyMode = "asserted"): Entity[] {
  const hierarchy = activeHierarchy(snapshot, mode);
  const classes = filterEntitiesByKind(snapshot.entities, "class");
  const entityIris = new Set(snapshot.entities.map((e) => e.iri));
  const childSet = new Set(hierarchy.edges.map((e) => e.child));

  const roots = classes.filter((c) => {
    if (!childSet.has(c.iri)) {
      return true;
    }
    const parents = hierarchy.parents[c.iri] ?? [];
    const hasCatalogParent = parents.some((p) => entityIris.has(p));
    return !hasCatalogParent;
  });
  // Pure SubClassOf cycles leave every class with a parent — fall back so classes
  // remain discoverable under hierarchy roots (#222).
  if (roots.length === 0 && classes.length > 0) {
    return [...classes].sort((a, b) => a.iri.localeCompare(b.iri));
  }
  return roots;
}

export function propertyGroupsPresent(
  snapshot: CatalogSnapshot
): Array<{ kind: string; label: string }> {
  const groups = [
    { kind: "object_property", label: "Object Properties" },
    { kind: "data_property", label: "Data Properties" },
    { kind: "annotation_property", label: "Annotation Properties" },
  ];
  return groups.filter(
    ({ kind }) => filterEntitiesByKind(snapshot.entities, kind).length > 0
  );
}

export function entityDisplayLabel(entity: Entity): string {
  const label = entity.labels[0] ?? entity.short_name ?? entity.iri;
  if (entity.obo_id) {
    return `${entity.obo_id} — ${label}`;
  }
  return label;
}

export function childEntitiesForClass(
  snapshot: CatalogSnapshot,
  parentIri: string,
  mode: HierarchyMode = "asserted"
): Entity[] {
  const hierarchy = activeHierarchy(snapshot, mode);
  const childIris = hierarchy.children[parentIri] ?? [];
  return childIris
    .map((iri) => snapshot.entities.find((e) => e.iri === iri))
    .filter((e): e is Entity => !!e);
}
