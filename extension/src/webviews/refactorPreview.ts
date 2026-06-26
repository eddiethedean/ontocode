import * as vscode from "vscode";
import { applyRefactor, findUsages, previewRefactor } from "../lsp/client";
import { RefactorPlan, RefactorRequest } from "../lsp/protocol";
import { PanelHost } from "./panelHost";
import type { WebviewMessage } from "./messages";

export async function showEntityUsages(iri: string): Promise<void> {
  const result = await findUsages(iri);
  if (result.usages.length === 0) {
    void vscode.window.showInformationMessage(`No usages found for ${iri}`);
    return;
  }
  const picked = await vscode.window.showQuickPick(
    result.usages.map((u) => ({
      label: `${u.file.split(/[/\\]/).pop() ?? u.file}:${u.line ?? 0}`,
      description: u.kind,
      detail: u.context,
      usage: u,
    })),
    { placeHolder: `Usages of ${iri}` }
  );
  if (!picked) {
    return;
  }
  const doc = await vscode.workspace.openTextDocument(
    vscode.Uri.file(picked.usage.file)
  );
  const editor = await vscode.window.showTextDocument(doc, vscode.ViewColumn.One);
  const line = Math.max(0, (picked.usage.line ?? 1) - 1);
  const col = picked.usage.column ?? 0;
  const pos = new vscode.Position(line, col);
  editor.selection = new vscode.Selection(pos, pos);
  editor.revealRange(new vscode.Range(pos, pos));
}

export async function renameEntityIri(
  extensionUri: vscode.Uri,
  iri?: string
): Promise<void> {
  const from =
    iri ??
    (await vscode.window.showInputBox({
      prompt: "Entity IRI to rename",
    }));
  if (!from) {
    return;
  }
  const to = await vscode.window.showInputBox({
    prompt: "New IRI",
    placeHolder: "http://example.org/ontology#NewName",
  });
  if (!to) {
    return;
  }
  await runRefactorPreview(extensionUri, {
    kind: "rename_iri",
    from_iri: from,
    to_iri: to,
  });
}

export async function migrateNamespace(
  extensionUri: vscode.Uri
): Promise<void> {
  const from = await vscode.window.showInputBox({
    prompt: "Old namespace base IRI",
  });
  if (!from) {
    return;
  }
  const to = await vscode.window.showInputBox({
    prompt: "New namespace base IRI",
  });
  if (!to) {
    return;
  }
  await runRefactorPreview(extensionUri, {
    kind: "migrate_namespace",
    from_base: from,
    to_base: to,
  });
}

export async function moveEntity(
  extensionUri: vscode.Uri,
  iri?: string
): Promise<void> {
  const entity =
    iri ??
    (await vscode.window.showInputBox({ prompt: "Entity IRI to move" }));
  if (!entity) {
    return;
  }
  const target = await vscode.window.showOpenDialog({
    canSelectMany: false,
    filters: { Turtle: ["ttl"] },
    openLabel: "Move entity here",
  });
  if (!target?.[0]) {
    return;
  }
  await runRefactorPreview(extensionUri, {
    kind: "move_entity",
    entity_iri: entity,
    target_file: target[0].fsPath,
  });
}

export async function extractModule(extensionUri: vscode.Uri): Promise<void> {
  const entitiesRaw = await vscode.window.showInputBox({
    prompt: "Entity IRIs (comma-separated)",
  });
  if (!entitiesRaw?.trim()) {
    return;
  }
  const output = await vscode.window.showSaveDialog({
    filters: { Turtle: ["ttl"] },
    defaultUri: vscode.Uri.file("module.ttl"),
  });
  if (!output) {
    return;
  }
  await runRefactorPreview(extensionUri, {
    kind: "extract_module",
    entity_iris: entitiesRaw
      .split(",")
      .map((s) => s.trim())
      .filter(Boolean),
    output_file: output.fsPath,
    leave_stub: false,
  });
}

export class RefactorPreviewPanel {
  public static current: RefactorPreviewPanel | undefined;
  private host: PanelHost;
  private plan: RefactorPlan | undefined;
  private onApplied?: () => Promise<void>;

  private constructor(host: PanelHost, onApplied?: () => Promise<void>) {
    this.host = host;
    this.onApplied = onApplied;
    host.panel.onDidDispose(() => {
      RefactorPreviewPanel.current = undefined;
    });
  }

  public static async show(
    extensionUri: vscode.Uri,
    plan: RefactorPlan,
    onApplied?: () => Promise<void>
  ): Promise<RefactorPreviewPanel> {
    if (RefactorPreviewPanel.current) {
      RefactorPreviewPanel.current.plan = plan;
      RefactorPreviewPanel.current.host.postMessage({
        type: "loadRefactorPlan",
        plan,
      });
      RefactorPreviewPanel.current.host.panel.reveal();
      return RefactorPreviewPanel.current;
    }
    const host = PanelHost.create(extensionUri, {
      viewType: "ontocodeRefactorPreview",
      title: "OntoCode Refactor Preview",
      panel: "refactorPreview",
      onMessage: async (message: WebviewMessage) => {
        const panel = RefactorPreviewPanel.current;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });
    const instance = new RefactorPreviewPanel(host, onApplied);
    instance.plan = plan;
    RefactorPreviewPanel.current = instance;
    instance.host.postMessage({ type: "loadRefactorPlan", plan });
    return instance;
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "applyRefactor" && this.plan) {
      try {
        const result = await applyRefactor(this.plan, false);
        if (result.reindex_warning) {
          void vscode.window.showWarningMessage(result.reindex_warning);
        }
        void vscode.window.showInformationMessage(
          `OntoCode: refactor applied to ${result.files_written} file(s)`
        );
        this.host.panel.dispose();
        if (this.onApplied) {
          await this.onApplied();
        }
      } catch (err) {
        const msg = err instanceof Error ? err.message : String(err);
        void vscode.window.showErrorMessage(`OntoCode refactor failed: ${msg}`);
      }
    }
    if (message.type === "cancelRefactor") {
      this.host.panel.dispose();
    }
  }
}

async function runRefactorPreview(
  extensionUri: vscode.Uri,
  request: RefactorRequest,
  onApplied?: () => Promise<void>
): Promise<void> {
  const result = await previewRefactor(request);
  const plan: RefactorPlan = {
    changes: result.changes,
    warnings: result.warnings,
  };
  if (plan.changes.length === 0) {
    void vscode.window.showWarningMessage(
      "Refactor preview produced no file changes"
    );
    return;
  }
  await RefactorPreviewPanel.show(extensionUri, plan, onApplied);
}
