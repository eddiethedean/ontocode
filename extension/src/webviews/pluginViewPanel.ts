import * as vscode from "vscode";
import { runPlugin } from "../lsp/client";
import type { PluginDescriptor, PluginViewContribution } from "../lsp/protocol";

export class PluginViewPanel {
  static async open(
    extensionUri: vscode.Uri,
    plugin: PluginDescriptor,
    view: PluginViewContribution
  ): Promise<void> {
    const panel = vscode.window.createWebviewPanel(
      "ontocode.pluginView",
      `${plugin.name}: ${view.title}`,
      vscode.ViewColumn.Beside,
      {
        enableScripts: false,
        retainContextWhenHidden: true,
      }
    );

    panel.webview.html = `<html><body><p>Loading view…</p></body></html>`;

    try {
      const result = await runPlugin({
        plugin_id: plugin.id,
        action: "ui_view",
        view_id: view.id,
      });

      const html =
        result.view_html ??
        (result.logs
          ? `<pre>${escapeHtml(result.logs)}</pre>`
          : `<p>No view output.</p>`);

      panel.webview.html = wrapHtml(
        `${plugin.name}: ${view.title}`,
        html,
        extensionUri
      );
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      panel.webview.html = wrapHtml(
        `${plugin.name}: ${view.title}`,
        `<pre>${escapeHtml(message)}</pre>`,
        extensionUri
      );
    }
  }
}

function wrapHtml(title: string, body: string, _extensionUri: vscode.Uri): string {
  // Keep this minimal and CSP-safe; plugin HTML is treated as trusted-by-user content
  // because it runs locally and is explicitly installed in the workspace.
  return `<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline';" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>${escapeHtml(title)}</title>
    <style>
      body { font-family: -apple-system, BlinkMacSystemFont, Segoe UI, sans-serif; padding: 12px; }
      pre { white-space: pre-wrap; }
    </style>
  </head>
  <body>
    ${body}
  </body>
</html>`;
}

function escapeHtml(text: string): string {
  return text
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#039;");
}

