import { normalizeFsPath, pathsEqual } from "../utils/pathUnder";

export type OpenDocumentLike = {
  uriString: string;
  fsPath: string;
  isDirty: boolean;
};

/**
 * Prefer exact URI match, then normalized fsPath equality (#299).
 */
export function findOpenDocumentForEntry(
  documents: OpenDocumentLike[],
  entryUri: string,
  entryPath: string
): OpenDocumentLike | undefined {
  const byUri = documents.find((doc) => doc.uriString === entryUri);
  if (byUri) {
    return byUri;
  }
  const target = normalizeFsPath(entryPath);
  return documents.find((doc) => pathsEqual(doc.fsPath, target));
}

/**
 * When no open buffer matches, semantic-dirty may still be clearable after a patch
 * already wrote disk — but it is not a successful file save (#299).
 */
export function noOpenDocumentSaveOutcome(): {
  claimSaved: boolean;
  markClean: boolean;
} {
  return { claimSaved: false, markClean: true };
}
