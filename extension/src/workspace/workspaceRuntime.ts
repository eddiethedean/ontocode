import * as vscode from "vscode";
import { appendError } from "../logging/errorLog";
import { focusRelay } from "../focus/focusRelay";
import { ontologyRegistry } from "./ontologyRegistry";
import { workspaceEventBus } from "./eventBus";
import { workspaceTransactionManager } from "./transactionManager";
import { saveCoordinator } from "./saveCoordinator";
import { selectionManager } from "./selectionManager";
import { navigationManager } from "./navigationManager";
import { workspaceSessionPersistence } from "./sessionPersistence";
import { externalChangeRecovery } from "./externalChange";

let initialized = false;

export function initializeWorkspaceRuntime(
  context: vscode.ExtensionContext
): void {
  if (initialized) {
    return;
  }
  initialized = true;

  ontologyRegistry.bindContext(context);
  selectionManager.bindContext(context);
  navigationManager.bindContext(context);
  workspaceSessionPersistence.bindContext(context);
  externalChangeRecovery.register(context);

  const disposables: vscode.Disposable[] = [
    vscode.workspace.onDidOpenTextDocument((doc) => {
      externalChangeRecovery.onDocumentOpened(doc);
    }),
    vscode.workspace.onDidChangeTextDocument((event) => {
      ontologyRegistry.onBufferChanged(event.document);
    }),
    vscode.workspace.onDidSaveTextDocument((document) => {
      ontologyRegistry.onBufferSaved(document);
    }),
    new vscode.Disposable(
      workspaceEventBus.subscribe(() => {
        void workspaceSessionPersistence.persist();
      })
    ),
    new vscode.Disposable(selectionManager.subscribeToFocus()),
  ];

  for (const disposable of disposables) {
    context.subscriptions.push(disposable);
  }

  void (async () => {
    try {
      await waitForWorkspaceFolder();
      await workspaceSessionPersistence.restore({ reopenPanels: true });
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      appendError(`Workspace session restore failed: ${message}`, "workspace");
    }
  })();
}

async function waitForWorkspaceFolder(): Promise<void> {
  if (vscode.workspace.workspaceFolders?.length) {
    return;
  }
  await new Promise<void>((resolve) => {
    const timeout = setTimeout(() => {
      disposable.dispose();
      resolve();
    }, 3000);
    const disposable = vscode.workspace.onDidChangeWorkspaceFolders(() => {
      if (vscode.workspace.workspaceFolders?.length) {
        clearTimeout(timeout);
        disposable.dispose();
        resolve();
      }
    });
  });
}

export {
  ontologyRegistry,
  workspaceEventBus,
  workspaceTransactionManager,
  saveCoordinator,
  selectionManager,
  navigationManager,
  workspaceSessionPersistence,
  externalChangeRecovery,
};

export function resetWorkspaceRuntimeForTests(): void {
  initialized = false;
  ontologyRegistry.resetForTests();
  workspaceEventBus.resetForTests();
  workspaceTransactionManager.resetForTests();
  selectionManager.resetForTests();
  navigationManager.resetForTests();
  workspaceSessionPersistence.resetForTests();
  externalChangeRecovery.resetForTests();
  focusRelay.resetForTests();
}
