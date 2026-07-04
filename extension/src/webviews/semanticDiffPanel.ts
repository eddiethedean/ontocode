import * as vscode from "vscode";
import { semanticDiff } from "../lsp/client";
import { PanelHost } from "./panelHost";
import type { DiffPayload, WebviewMessage } from "./messages";

export class SemanticDiffPanel {
  public static current: SemanticDiffPanel | undefined;

  private constructor(private readonly host: PanelHost) {
    host.panel.onDidDispose(() => {
      SemanticDiffPanel.current = undefined;
    });
  }

  public static async show(
    extensionUri: vscode.Uri,
    params: { leftRef?: string; rightRef?: string }
  ): Promise<SemanticDiffPanel> {
    if (SemanticDiffPanel.current) {
      SemanticDiffPanel.current.host.panel.reveal(vscode.ViewColumn.Beside);
      await SemanticDiffPanel.current.refresh(params);
      return SemanticDiffPanel.current;
    }

    const host = PanelHost.create(extensionUri, {
      viewType: "ontocodeSemanticDiff",
      title: "OntoCode Semantic Diff",
      panel: "semanticDiff",
      onMessage: async (message: WebviewMessage) => {
        const panel = SemanticDiffPanel.current;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });

    const instance = new SemanticDiffPanel(host);
    SemanticDiffPanel.current = instance;
    await instance.refresh(params);
    return instance;
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "copyMarkdown") {
      const diff = this.lastDiff;
      if (!diff) {
        return;
      }
      const lines = ["# Ontology semantic diff", ""];
      if (diff.breaking_changes.length > 0) {
        lines.push("## Breaking changes", "");
        for (const b of diff.breaking_changes) {
          lines.push(`- **${b.reason}**: ${b.message}`);
        }
      }
      await vscode.env.clipboard.writeText(lines.join("\n"));
      void vscode.window.showInformationMessage("OntoCode: diff Markdown copied to clipboard");
    }
  }

  private lastDiff: DiffPayload | undefined;

  private async refresh(params: {
    leftRef?: string;
    rightRef?: string;
  }): Promise<void> {
    try {
      const result = await semanticDiff({
        left_ref: params.leftRef ?? "HEAD",
        right_ref: params.rightRef ?? "WORKSPACE",
      });
      this.lastDiff = result.diff;
      this.host.postMessage({ type: "semanticDiffData", diff: result.diff });
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      this.host.postMessage({ type: "error", message });
    }
  }
}
