import { CatalogSnapshot, DiagnosticSummary, Entity } from "../lsp/protocol";

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

export function classRootEntities(snapshot: CatalogSnapshot): Entity[] {
  const classes = filterEntitiesByKind(snapshot.entities, "class");
  const entityIris = new Set(snapshot.entities.map((e) => e.iri));
  const childSet = new Set(snapshot.hierarchy.edges.map((e) => e.child));

  return classes.filter((c) => {
    if (!childSet.has(c.iri)) {
      return true;
    }
    const parents = snapshot.hierarchy.parents[c.iri] ?? [];
    const hasCatalogParent = parents.some((p) => entityIris.has(p));
    return !hasCatalogParent;
  });
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
  return entity.labels[0] ?? entity.short_name ?? entity.iri;
}

export function childEntitiesForClass(
  snapshot: CatalogSnapshot,
  parentIri: string
): Entity[] {
  const childIris = snapshot.hierarchy.children[parentIri] ?? [];
  return childIris
    .map((iri) => snapshot.entities.find((e) => e.iri === iri))
    .filter((e): e is Entity => !!e);
}
