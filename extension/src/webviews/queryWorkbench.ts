import * as vscode from "vscode";
import { runSqlQuery, runSparqlQuery } from "../lsp/client";
import { TabularQueryResult, SavedQuery } from "../lsp/protocol";
import {
  STARTER_SPARQL,
  STARTER_SQL,
  SQL_TABLES,
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
  private readonly panel: vscode.WebviewPanel;
  private lastResult: TabularQueryResult | undefined;
  private runId = 0;

  private constructor(
    panel: vscode.WebviewPanel,
    private readonly context: vscode.ExtensionContext
  ) {
    this.panel = panel;
    this.panel.onDidDispose(() => {
      QueryWorkbenchPanel.current = undefined;
    });
    this.panel.webview.onDidReceiveMessage(async (msg) => {
      if (msg.command === "run") {
        const runId =
          typeof msg.runId === "number" ? msg.runId : ++this.runId;
        await this.runQuery(msg.mode as "sql" | "sparql", msg.text as string, runId);
      }
      if (msg.command === "save") {
        await this.saveQuery(msg.mode, msg.text, msg.name);
      }
      if (msg.command === "export") {
        await this.exportResult(msg.format as "csv" | "json");
      }
    });
    this.panel.webview.html = this.renderHtml();
  }

  public static show(context: vscode.ExtensionContext): QueryWorkbenchPanel {
    if (QueryWorkbenchPanel.current) {
      QueryWorkbenchPanel.current.panel.reveal(vscode.ViewColumn.Beside);
      return QueryWorkbenchPanel.current;
    }
    const panel = vscode.window.createWebviewPanel(
      "ontocodeQueryWorkbench",
      "OntoCode Query Workbench",
      vscode.ViewColumn.Beside,
      { enableScripts: true, retainContextWhenHidden: true }
    );
    QueryWorkbenchPanel.current = new QueryWorkbenchPanel(panel, context);
    return QueryWorkbenchPanel.current;
  }

  private async runQuery(
    mode: "sql" | "sparql",
    text: string,
    runId: number
  ): Promise<void> {
    try {
      const result =
        mode === "sql"
          ? await runSqlQuery(text)
          : await runSparqlQuery(text);
      if (runId !== this.runId) {
        return;
      }
      this.lastResult = result;
      this.panel.webview.postMessage({
        command: "result",
        runId,
        result,
        error: undefined,
      });
      const history = mergeHistory(
        this.context.workspaceState.get<SavedQuery[]>(HISTORY_KEY) ?? [],
        { name: `${mode} @ ${new Date().toLocaleTimeString()}`, mode, text },
        vscode.workspace
          .getConfiguration("ontocode")
          .get<number>("queryHistoryLimit", DEFAULT_HISTORY_LIMIT)
      );
      await this.context.workspaceState.update(HISTORY_KEY, history);
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      this.panel.webview.postMessage({
        command: "result",
        runId,
        error: message,
      });
    }
  }

  private async saveQuery(
    mode: "sql" | "sparql",
    text: string,
    name?: string
  ): Promise<void> {
    const queryName =
      name?.trim() ||
      (await vscode.window.showInputBox({ prompt: "Query name" }));
    if (!queryName) {
      return;
    }
    const saved = upsertSavedQuery(
      this.context.workspaceState.get<SavedQuery[]>(SAVED_KEY) ?? [],
      { name: queryName, mode, text }
    );
    await this.context.workspaceState.update(SAVED_KEY, saved);
    this.panel.webview.postMessage({ command: "saved", saved });
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

  private renderHtml(): string {
    const saved =
      this.context.workspaceState.get<SavedQuery[]>(SAVED_KEY) ?? [];
    const history =
      this.context.workspaceState.get<SavedQuery[]>(HISTORY_KEY) ?? [];
    const tables = SQL_TABLES.map((t) => `<option value="${t}">${t}</option>`).join(
      ""
    );
    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <style>
    body { font-family: var(--vscode-font-family); color: var(--vscode-foreground); background: var(--vscode-editor-background); padding: 12px; margin: 0; }
    textarea { width: 100%; min-height: 140px; font-family: var(--vscode-editor-font-family); box-sizing: border-box; }
    table { border-collapse: collapse; width: 100%; font-size: 0.85rem; margin-top: 8px; }
    th, td { border: 1px solid var(--vscode-panel-border); padding: 4px 6px; text-align: left; }
    th { background: var(--vscode-editor-inactiveSelectionBackground); }
    .toolbar { display: flex; gap: 8px; flex-wrap: wrap; align-items: center; margin: 8px 0; }
    button, select { background: var(--vscode-button-background); color: var(--vscode-button-foreground); border: none; padding: 4px 10px; cursor: pointer; }
    .error { color: var(--vscode-errorForeground); margin-top: 8px; }
    .banner { color: var(--vscode-descriptionForeground); font-size: 0.85rem; margin-top: 4px; }
    .results { max-height: 360px; overflow: auto; }
  </style>
</head>
<body>
  <h2>Query Workbench</h2>
  <div class="toolbar">
    <label>Mode <select id="mode"><option value="sql">SQL</option><option value="sparql">SPARQL</option></select></label>
    <label>Table <select id="tablePick"><option value="">—</option>${tables}</select></label>
    <button id="run">Run</button>
    <button id="save">Save Query</button>
    <button id="exportCsv">Export CSV</button>
    <button id="exportJson">Export JSON</button>
  </div>
  <textarea id="query">${escapeHtml(STARTER_SQL)}</textarea>
  <div class="toolbar">
    <label>Saved <select id="savedPick"><option value="">—</option>${saved.map((s) => `<option value="${escapeHtml(JSON.stringify(s))}">${escapeHtml(s.name)} (${s.mode})</option>`).join("")}</select></label>
    <label>History <select id="historyPick"><option value="">—</option>${history.map((h) => `<option value="${escapeHtml(JSON.stringify(h))}">${escapeHtml(h.name)}</option>`).join("")}</select></label>
  </div>
  <div id="error" class="error"></div>
  <div id="banner" class="banner"></div>
  <div id="results" class="results"></div>
  <script>
    const vscode = acquireVsCodeApi();
    const queryEl = document.getElementById('query');
    const modeEl = document.getElementById('mode');
    const tablePick = document.getElementById('tablePick');
    const starterSql = ${JSON.stringify(STARTER_SQL)};
    const starterSparql = ${JSON.stringify(STARTER_SPARQL)};

    modeEl.addEventListener('change', () => {
      queryEl.value = modeEl.value === 'sql' ? starterSql : starterSparql;
    });

    tablePick.addEventListener('change', () => {
      if (!tablePick.value || modeEl.value !== 'sql') return;
      queryEl.value = 'SELECT * FROM ' + tablePick.value;
    });

    function loadPick(selectEl) {
      const raw = selectEl.value;
      if (!raw) return;
      const item = JSON.parse(raw);
      modeEl.value = item.mode;
      queryEl.value = item.text;
    }
    document.getElementById('savedPick').addEventListener('change', (e) => loadPick(e.target));
    document.getElementById('historyPick').addEventListener('change', (e) => loadPick(e.target));

    document.getElementById('run').addEventListener('click', () => {
      window.latestRunId = (window.latestRunId || 0) + 1;
      vscode.postMessage({ command: 'run', mode: modeEl.value, text: queryEl.value, runId: window.latestRunId });
    });
    document.getElementById('save').addEventListener('click', () => {
      vscode.postMessage({ command: 'save', mode: modeEl.value, text: queryEl.value });
    });
    document.getElementById('exportCsv').addEventListener('click', () => vscode.postMessage({ command: 'export', format: 'csv' }));
    document.getElementById('exportJson').addEventListener('click', () => vscode.postMessage({ command: 'export', format: 'json' }));

    window.addEventListener('message', (event) => {
      const msg = event.data;
      if (msg.command === 'result') {
        if (msg.runId != null && msg.runId !== window.latestRunId) return;
        document.getElementById('error').textContent = msg.error || '';
        const banner = document.getElementById('banner');
        const results = document.getElementById('results');
        if (!msg.result) { results.textContent = ''; banner.textContent = ''; return; }
        banner.textContent = msg.result.truncated ? 'Results truncated at server row limit.' : '';
        const cols = msg.result.columns;
        const rows = msg.result.rows;
        results.replaceChildren();
        const table = document.createElement('table');
        const thead = document.createElement('thead');
        const headerRow = document.createElement('tr');
        for (const c of cols) {
          const th = document.createElement('th');
          th.textContent = c;
          headerRow.appendChild(th);
        }
        thead.appendChild(headerRow);
        table.appendChild(thead);
        const tbody = document.createElement('tbody');
        for (const row of rows) {
          const tr = document.createElement('tr');
          for (const c of cols) {
            const td = document.createElement('td');
            const val = row[c];
            td.textContent = val == null ? '' : String(val);
            tr.appendChild(td);
          }
          tbody.appendChild(tr);
        }
        table.appendChild(tbody);
        results.appendChild(table);
      }
    });
  </script>
</body>
</html>`;
  }
}

function escapeHtml(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}
