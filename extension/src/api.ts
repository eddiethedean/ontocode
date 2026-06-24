import { LanguageClient } from "vscode-languageclient/node";
import {
  CatalogSnapshot,
  GetEntityResult,
  IndexWorkspaceResult,
  ParseManchesterResult,
  TabularQueryResult,
} from "./lsp/protocol";

/** Extension activation API (used by VS Code integration tests). */
export interface OntoCodeApi {
  getClient(): LanguageClient | undefined;
  indexWorkspace(workspaceUri?: string): Promise<IndexWorkspaceResult>;
  getCatalogSnapshot(): Promise<CatalogSnapshot>;
  getEntity(iri: string): Promise<GetEntityResult>;
  runSqlQuery(sql: string): Promise<TabularQueryResult>;
  runSparqlQuery(query: string): Promise<TabularQueryResult>;
  parseManchester(
    expression: string,
    axiomKind: string,
    entityIri?: string,
    documentUri?: string
  ): Promise<ParseManchesterResult>;
}
