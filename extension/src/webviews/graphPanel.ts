import * as vscode from "vscode";
import { getGraph } from "../lsp/client";
import { PanelHost } from "./panelHost";
import type { GraphFilters, WebviewMessage } from "./messages";

export interface GraphPanelOptions {
  graphKind: string;
  rootIri?: string;
  depth?: number;
  includeInferred?: boolean;
  filters?: GraphFilters;
}

export class GraphPanel {
  public static currentPanel: GraphPanel | undefined;

  private constructor(
    private readonly host: PanelHost,
    private options: GraphPanelOptions
  ) {
    host.panel.onDidDispose(() => {
      GraphPanel.currentPanel = undefined;
    });
  }

  public dispose(): void {
    this.host.panel.dispose();
  }

  public static async show(
    extensionUri: vscode.Uri,
    options: GraphPanelOptions,
    title = "OntoCode Graph"
  ): Promise<GraphPanel> {
    if (GraphPanel.currentPanel) {
      GraphPanel.currentPanel.host.panel.reveal(vscode.ViewColumn.Beside);
      GraphPanel.currentPanel.options = options;
      await GraphPanel.currentPanel.refresh();
      return GraphPanel.currentPanel;
    }

    const host = PanelHost.create(extensionUri, {
      viewType: "ontocodeGraph",
      title,
      panel: "graph",
      extraQuery: {
        graphKind: options.graphKind,
        ...(options.rootIri ? { root: options.rootIri } : {}),
      },
      onMessage: async (message: WebviewMessage) => {
        const panel = GraphPanel.currentPanel;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });

    const instance = new GraphPanel(host, options);
    GraphPanel.currentPanel = instance;
    await instance.refresh();
    return instance;
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "requestGraph") {
      this.options = {
        graphKind: message.graphKind,
        rootIri: message.rootIri,
        depth: message.depth,
        includeInferred: message.includeInferred,
        filters: message.filters,
      };
      await this.refresh();
    }
    if (message.type === "selectNode") {
      await vscode.commands.executeCommand("ontocode.openEntity", message.iri);
    }
  }

  private async refresh(): Promise<void> {
    try {
      const result = await getGraph({
        graph_kind: this.options.graphKind,
        root_iri: this.options.rootIri,
        depth: this.options.depth ?? 2,
        include_inferred: this.options.includeInferred ?? false,
        filters: this.options.filters,
      });
      this.host.postMessage({
        type: "graphData",
        graph: result.graph,
        rootIri: this.options.rootIri,
      });
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      this.host.postMessage({ type: "error", message: msg });
    }
  }
}
