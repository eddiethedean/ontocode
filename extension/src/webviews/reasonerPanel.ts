import * as vscode from "vscode";
import { runReasoner } from "../lsp/client";
import { RunReasonerResult } from "../lsp/protocol";
import {
  AVAILABLE_PROFILES,
  summarizeResult,
} from "./reasonerPanelLogic";

export class ReasonerPanel {
  public static current: ReasonerPanel | undefined;
  private readonly panel: vscode.WebviewPanel;
  private lastResult: RunReasonerResult | undefined;
  private runId = 0;
  private webviewReady = false;
  private pendingMessages: unknown[] = [];

  private constructor(panel: vscode.WebviewPanel) {
    this.panel = panel;
    this.panel.onDidDispose(() => {
      ReasonerPanel.current = undefined;
    });
    this.panel.webview.onDidReceiveMessage(async (msg) => {
      if (!msg || typeof msg !== "object" || typeof msg.command !== "string") {
        return;
      }
      if (msg.command === "ready") {
        this.webviewReady = true;
        this.flushPending();
        return;
      }
      if (msg.command === "run") {
        const runId =
          typeof msg.runId === "number" ? msg.runId : ++this.runId;
        const profile = typeof msg.profile === "string" ? msg.profile : "el";
        const autoDetect = msg.autoDetect !== false;
        await this.run(profile, autoDetect, runId);
      }
      if (msg.command === "explain" && typeof msg.classIri === "string") {
        await vscode.commands.executeCommand(
          "ontocode.showExplanation",
          msg.classIri
        );
      }
      if (msg.command === "showInferred") {
        await vscode.workspace
          .getConfiguration("ontocode")
          .update("hierarchy.mode", "combined", vscode.ConfigurationTarget.Workspace);
        void vscode.commands.executeCommand("ontocode.refreshExplorer");
      }
    });
    this.panel.webview.html = this.renderHtml();
  }

  private postToWebview(message: unknown): void {
    if (!this.webviewReady) {
      this.pendingMessages.push(message);
      return;
    }
    void this.panel.webview.postMessage(message);
  }

  private flushPending(): void {
    const queued = this.pendingMessages;
    this.pendingMessages = [];
    for (const message of queued) {
      void this.panel.webview.postMessage(message);
    }
  }

  public static show(): ReasonerPanel {
    if (ReasonerPanel.current) {
      ReasonerPanel.current.panel.reveal(vscode.ViewColumn.Beside);
      return ReasonerPanel.current;
    }
    const panel = vscode.window.createWebviewPanel(
      "ontocodeReasoner",
      "OntoCode Reasoner",
      vscode.ViewColumn.Beside,
      { enableScripts: true, retainContextWhenHidden: true }
    );
    ReasonerPanel.current = new ReasonerPanel(panel);
    return ReasonerPanel.current;
  }

  public async runWithDefaults(): Promise<void> {
    const cfg = vscode.workspace.getConfiguration("ontocode");
    const profile = cfg.get<string>("reasoner.default") ?? "el";
    const autoDetect = cfg.get<boolean>("reasoner.autoProfile") ?? true;
    await this.run(profile, autoDetect, ++this.runId);
  }

  private async run(profile: string, autoDetect: boolean, runId: number): Promise<void> {
    this.runId = runId;
    this.postToWebview({ command: "syncRunId", runId });
    try {
      const result = await runReasoner({ profile, auto_detect: autoDetect });
      if (runId !== this.runId) {
        return;
      }
      this.lastResult = result;
      this.postToWebview({
        command: "result",
        runId,
        result,
        summary: summarizeResult(result),
        error: undefined,
      });
      void vscode.commands.executeCommand("ontocode.refreshExplorer");
    } catch (err) {
      if (runId !== this.runId) {
        return;
      }
      const message = err instanceof Error ? err.message : String(err);
      this.postToWebview({
        command: "result",
        runId,
        result: undefined,
        error: message,
      });
    }
  }

  private renderHtml(): string {
    const profiles = AVAILABLE_PROFILES.map(
      (p) =>
        `<option value="${p.id}" ${p.enabled ? "" : "disabled"} title="${p.hint ?? ""}">${p.label}${p.hint ? ` (${p.hint})` : ""}</option>`
    ).join("");
    return `<!DOCTYPE html>
<html><head><meta charset="UTF-8" />
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'; script-src 'unsafe-inline';" />
<style>
body { font-family: var(--vscode-font-family); padding: 12px; color: var(--vscode-foreground); }
button { margin: 4px 4px 4px 0; }
select { margin-right: 8px; }
#status { margin: 8px 0; font-weight: 600; }
.error { color: var(--vscode-errorForeground); }
section { margin-top: 12px; }
ul { padding-left: 18px; }
li button { font-size: 0.9em; }
</style></head><body>
<h2>Reasoner</h2>
<label>Profile <select id="profile">${profiles}</select></label>
<label><input type="checkbox" id="auto" checked /> Profile warnings</label>
<button id="run">Run Reasoner</button>
<button id="inferred">Show Inferred Hierarchy</button>
<div id="status"></div>
<section><h3>Unsatisfiable</h3><ul id="unsat"></ul></section>
<section><h3>Inferred Changes</h3><ul id="inferences"></ul></section>
<section><h3>Warnings</h3><ul id="warnings"></ul></section>
<script>
const vscode = acquireVsCodeApi();
vscode.postMessage({ command: 'ready' });
document.getElementById('run').onclick = () => {
  window.latestRunId = (window.latestRunId || 0) + 1;
  vscode.postMessage({
    command: 'run',
    profile: document.getElementById('profile').value,
    autoDetect: document.getElementById('auto').checked,
    runId: window.latestRunId,
  });
};
document.getElementById('inferred').onclick = () => vscode.postMessage({ command: 'showInferred' });
window.addEventListener('message', (e) => {
  const msg = e.data;
  if (msg.command === 'syncRunId' && typeof msg.runId === 'number') {
    window.latestRunId = msg.runId;
    return;
  }
  if (msg.command !== 'result') return;
  if (msg.runId != null && msg.runId !== window.latestRunId) return;
  const status = document.getElementById('status');
  if (msg.error) {
    status.textContent = msg.error;
    status.className = 'error';
    document.getElementById('unsat').innerHTML = '';
    document.getElementById('inferences').innerHTML = '';
    document.getElementById('warnings').innerHTML = '';
    return;
  }
  status.className = '';
  status.textContent = msg.summary || 'Done';
  const unsat = document.getElementById('unsat');
  unsat.innerHTML = '';
  for (const iri of (msg.result?.unsatisfiable || [])) {
    const li = document.createElement('li');
    const btn = document.createElement('button');
    btn.textContent = iri;
    btn.onclick = () => vscode.postMessage({ command: 'explain', classIri: iri });
    li.appendChild(btn);
    unsat.appendChild(li);
  }
  const inf = document.getElementById('inferences');
  inf.innerHTML = '';
  for (const edge of (msg.result?.new_inferences || [])) {
    const li = document.createElement('li');
    li.textContent = edge.child + ' SubClassOf ' + edge.parent;
    inf.appendChild(li);
  }
  const warn = document.getElementById('warnings');
  warn.innerHTML = '';
  for (const w of (msg.result?.warnings || [])) {
    const li = document.createElement('li');
    li.textContent = w.message;
    warn.appendChild(li);
  }
});
</script></body></html>`;
  }
}
