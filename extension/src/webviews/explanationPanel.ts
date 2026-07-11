import * as vscode from "vscode";
import { focusRelay } from "../focus/focusRelay";
import { getExplanation } from "../lsp/client";
import type { GetExplanationResult } from "../lsp/protocol";
import {
  isExplanationStale,
  resolveExplanationProfile,
} from "./explanationPanelLogic";
import { rememberPanelRestoreState } from "./layoutPersistence";

export class ExplanationPanel {
  public static current: ExplanationPanel | undefined;
  private readonly panel: vscode.WebviewPanel;
  private classIri = "";
  private profile = "el";
  private lastResult: GetExplanationResult | undefined;
  private unsubscribeCatalog?: () => void;

  private constructor(panel: vscode.WebviewPanel) {
    this.panel = panel;
    this.panel.onDidDispose(() => {
      this.unsubscribeCatalog?.();
      ExplanationPanel.current = undefined;
    });
    this.unsubscribeCatalog = focusRelay.subscribeCatalog(() => {
      if (this.lastResult) {
        this.render(this.classIri, this.lastResult, this.profile);
      }
    });
    this.panel.webview.onDidReceiveMessage(async (msg) => {
      if (!msg || typeof msg !== "object" || typeof msg.command !== "string") {
        return;
      }
      if (msg.command === "copy" && typeof msg.text === "string") {
        await vscode.env.clipboard.writeText(msg.text);
      }
      if (msg.command === "rerun") {
        await vscode.commands.executeCommand("ontocode.runReasoner");
        if (this.classIri) {
          await ExplanationPanel.show(this.classIri, this.profile);
        }
      }
      if (msg.command === "openEntity" && typeof (msg as { iri?: unknown }).iri === "string") {
        await vscode.commands.executeCommand("ontocode.openEntity", (msg as { iri: string }).iri);
      }
    });
  }

  public dispose(): void {
    this.panel.dispose();
  }

  public static async show(classIri: string, profileOverride?: string): Promise<void> {
    const cfg = vscode.workspace.getConfiguration("ontocode");
    const profile = resolveExplanationProfile({
      explicit: profileOverride,
      lastRunProfile: focusRelay.getReasoning()?.profile,
      settingsDefault: cfg.get<string>("reasoner.default"),
    });
    const result = await getExplanation({ class_iri: classIri, profile });

    if (ExplanationPanel.current) {
      ExplanationPanel.current.panel.reveal(vscode.ViewColumn.Beside);
      ExplanationPanel.current.setContent(classIri, result, profile);
      return;
    }

    const panel = vscode.window.createWebviewPanel(
      "ontocodeExplanation",
      `Explanation: ${classIri.split(/[#/]/).pop() ?? classIri}`,
      vscode.ViewColumn.Beside,
      { enableScripts: true, retainContextWhenHidden: true }
    );
    const view = new ExplanationPanel(panel);
    ExplanationPanel.current = view;
    view.setContent(classIri, result, profile);
  }

  private setContent(
    classIri: string,
    result: GetExplanationResult,
    profile: string
  ): void {
    this.classIri = classIri;
    this.profile = profile;
    this.lastResult = result;
    void rememberPanelRestoreState("ontocodeExplanation", {
      command: "ontocode.showExplanation",
      args: [classIri, profile],
      title: `Explanation: ${classIri.split(/[#/]/).pop() ?? classIri}`,
    });
    this.render(classIri, result, profile);
  }

