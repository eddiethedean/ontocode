import * as vscode from "vscode";
import { focusRelay } from "../focus/focusRelay";
import { cancelActiveReasonerRequest, runReasoner } from "../lsp/client";
import type { RunReasonerResult } from "../lsp/protocol";
import { rememberPanelRestoreState } from "./layoutPersistence";
import type { ReasonerResultPayload, WebviewMessage } from "./messages";
import { PanelHost } from "./panelHost";
import { summarizeResult } from "./reasonerPanelLogic";

function toPayload(result: RunReasonerResult): ReasonerResultPayload {
  return {
    profile_used: result.profile_used,
    consistent: result.consistent,
    unsatisfiable: result.unsatisfiable,
    inferred_edge_count: result.inferred_edge_count,
    new_inferences: result.new_inferences,
    warnings: result.warnings,
    duration_ms: result.duration_ms,
  };
}

export class ReasonerPanel {
  public static current: ReasonerPanel | undefined;
  private lastResult: RunReasonerResult | undefined;
  private runId = 0;

  private constructor(private readonly host: PanelHost) {
    this.host.panel.onDidDispose(() => {
      ReasonerPanel.current = undefined;
    });
  }

  public dispose(): void {
    this.host.panel.dispose();
  }

  public static show(extensionUri: vscode.Uri): ReasonerPanel {
    void rememberPanelRestoreState("ontocodeReasoner", {
      command: "ontocode.classifyOntology",
      title: "OntoCode Reasoner",
    });
    if (ReasonerPanel.current) {
      ReasonerPanel.current.host.panel.reveal(vscode.ViewColumn.Beside);
      return ReasonerPanel.current;
    }
    const host = PanelHost.create(extensionUri, {
      viewType: "ontocodeReasoner",
      title: "OntoCode Reasoner",
      panel: "reasoner",
      onMessage: async (message: WebviewMessage) => {
        const panel = ReasonerPanel.current;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });
    ReasonerPanel.current = new ReasonerPanel(host);
    return ReasonerPanel.current;
  }

  public async runWithDefaults(): Promise<void> {
    const cfg = vscode.workspace.getConfiguration("ontocode");
    const profile = cfg.get<string>("reasoner.default") ?? "el";
    const autoDetect = cfg.get<boolean>("reasoner.autoProfile") ?? true;
    await this.run(profile, autoDetect, ++this.runId);
  }

  /** Push an already-computed result into the React panel. */
  public presentResult(result: RunReasonerResult): void {
    this.lastResult = result;
    const runId = ++this.runId;
    this.host.postMessage({ type: "reasonerSyncRunId", runId });
    this.host.postMessage({
      type: "reasonerResult",
      runId,
      result: toPayload(result),
      summary: summarizeResult(result),
    });
  }

  /** Invalidate in-flight runs so late RPC results are ignored (#141). */
  public cancelActiveRun(): void {
    cancelActiveReasonerRequest();
    this.runId += 1;
    this.host.postMessage({ type: "reasonerSyncRunId", runId: this.runId });
    const prev = focusRelay.getReasoning();
    focusRelay.setReasoningState({
      profile: this.lastResult?.profile_used ?? prev?.profile ?? "el",
      unsatisfiable: this.lastResult?.unsatisfiable ?? prev?.unsatisfiable ?? [],
      lastRunAt: prev?.lastRunAt ?? 0,
      dirty: prev?.dirty ?? true,
      running: false,
    });
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "runReasoner") {
      await this.run(message.profile, message.autoDetect, message.runId);
      return;
    }
    if (message.type === "explainUnsat") {
      const profile =
        this.lastResult?.profile_used ?? focusRelay.getReasoning()?.profile;
      await vscode.commands.executeCommand(
        "ontocode.showExplanation",
        message.classIri,
        profile
      );
      return;
    }
    if (message.type === "showInferredHierarchy") {
      await vscode.workspace
        .getConfiguration("ontocode")
        .update("hierarchy.mode", "combined", vscode.ConfigurationTarget.Workspace);
      void vscode.commands.executeCommand("ontocode.refreshExplorer");
    }
  }

  private async run(
    profile: string,
    autoDetect: boolean,
    runId: number
  ): Promise<void> {
    this.runId = runId;
    this.host.postMessage({ type: "reasonerSyncRunId", runId });
    try {
      focusRelay.setReasoningState({
        profile,
        unsatisfiable: this.lastResult?.unsatisfiable ?? [],
        lastRunAt: Date.now(),
        dirty: false,
        running: true,
      });
      const result = await runReasoner({ profile, auto_detect: autoDetect });
      if (runId !== this.runId) {
        return;
      }
      this.lastResult = result;
      focusRelay.setReasoningState({
        profile: result.profile_used ?? profile,
        unsatisfiable: result.unsatisfiable ?? [],
        lastRunAt: Date.now(),
        dirty: false,
        running: false,
      });
      this.host.postMessage({
        type: "reasonerResult",
        runId,
        result: toPayload(result),
        summary: summarizeResult(result),
      });
      void vscode.commands.executeCommand("ontocode.refreshExplorer");
    } catch (err) {
      if (runId !== this.runId) {
        return;
      }
      const message = err instanceof Error ? err.message : String(err);
      focusRelay.setReasoningState({
        profile,
        unsatisfiable: this.lastResult?.unsatisfiable ?? [],
        lastRunAt: Date.now(),
        dirty: true,
        running: false,
      });
      this.host.postMessage({
        type: "reasonerResult",
        runId,
        error: message,
      });
    }
  }
}
