export function shortLabel(iri: string): string {
  const hash = iri.lastIndexOf("#");
  if (hash >= 0 && hash < iri.length - 1) {
    return iri.slice(hash + 1);
  }
  const slash = iri.lastIndexOf("/");
  if (slash >= 0 && slash < iri.length - 1) {
    return iri.slice(slash + 1);
  }
  return iri;
}

export function entityKindLabel(kind: string): string {
  switch (kind) {
    case "class":
      return "Class";
    case "object_property":
      return "Object Property";
    case "data_property":
      return "Data Property";
    case "annotation_property":
      return "Annotation Property";
    case "individual":
      return "Individual";
    case "ontology":
      return "Ontology";
    default:
      return kind;
  }
}
