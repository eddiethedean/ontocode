import * as vscode from "vscode";
import type { OntologyDocument } from "../lsp/protocol";
import {
  documentUriInWorkspace,
  isPathUnderFolder,
  isUriInWorkspace,
  normalizeFsPath,
} from "./workspacePath";

export interface ResolveTurtleFilePathOptions {
  /** Explicit path from tree item or session restore. */
  filePath?: string;
  /** Indexed Turtle documents (catalog snapshot). */
  turtleDocuments?: OntologyDocument[];
}

/** Pick a workspace Turtle file for Imports / palette entry points. */
export async function resolveTurtleFilePath(
  options: ResolveTurtleFilePathOptions
): Promise<string | undefined> {
  if (options.filePath) {
    return documentUriInWorkspace(options.filePath)
      ? normalizeFsPath(options.filePath)
      : undefined;
  }

  const activeEditor = vscode.window.activeTextEditor;
  if (activeEditor) {
    const doc = activeEditor.document;
    if (
      (doc.languageId === "turtle" || doc.fileName.endsWith(".ttl")) &&
      isUriInWorkspace(doc.uri)
    ) {
      return doc.uri.fsPath;
    }
  }

  let ttlDocs = (options.turtleDocuments ?? []).filter(
    (d) => d.format === "turtle" && documentUriInWorkspace(d.path) !== undefined
  );

  if (activeEditor) {
    const folder = vscode.workspace.getWorkspaceFolder(activeEditor.document.uri);
    if (folder) {
      const prefix = folder.uri.fsPath;
      const inFolder = ttlDocs.filter((d) => isPathUnderFolder(d.path, prefix));
      if (inFolder.length > 0) {
        ttlDocs = inFolder;
      }
    }
  }

  if (ttlDocs.length === 0) {
    return undefined;
  }
  if (ttlDocs.length === 1) {
    return normalizeFsPath(ttlDocs[0].path);
  }

  const pick = await vscode.window.showQuickPick(
    ttlDocs.map((d) => ({ label: d.path, path: d.path })),
    { placeHolder: "Select Turtle ontology file" }
  );
  return pick?.path ? normalizeFsPath(pick.path) : undefined;
}
