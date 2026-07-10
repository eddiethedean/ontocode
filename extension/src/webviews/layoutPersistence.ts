import * as vscode from "vscode";
import { appendError } from "../logging/errorLog";

export interface Perspective {
  name: string;
  panels: string[];
}

const PERSPECTIVE_KEY = "ontocode.perspectives";

export const PERSPECTIVES: readonly Perspective[] = [
  { name: "Modeling", panels: ["inspector", "query"] },
  { name: "Reasoning", panels: ["reasoner", "explanation", "graph"] },
  { name: "Review", panels: ["semanticDiff", "imports"] },
];

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

const SERIALIZED_VIEWS = [
  "ontocodeInspector",
  "ontocodeGraph",
  "ontocodeQueryWorkbench",
  "ontocodeImports",
  "ontocodeReasoner",
  "ontocodeRefactorPreview",
  "ontocodeExplanation",
  "ontocodeSemanticDiff",
  "ontocodeManchesterEditor",
] as const;

export function registerWebviewPanelSerializers(
  context: vscode.ExtensionContext
): void {
  for (const viewType of SERIALIZED_VIEWS) {
    context.subscriptions.push(
      vscode.window.registerWebviewPanelSerializer(viewType, {
        async deserializeWebviewPanel(panel): Promise<void> {
          // Stateful panels need an ontology/entity/refactor payload before they can
          // safely resume. Keep the tab and provide a useful recovery action.
          panel.webview.html = restoredPanelHtml(viewType);
          panel.webview.onDidReceiveMessage((message) => {
            if (message?.command === "close") {
              panel.dispose();
            }
          });
          appendError(`Restored ${viewType} without transient panel state`, "layout");
        },
      })
    );
  }
}

function restoredPanelHtml(viewType: string): string {
  return `<!doctype html><html><body style="font-family:var(--vscode-font-family);padding:16px">
<h2>OntoCode panel restored</h2>
<p>The transient state for <code>${escapeHtml(viewType)}</code> is no longer available. Reopen the panel from the OntoCode commands.</p>
<button id="close">Close</button>
<script>const vscode=acquireVsCodeApi();document.getElementById('close').onclick=()=>vscode.postMessage({command:'close'});</script>
</body></html>`;
}

function escapeHtml(value: string): string {
  return value.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}
