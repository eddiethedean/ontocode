import * as vscode from "vscode";
import { appendError } from "../logging/errorLog";
import {
  DEFAULT_REOPEN,
  PERSPECTIVES,
  resolvePanelRestoreState,
  type PanelRestoreState,
  type Perspective,
} from "./layoutPersistenceLogic";

export type { PanelRestoreState, Perspective };
export { PERSPECTIVES };

const PERSPECTIVE_KEY = "ontocode.perspectives";
const PANEL_STATE_KEY = "ontocode.panelRestoreState";

let extensionContext: vscode.ExtensionContext | undefined;

export function bindLayoutPersistenceContext(
  context: vscode.ExtensionContext
): void {
  extensionContext = context;
}

export async function persistPerspective(
  context: vscode.ExtensionContext,
  perspective: Perspective
): Promise<void> {
  const existing = listPerspectives(context).filter(
    (item) => item.name.toLowerCase() !== perspective.name.toLowerCase()
  );
  await context.workspaceState.update(PERSPECTIVE_KEY, [...existing, perspective]);
}

export function listPerspectives(context: vscode.ExtensionContext): Perspective[] {
  return [
    ...PERSPECTIVES,
    ...(context.workspaceState.get<Perspective[]>(PERSPECTIVE_KEY) ?? []),
  ];
}

export function loadPerspective(
  context: vscode.ExtensionContext,
  name: string
): Perspective | undefined {
  return listPerspectives(context).find(
    (perspective) => perspective.name.toLowerCase() === name.toLowerCase()
  );
}

export async function rememberPanelRestoreState(
  viewType: string,
  state: PanelRestoreState
): Promise<void> {
  if (!extensionContext) {
    return;
  }
  const all =
    extensionContext.workspaceState.get<Record<string, PanelRestoreState>>(
      PANEL_STATE_KEY
    ) ?? {};
  all[viewType] = state;
  await extensionContext.workspaceState.update(PANEL_STATE_KEY, all);
}

export function getPanelRestoreState(
  context: vscode.ExtensionContext,
  viewType: string
): PanelRestoreState | undefined {
  const all =
    context.workspaceState.get<Record<string, PanelRestoreState>>(PANEL_STATE_KEY) ??
    {};
  return resolvePanelRestoreState(all, viewType);
}

const SERIALIZED_VIEWS = Object.keys(DEFAULT_REOPEN);

export function registerWebviewPanelSerializers(
  context: vscode.ExtensionContext
): void {
  bindLayoutPersistenceContext(context);
  for (const viewType of SERIALIZED_VIEWS) {
    context.subscriptions.push(
      vscode.window.registerWebviewPanelSerializer(viewType, {
        async deserializeWebviewPanel(panel): Promise<void> {
          const restore = getPanelRestoreState(context, viewType);
          panel.webview.html = restoredPanelHtml(viewType, restore);
          panel.webview.onDidReceiveMessage(async (message) => {
            if (message?.command === "close") {
              panel.dispose();
              return;
            }
            if (message?.command === "reopen" && restore?.command) {
              try {
                await vscode.commands.executeCommand(
                  restore.command,
                  ...(restore.args ?? [])
                );
                panel.dispose();
              } catch (err) {
                const detail = err instanceof Error ? err.message : String(err);
                appendError(
                  `Failed to restore ${viewType}: ${detail}`,
                  "layout"
                );
                void vscode.window.showErrorMessage(
                  `OntoCode: could not restore panel — ${detail}`
                );
              }
            }
          });
          appendError(
            `Restored ${viewType}; reopen via ${restore?.command ?? "commands"}`,
            "layout"
          );
        },
      })
    );
  }
}

function restoredPanelHtml(
  viewType: string,
  restore: PanelRestoreState | undefined
): string {
  const label = restore?.title ?? viewType;
  const canReopen = Boolean(restore?.command);
  return `<!doctype html><html lang="en"><body style="font-family:var(--vscode-font-family);padding:16px">
<main aria-label="Restored OntoCode panel">
<h2>OntoCode panel restored</h2>
<p>The previous session tab for <code>${escapeHtml(label)}</code> was recovered.</p>
<p>${canReopen ? "Reopen to reload live ontology context." : "Reopen the panel from the OntoCode commands."}</p>
<div role="toolbar" aria-label="Restore actions" style="display:flex;gap:8px;flex-wrap:wrap">
${canReopen ? `<button id="reopen" type="button" autofocus>Reopen panel</button>` : ""}
<button id="close" type="button">Close</button>
</div>
</main>
<script>
const vscode=acquireVsCodeApi();
const reopen=document.getElementById('reopen');
if(reopen){reopen.onclick=()=>vscode.postMessage({command:'reopen'});}
document.getElementById('close').onclick=()=>vscode.postMessage({command:'close'});
</script>
</body></html>`;
}

function escapeHtml(value: string): string {
  return value.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}