  private render(
    classIri: string,
    result: GetExplanationResult,
    profile: string
  ): void {
    const justifications = [
      { title: "Justification 1", steps: result.steps, text: result.text },
      ...(result.alternatives ?? []).map((a, i) => ({
        title: `Justification ${i + 2}`,
        steps: a.steps,
        text: a.text,
      })),
    ];

    const catalog = focusRelay.getCatalogFingerprint();
    const stale = isExplanationStale({
      shownContentHash: result.content_hash,
      shownIndexedAt: result.indexed_at,
      currentContentHash: catalog?.contentHash ?? result.content_hash,
      currentIndexedAt: catalog?.indexedAt ?? result.indexed_at,
    });

    this.panel.webview.html = `<!DOCTYPE html>
<html lang="en"><head><meta charset="UTF-8" />
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'; script-src 'unsafe-inline';" />
<style>
body { font-family: var(--vscode-font-family); padding: 12px; }
pre { white-space: pre-wrap; background: var(--vscode-textBlockQuote-background); padding: 8px; }
a { color: var(--vscode-textLink-foreground); text-decoration: none; }
a:hover { text-decoration: underline; }
.stale { background: var(--vscode-inputValidation-warningBackground); padding: 8px; border-radius: 6px; margin: 8px 0; }
.row { display: flex; gap: 8px; align-items: center; flex-wrap: wrap; }
.muted { opacity: 0.8; font-size: 12px; }
button:focus-visible, select:focus-visible, a:focus-visible { outline: 2px solid var(--vscode-focusBorder); outline-offset: 2px; }
</style></head><body>
<main aria-label="Explanation panel">
<h2 id="explanation-heading">Explanation</h2>
<p><code>${escapeHtml(classIri)}</code></p>
${stale ? `<div class="stale" role="status" aria-live="polite"><strong>Stale explanation</strong><div class="muted">Ontology or reasoner state changed since this explanation was generated. Re-generate to ensure correctness.</div></div>` : ""}
<div class="row">
  <label for="justification">Justification</label>
  <select id="justification" aria-labelledby="explanation-heading"></select>
  <span class="muted">profile=${escapeHtml(profile)} • indexed_at=${result.indexed_at} • content_hash=${escapeHtml(result.content_hash)}</span>
</div>
<ol id="steps" aria-label="Justification steps"></ol>
<pre id="text" aria-label="Explanation text"></pre>
<div class="row" role="toolbar" aria-label="Explanation actions">
  <button id="copy" type="button">Copy</button>
  <button id="rerun" type="button">Re-run Reasoner</button>
</div>
</main>
<script>
const vscode = acquireVsCodeApi();
const justifications = ${JSON.stringify(justifications)};
const select = document.getElementById('justification');
const stepsEl = document.getElementById('steps');
const textEl = document.getElementById('text');

function escapeHtml(s) {
  return String(s).replaceAll('&','&amp;').replaceAll('<','&lt;').replaceAll('>','&gt;');
}

function render(idx) {
  const j = justifications[idx];
  textEl.textContent = j.text;
  stepsEl.innerHTML = '';
  for (const step of j.steps) {
    const li = document.createElement('li');
    const text = step.display ?? '';
    li.innerHTML = escapeHtml(text);
    if (step.subject_iri) {
      const a = document.createElement('a');
      a.href = '#';
      a.textContent = ' subject';
      a.onclick = (e) => { e.preventDefault(); vscode.postMessage({ command: 'openEntity', iri: step.subject_iri }); };
      li.appendChild(document.createTextNode(' '));
      li.appendChild(a);
    }
    if (step.object_iri) {
      const a = document.createElement('a');
      a.href = '#';
      a.textContent = ' object';
      a.onclick = (e) => { e.preventDefault(); vscode.postMessage({ command: 'openEntity', iri: step.object_iri }); };
      li.appendChild(document.createTextNode(' '));
      li.appendChild(a);
    }
    stepsEl.appendChild(li);
  }
}

for (let i = 0; i < justifications.length; i++) {
  const opt = document.createElement('option');
  opt.value = String(i);
  opt.textContent = justifications[i].title;
  select.appendChild(opt);
}
select.onchange = () => render(Number(select.value));
render(0);

document.getElementById('copy').onclick = () => vscode.postMessage({ command: 'copy', text: textEl.textContent });
document.getElementById('rerun').onclick = () => vscode.postMessage({ command: 'rerun' });
</script></body></html>`;
  }
}

function escapeHtml(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}
