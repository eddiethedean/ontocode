import * as vscode from "vscode";
import { getWebviewHtml } from "./getWebviewHtml";
import type { HostMessage, PanelKind, WebviewMessage } from "./messages";
import { isWebviewMessage } from "./messages";

export interface PanelHostOptions {
  viewType: string;
  title: string;
  panel: PanelKind;
  column?: vscode.ViewColumn;
  extraQuery?: Record<string, string>;
  onMessage?: (message: WebviewMessage, panel: vscode.WebviewPanel) => void | Promise<void>;
}

export class PanelHost {
  private disposed = false;

  constructor(
    public readonly panel: vscode.WebviewPanel,
    private readonly extensionUri: vscode.Uri,
    public readonly panelKind: PanelKind
  ) {
    panel.onDidDispose(() => {
      this.disposed = true;
    });
  }

  get isDisposed(): boolean {
    return this.disposed;
  }

  postMessage(message: HostMessage): void {
    if (!this.disposed) {
      void this.panel.webview.postMessage(message);
    }
  }

  static create(
    extensionUri: vscode.Uri,
    options: PanelHostOptions
  ): PanelHost {
    const panel = vscode.window.createWebviewPanel(
      options.viewType,
      options.title,
      options.column ?? vscode.ViewColumn.Beside,
      {
        enableScripts: true,
        retainContextWhenHidden: true,
        localResourceRoots: [
          vscode.Uri.joinPath(extensionUri, "webview-ui", "dist"),
        ],
      }
    );

    panel.webview.options = {
      enableScripts: true,
      localResourceRoots: [
        vscode.Uri.joinPath(extensionUri, "webview-ui", "dist"),
      ],
    };

    panel.webview.html = getWebviewHtml(
      panel.webview,
      extensionUri,
      options.panel,
      options.extraQuery
    );

    const host = new PanelHost(panel, extensionUri, options.panel);

    panel.webview.onDidReceiveMessage(async (data: unknown) => {
      if (!isWebviewMessage(data)) {
        return;
      }
      if (data.type === "ready" && data.panel === options.panel) {
        host.postMessage({ type: "init", panel: options.panel });
      }
      if (options.onMessage) {
        await options.onMessage(data, panel);
      }
    });

    return host;
  }
}
