import { LanguageClient } from "vscode-languageclient/node";
import { CatalogSnapshot, IndexWorkspaceResult } from "./lsp/protocol";

/** Extension activation API (used by VS Code integration tests). */
export interface OntoCodeApi {
  getClient(): LanguageClient | undefined;
  indexWorkspace(workspaceUri?: string): Promise<IndexWorkspaceResult>;
  getCatalogSnapshot(): Promise<CatalogSnapshot>;
}
