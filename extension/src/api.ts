import { LanguageClient } from "vscode-languageclient/node";
import {
  CatalogSnapshot,
  GetEntityResult,
  IndexWorkspaceResult,
  ParseManchesterResult,
  TabularQueryResult,
} from "./lsp/protocol";
import type { PanelKind } from "./webviews/messages";

/** VS Code integration test hooks (only when ONTOCODE_TEST_FIXTURES is set). */
export interface OntoCodeTestHooks {
  openEntityInspector(iri: string): Promise<void>;
  getInspectorWebviewHtml(): string | undefined;
  assertInspectorHtmlRoutesPanel(): void;
  waitForInspectorReady(timeoutMs?: number): Promise<void>;
  openQueryWorkbench(): Promise<void>;
  getQueryWorkbenchWebviewHtml(): string | undefined;
  assertQueryWorkbenchHtmlRoutesPanel(): void;
  waitForQueryWorkbenchReady(timeoutMs?: number): Promise<void>;
  disposeAllPanels(): Promise<void>;
  assertWebviewHtmlRoutesPanel(html: string, panel: PanelKind): void;
  openEntity(iri: string): Promise<void>;
  getInspectorLoadedIri(): string | undefined;
  getInspectorPanelTitle(): string | undefined;
  waitForInspectorIri(iri: string, timeoutMs?: number): Promise<void>;
  getInspectorPanelRef(): object | undefined;
}

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
  /** Present when ONTOCODE_TEST_FIXTURES is set. */
  __test?: OntoCodeTestHooks;
}
