import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { focusRelay } from "../focus/focusRelay";
import type { PanelRestoreState } from "../webviews/layoutPersistenceLogic";
import { isAllowedPanelRestoreCommand } from "../webviews/layoutPersistenceLogic";
import { getPanelRestoreState } from "../webviews/layoutPersistence";
import { ontologyRegistry } from "./ontologyRegistry";
import { navigationManager } from "./navigationManager";
import { selectionManager } from "./selectionManager";
import type { WorkspaceSessionSnapshot } from "./types";

const SESSION_KEY = "ontocode.workspaceSession";
const SESSION_FILE = ".ontocode/session.json";
const MAX_PANEL_RESTORE = 12;

export class WorkspaceSessionPersistence {
  private context: vscode.ExtensionContext | undefined;

  bindContext(context: vscode.ExtensionContext): void {
    this.context = context;
  }

  async capture(): Promise<WorkspaceSessionSnapshot> {
    const focus = focusRelay.getFocus();
    const panelRestore: WorkspaceSessionSnapshot["panelRestore"] = {};
    if (this.context) {
      const viewTypes = [
        "ontocodeInspector",
        "ontocodeGraph",
        "ontocodeQueryWorkbench",
        "ontocodeImports",
        "ontocodeReasoner",
        "ontocodeExplanation",
        "ontocodeSemanticDiff",
        "ontocodeManchesterEditor",
      ];
      for (const viewType of viewTypes) {
        const state = getPanelRestoreState(this.context, viewType);
        if (state?.command && isAllowedPanelRestoreCommand(state.command)) {
          panelRestore[viewType] = state;
        }
      }
    }
    return {
      openOntologyUris: ontologyRegistry
        .getSnapshot()
        .map((entry) => entry.uri),
      activeOntologyId: ontologyRegistry.getActiveId(),
      focus: focus
        ? { kind: focus.kind, id: focus.id, source: focus.source }
        : undefined,
      navigation: navigationManager.getStack(),
      navigationIndex: navigationManager.getIndex(),
      panelRestore,
    };
  }

  async persist(): Promise<void> {
    if (!this.context) {
      return;
    }
    const snapshot = await this.capture();
    await this.context.workspaceState.update(SESSION_KEY, snapshot);
    await this.writeSessionFile(snapshot);
  }

  async restore(options?: { reopenPanels?: boolean }): Promise<void> {
    if (!this.context) {
      return;
    }
    if (!vscode.workspace.workspaceFolders?.length) {
      return;
    }
    const snapshot =
      this.context.workspaceState.get<WorkspaceSessionSnapshot>(SESSION_KEY) ??
      (await this.readSessionFile());
    if (!snapshot) {
      return;
    }

    ontologyRegistry.hydrateOpenUris(snapshot.openOntologyUris ?? []);
    for (const uri of snapshot.openOntologyUris ?? []) {
      try {
        await vscode.workspace.openTextDocument(vscode.Uri.parse(uri));
      } catch {
        // File may have moved; registry sync will reconcile.
      }
    }

    await ontologyRegistry.syncFromCatalog();

    if (snapshot.activeOntologyId) {
      await ontologyRegistry.activate(snapshot.activeOntologyId);
    }

    if (snapshot.focus?.id) {
      selectionManager.hydrate(snapshot.focus.id);
    }

    if (snapshot.navigation?.length) {
      navigationManager.hydrate(
        snapshot.navigation,
        snapshot.navigationIndex ?? snapshot.navigation.length - 1
      );
    }

    if (options?.reopenPanels !== false && snapshot.panelRestore) {
      await this.reopenPanels(snapshot.panelRestore);
    }
  }

  async reopenPanels(
    panelRestore: Record<string, PanelRestoreState>
  ): Promise<void> {
    // Do not execute restore commands from session.json in untrusted workspaces (#309).
    if (!vscode.workspace.isTrusted) {
      return;
    }
    const entries = Object.entries(panelRestore).slice(0, MAX_PANEL_RESTORE);
    for (const [, state] of entries) {
      if (!state.command || !isAllowedPanelRestoreCommand(state.command)) {
        continue;
      }
      try {
        await vscode.commands.executeCommand(state.command, ...(state.args ?? []));
      } catch {
        // Panel restore is best-effort.
      }
    }
  }

  private async writeSessionFile(snapshot: WorkspaceSessionSnapshot): Promise<void> {
    const folder = vscode.workspace.workspaceFolders?.[0];
    if (!folder) {
      return;
    }
    const dir = path.join(folder.uri.fsPath, ".ontocode");
    const file = path.join(dir, "session.json");
    try {
      await fs.promises.mkdir(dir, { recursive: true });
      await fs.promises.writeFile(file, JSON.stringify(snapshot, null, 2), "utf8");
    } catch {
      // File session is optional for multi-root sharing.
    }
  }

  private async readSessionFile(): Promise<WorkspaceSessionSnapshot | undefined> {
    const folder = vscode.workspace.workspaceFolders?.[0];
    if (!folder) {
      return undefined;
    }
    const file = path.join(folder.uri.fsPath, SESSION_FILE);
    try {
      const raw = await fs.promises.readFile(file, "utf8");
      return JSON.parse(raw) as WorkspaceSessionSnapshot;
    } catch {
      return undefined;
    }
  }

  resetForTests(): void {
    this.context = undefined;
  }
}

export const workspaceSessionPersistence = new WorkspaceSessionPersistence();
