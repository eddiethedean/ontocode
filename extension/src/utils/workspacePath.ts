import * as vscode from "vscode";

/** True when a file URI belongs to an open workspace folder. */
export function isUriInWorkspace(uri: vscode.Uri): boolean {
  if (uri.scheme !== "file") {
    return false;
  }
  return vscode.workspace.getWorkspaceFolder(uri) !== undefined;
}

/** Open a file only when it lies inside the workspace; show an error otherwise. */
export async function openWorkspaceTextDocument(
  filePath: string
): Promise<vscode.TextDocument | undefined> {
  const uri = vscode.Uri.file(filePath);
  if (!isUriInWorkspace(uri)) {
    void vscode.window.showErrorMessage(
      "OntoCode: path is outside the workspace"
    );
    return undefined;
  }
  return vscode.workspace.openTextDocument(uri);
}

/** Validate a document path from the language server before editing. */
export function documentUriInWorkspace(documentPath: string): string | undefined {
  const uri = vscode.Uri.file(documentPath);
  if (!isUriInWorkspace(uri)) {
    return undefined;
  }
  return uri.toString();
}
