import * as vscode from "vscode";
import { runSqlQuery, runSparqlQuery } from "../lsp/client";
import { SavedQuery, TabularQueryResult } from "../lsp/protocol";
import { PanelHost } from "./panelHost";
import type { WebviewMessage } from "./messages";
import {
  SQL_TABLES,
  STARTER_SPARQL,
  STARTER_SQL,
  exportResultCsv,
  exportResultJson,
  mergeHistory,
  upsertSavedQuery,
} from "./queryWorkbenchLogic";

const SAVED_KEY = "ontocode.savedQueries";
const HISTORY_KEY = "ontocode.queryHistory";
const DEFAULT_HISTORY_LIMIT = 20;

export class QueryWorkbenchPanel {
  public static current: QueryWorkbenchPanel | undefined;
  private host: PanelHost;
  private lastResult: TabularQueryResult | undefined;
  private runId = 0;

  private constructor(
    host: PanelHost,
    private readonly context: vscode.ExtensionContext
  ) {
    this.host = host;
    host.panel.onDidDispose(() => {
      QueryWorkbenchPanel.current = undefined;
    });
  }

  public static show(context: vscode.ExtensionContext): QueryWorkbenchPanel {
    if (QueryWorkbenchPanel.current) {
      QueryWorkbenchPanel.current.host.panel.reveal(vscode.ViewColumn.Beside);
      QueryWorkbenchPanel.current.bootstrap();
      return QueryWorkbenchPanel.current;
    }
    const host = PanelHost.create(context.extensionUri, {
      viewType: "ontocodeQueryWorkbench",
      title: "OntoCode Query Workbench",
      panel: "queryWorkbench",
      onMessage: async (message: WebviewMessage) => {
        const panel = QueryWorkbenchPanel.current;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });
    const instance = new QueryWorkbenchPanel(host, context);
    QueryWorkbenchPanel.current = instance;
    instance.bootstrap();
    return instance;
  }

  private bootstrap(): void {
    const saved = this.context.workspaceState.get<SavedQuery[]>(SAVED_KEY) ?? [];
    const history =
      this.context.workspaceState.get<SavedQuery[]>(HISTORY_KEY) ?? [];
    this.host.postMessage({
      type: "queryInit",
      saved,
      history,
      sqlTables: [...SQL_TABLES],
    });
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "runQuery") {
      await this.runQuery(message.mode, message.text, message.runId);
    }
    if (message.type === "saveQuery") {
      await this.saveQuery(message.mode, message.text, message.name);
    }
    if (message.type === "exportQueryResult") {
      await this.exportResult(message.format);
    }
  }

  private async runQuery(
    mode: "sql" | "sparql",
    text: string,
    runId: number
  ): Promise<void> {
    try {
      const result =
        mode === "sql" ? await runSqlQuery(text) : await runSparqlQuery(text);
      if (runId !== this.runId) {
        return;
      }
      this.lastResult = result;
      this.host.postMessage({
        type: "queryResult",
        runId,
        result,
      });
      const history = mergeHistory(
        this.context.workspaceState.get<SavedQuery[]>(HISTORY_KEY) ?? [],
        { name: `${mode} @ ${new Date().toLocaleTimeString()}`, mode, text },
        vscode.workspace
          .getConfiguration("ontocode")
          .get<number>("queryHistoryLimit", DEFAULT_HISTORY_LIMIT)
      );
      await this.context.workspaceState.update(HISTORY_KEY, history);
      this.bootstrap();
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      this.host.postMessage({ type: "queryResult", runId, error: msg });
    }
  }

  private async saveQuery(
    mode: "sql" | "sparql",
    text: string,
    name: string
  ): Promise<void> {
    const saved = upsertSavedQuery(
      this.context.workspaceState.get<SavedQuery[]>(SAVED_KEY) ?? [],
      { name, mode, text }
    );
    await this.context.workspaceState.update(SAVED_KEY, saved);
    this.bootstrap();
    void vscode.window.showInformationMessage(`OntoCode: saved query "${name}"`);
  }

  private async exportResult(format: "csv" | "json"): Promise<void> {
    if (!this.lastResult) {
      return;
    }
    const body =
      format === "csv"
        ? exportResultCsv(this.lastResult)
        : exportResultJson(this.lastResult);
    await vscode.env.clipboard.writeText(body);
    void vscode.window.showInformationMessage(
      `OntoCode: ${format.toUpperCase()} copied to clipboard`
    );
  }
}
