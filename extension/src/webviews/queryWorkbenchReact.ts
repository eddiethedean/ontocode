import * as vscode from "vscode";
import { runSqlQuery, runSparqlQuery, listSqlSchema } from "../lsp/client";
import { SavedQuery, TabularQueryResult } from "../lsp/protocol";
import { PanelHost } from "./panelHost";
import type { WebviewMessage } from "./messages";
import {
  parseRunQueryMessage,
  parseSaveQueryMessage,
} from "./messages";
import { rememberPanelRestoreState } from "./layoutPersistence";
import {
  SQL_TABLES,
  exportResultCsv,
  exportResultJson,
  mergeHistory,
  shouldDeliverQueryResult,
  upsertSavedQuery,
} from "./queryWorkbenchLogic";

const SAVED_KEY = "ontocode.savedQueries";
const HISTORY_KEY = "ontocode.queryHistory";
const DEFAULT_HISTORY_LIMIT = 20;

export class QueryWorkbenchPanel {
  public static current: QueryWorkbenchPanel | undefined;
  private host: PanelHost;
  private lastResult: TabularQueryResult | undefined;
  private lastResultRunId = 0;
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

  public dispose(): void {
    this.host.panel.dispose();
  }

  public static async show(context: vscode.ExtensionContext): Promise<QueryWorkbenchPanel> {
    void rememberPanelRestoreState("ontocodeQueryWorkbench", {
      command: "ontocode.openQueryWorkbench",
      title: "OntoCode Query Workbench",
    });
    if (QueryWorkbenchPanel.current) {
      QueryWorkbenchPanel.current.host.panel.reveal(vscode.ViewColumn.Beside);
      await QueryWorkbenchPanel.current.bootstrap();
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
    void instance.bootstrap();
    return instance;
  }

  private async bootstrap(): Promise<void> {
    const saved = this.context.workspaceState.get<SavedQuery[]>(SAVED_KEY) ?? [];
    const history =
      this.context.workspaceState.get<SavedQuery[]>(HISTORY_KEY) ?? [];
    let sqlSchema: Awaited<ReturnType<typeof listSqlSchema>> | undefined;
    try {
      sqlSchema = await listSqlSchema();
    } catch {
      sqlSchema = undefined;
    }
    const sqlTables = sqlSchema?.map((t) => t.name) ?? [...SQL_TABLES];
    this.host.postMessage({
      type: "queryInit",
      saved,
      history,
      sqlTables,
      sqlSchema,
    });
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "runQuery") {
      const parsed = parseRunQueryMessage(message);
      if (!parsed) {
        return;
      }
      await this.runQuery(parsed.mode, parsed.text, parsed.runId);
    }
    if (message.type === "saveQuery") {
      const parsed = parseSaveQueryMessage(message);
      if (!parsed) {
        return;
      }
      await this.saveQuery(parsed.mode, parsed.text, parsed.name);
    }
    if (message.type === "exportQueryResult") {
      await this.exportResult(message.format, message.runId);
    }
  }

  private async runQuery(
    mode: "sql" | "sparql",
    text: string,
    runId: number
  ): Promise<void> {
    this.runId = runId;
    try {
      const result =
        mode === "sql" ? await runSqlQuery(text) : await runSparqlQuery(text);
      if (!shouldDeliverQueryResult(runId, this.runId)) {
        return;
      }
      this.lastResult = result;
      this.lastResultRunId = runId;
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
      await this.bootstrap();
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      if (!shouldDeliverQueryResult(runId, this.runId)) {
        return;
      }
      this.lastResult = undefined;
      this.lastResultRunId = 0;
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
    await this.bootstrap();
    void vscode.window.showInformationMessage(`OntoCode: saved query "${name}"`);
  }

  private async exportResult(format: "csv" | "json", runId?: number): Promise<void> {
    if (!this.lastResult) {
      return;
    }
    if (runId !== undefined && runId !== this.lastResultRunId) {
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

  /** @internal VS Code integration tests */
  getWebviewHtmlForTests(): string {
    return this.host.getWebviewHtml();
  }

  isWebviewReadyForTests(): boolean {
    return this.host.isWebviewReady();
  }

  disposeForTests(): void {
    if (!this.host.isDisposed) {
      this.host.panel.dispose();
    }
  }
}
