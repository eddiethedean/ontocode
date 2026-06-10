import * as vscode from "vscode";
import { EntityDetail } from "../lsp/protocol";
import { entityKindLabel, shortLabel } from "../utils/iri";

export class EntityInspectorPanel {
  public static currentPanel: EntityInspectorPanel | undefined;
  private readonly panel: vscode.WebviewPanel;
  private iri: string | undefined;

  private constructor(
    panel: vscode.WebviewPanel,
    private readonly extensionUri: vscode.Uri
  ) {
    this.panel = panel;
    this.panel.onDidDispose(() => {
      EntityInspectorPanel.currentPanel = undefined;
    });
    this.panel.webview.onDidReceiveMessage(async (message) => {
      if (message.command === "jumpToSource" && this.iri) {
        await vscode.commands.executeCommand(
          "ontocode.jumpToSource",
          this.iri
        );
      }
    });
  }

  public static show(
    extensionUri: vscode.Uri,
    detail: EntityDetail
  ): EntityInspectorPanel {
    if (EntityInspectorPanel.currentPanel) {
      EntityInspectorPanel.currentPanel.panel.reveal(
        vscode.ViewColumn.Beside
      );
      EntityInspectorPanel.currentPanel.update(detail);
      return EntityInspectorPanel.currentPanel;
    }

    const panel = vscode.window.createWebviewPanel(
      "ontocodeInspector",
      `Entity: ${detail.entity.short_name || shortLabel(detail.entity.iri)}`,
      vscode.ViewColumn.Beside,
      { enableScripts: true, retainContextWhenHidden: true }
    );

    EntityInspectorPanel.currentPanel = new EntityInspectorPanel(
      panel,
      extensionUri
    );
    EntityInspectorPanel.currentPanel.update(detail);
    return EntityInspectorPanel.currentPanel;
  }

  public update(detail: EntityDetail): void {
    this.iri = detail.entity.iri;
    this.panel.title = `${entityKindLabel(detail.entity.kind)}: ${
      detail.entity.labels[0] ?? detail.entity.short_name
    }`;
    this.panel.webview.html = this.renderHtml(detail);
  }

  private renderHtml(detail: EntityDetail): string {
    const { entity, parents, children, axioms } = detail;
    const list = (items: string[]) =>
      items.length > 0
        ? `<ul>${items.map((i) => `<li><code>${escapeHtml(shortLabel(i))}</code> <span class="muted">${escapeHtml(i)}</span></li>`).join("")}</ul>`
        : `<p class="muted">None</p>`;

    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <style>
    body { font-family: var(--vscode-font-family); color: var(--vscode-foreground); background: var(--vscode-editor-background); padding: 16px; }
    h1 { font-size: 1.2rem; margin-bottom: 0.25rem; }
    h2 { font-size: 0.95rem; margin-top: 1.25rem; border-bottom: 1px solid var(--vscode-panel-border); padding-bottom: 4px; }
    code { font-family: var(--vscode-editor-font-family); }
    .muted { color: var(--vscode-descriptionForeground); font-size: 0.85rem; }
    .iri { word-break: break-all; }
    button { background: var(--vscode-button-background); color: var(--vscode-button-foreground); border: none; padding: 6px 12px; cursor: pointer; margin-top: 12px; }
    button:hover { background: var(--vscode-button-hoverBackground); }
    .deprecated { color: var(--vscode-errorForeground); font-weight: bold; }
  </style>
</head>
<body>
  <h1>${escapeHtml(entity.labels[0] ?? entity.short_name)}</h1>
  <p class="muted">${escapeHtml(entityKindLabel(entity.kind))}${entity.deprecated ? ' <span class="deprecated">(deprecated)</span>' : ""}</p>

  <h2>IRI</h2>
  <p class="iri"><code>${escapeHtml(entity.iri)}</code></p>

  <h2>Labels</h2>
  ${list(entity.labels)}

  <h2>Comments</h2>
  ${list(entity.comments)}

  <h2>Parents</h2>
  ${list(parents)}

  <h2>Children</h2>
  ${list(children)}

  <h2>Axioms</h2>
  ${list(axioms)}

  <button id="jump">Jump to Source</button>
  <script>
    const vscode = acquireVsCodeApi();
    document.getElementById('jump').addEventListener('click', () => {
      vscode.postMessage({ command: 'jumpToSource' });
    });
  </script>
</body>
</html>`;
  }
}

function escapeHtml(value: string): string {
  return value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}
