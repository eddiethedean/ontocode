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
  private webviewReady = false;
  private pendingMessages: HostMessage[] = [];

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

  /** True after the webview posted `{ type: "ready", panel: … }`. */
  isWebviewReady(): boolean {
    return this.webviewReady;
  }

  getWebviewHtml(): string {
    return this.panel.webview.html;
  }

  /** Post a host message; buffers until the webview sends `ready`. */
  postMessage(message: HostMessage): void {
    if (this.disposed) {
      return;
    }
    if (!this.webviewReady) {
      this.pendingMessages.push(message);
      return;
    }
    void this.panel.webview.postMessage(message);
  }

  private flushPending(): void {
    if (this.disposed) {
      return;
    }
    const queued = this.pendingMessages;
    this.pendingMessages = [];
    for (const message of queued) {
      void this.panel.webview.postMessage(message);
    }
  }

  private onWebviewReady(): void {
    if (this.webviewReady || this.disposed) {
      return;
    }
    this.webviewReady = true;
    void this.panel.webview.postMessage({ type: "init", panel: this.panelKind });
    this.flushPending();
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
        host.onWebviewReady();
      }
      if (options.onMessage) {
        await options.onMessage(data, panel);
      }
    });

    return host;
  }
}
