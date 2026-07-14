/** LSP error helpers that must remain vscode-free for unit tests. */

/** Catalog / snapshot RPCs throw this before the first index finishes (#294). */
export function isNotIndexedError(err: unknown): boolean {
  return err instanceof Error && /^NOT_INDEXED\b/.test(err.message);
}
