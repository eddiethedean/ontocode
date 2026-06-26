import * as vscode from "vscode";
import type { LspWorkspaceEdit } from "./protocol";

/** Apply a language-server `WorkspaceEdit` to open VS Code editors. */
export async function applyLspWorkspaceEdit(
  edit: LspWorkspaceEdit | undefined
): Promise<boolean> {
  if (!edit?.document_changes?.length) {
    return true;
  }
  const workspaceEdit = new vscode.WorkspaceEdit();
  for (const docChange of edit.document_changes) {
    const uri = vscode.Uri.parse(docChange.text_document.uri);
    const document = await vscode.workspace.openTextDocument(uri);
    for (const rawEdit of docChange.edits) {
      const range = rawEdit.range;
      const endLine =
        range.end.line >= 0xfffffff0 ? document.lineCount : range.end.line;
      const endCharacter =
        range.end.line >= 0xfffffff0 ? 0 : range.end.character;
      const vscodeRange = new vscode.Range(
        range.start.line,
        range.start.character,
        endLine,
        endCharacter
      );
      const newText =
        (rawEdit as { new_text?: string; newText?: string }).new_text ??
        (rawEdit as { newText?: string }).newText ??
        "";
      workspaceEdit.replace(uri, vscodeRange, newText);
    }
  }
  return vscode.workspace.applyEdit(workspaceEdit);
}
