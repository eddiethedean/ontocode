import * as vscode from "vscode";
import { getExplanation } from "../lsp/client";

export class ExplanationPanel {
  public static current: ExplanationPanel | undefined;
  private readonly panel: vscode.WebviewPanel;

  private constructor(panel: vscode.WebviewPanel) {
    this.panel = panel;
    this.panel.onDidDispose(() => {
      ExplanationPanel.current = undefined;
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
      }
    });
  }

  public static async show(classIri: string): Promise<void> {
    const cfg = vscode.workspace.getConfiguration("ontocode");
    const profile = cfg.get<string>("reasoner.default") ?? "el";
    const result = await getExplanation({ class_iri: classIri, profile });

    if (ExplanationPanel.current) {
      ExplanationPanel.current.panel.reveal(vscode.ViewColumn.Beside);
      ExplanationPanel.current.setContent(classIri, result.text, result.steps);
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
    view.setContent(classIri, result.text, result.steps);
  }

  private setContent(
    classIri: string,
    text: string,
    steps: Array<{ index: number; display: string }>
  ): void {
    const stepLines = steps
      .map((s) => `<li>${s.index}. ${escapeHtml(s.display)}</li>`)
      .join("");
    this.panel.webview.html = `<!DOCTYPE html>
<html><head><meta charset="UTF-8" />
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'; script-src 'unsafe-inline';" />
<style>
body { font-family: var(--vscode-font-family); padding: 12px; }
pre { white-space: pre-wrap; background: var(--vscode-textBlockQuote-background); padding: 8px; }
</style></head><body>
<h2>Explanation</h2>
<p><code>${escapeHtml(classIri)}</code></p>
${text.startsWith("DL profile") ? '<p class="profile"><strong>DL justification</strong> (EL/RL trace fallback)</p>' : text.startsWith("EL ") ? '<p class="profile"><strong>EL justification</strong></p>' : ""}
<ol>${stepLines}</ol>
<pre id="text">${escapeHtml(text)}</pre>
<button id="copy">Copy</button>
<button id="rerun">Re-run Reasoner</button>
<script>
const vscode = acquireVsCodeApi();
const text = document.getElementById('text').textContent;
document.getElementById('copy').onclick = () => vscode.postMessage({ command: 'copy', text });
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
