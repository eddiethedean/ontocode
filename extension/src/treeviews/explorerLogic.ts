import { CatalogSnapshot, Entity } from "../lsp/protocol";

export function filterEntitiesByKind(entities: Entity[], kind: string): Entity[] {
  return entities.filter((e) => e.kind === kind);
}

export function classRootEntities(snapshot: CatalogSnapshot): Entity[] {
  const classes = filterEntitiesByKind(snapshot.entities, "class");
  const childSet = new Set(snapshot.hierarchy.edges.map((e) => e.child));
  return classes.filter((c) => !childSet.has(c.iri));
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
