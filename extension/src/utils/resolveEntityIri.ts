/** Tree item click passes `arguments: [iri]`; context menus pass the item itself. */
export function resolveEntityIri(arg: unknown): string | undefined {
  if (typeof arg === "string" && arg.length > 0) {
    return arg;
  }
  if (arg && typeof arg === "object" && "iri" in arg) {
    const iri = (arg as { iri?: unknown }).iri;
    if (typeof iri === "string" && iri.length > 0) {
      return iri;
    }
  }
  return undefined;
}
