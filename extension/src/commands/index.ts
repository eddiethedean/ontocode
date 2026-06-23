import * as vscode from "vscode";
import {
  getCatalogSnapshot,
  getEntity,
  indexWorkspace,
} from "../lsp/client";
import { EntityInspectorPanel } from "../webviews/inspector";
import { ExplorerTreeProvider } from "../treeviews/explorer";
import { resolveEntityIri } from "../utils/resolveEntityIri";
import { byteColToUtf16 } from "../utils/positions";

export function registerCommands(
  context: vscode.ExtensionContext,
  providers: {
    ontologies: ExplorerTreeProvider;
    classes: ExplorerTreeProvider;
    properties: ExplorerTreeProvider;
    individuals: ExplorerTreeProvider;
    diagnostics: ExplorerTreeProvider;
  }
): void {
  context.subscriptions.push(
    vscode.commands.registerCommand("ontocode.indexWorkspace", async () => {
      await runIndexAndRefresh(providers);
      vscode.window.showInformationMessage("OntoCode: workspace indexed");
    }),
    vscode.commands.registerCommand("ontocode.refreshExplorer", async () => {
      await refreshExplorer(providers);
    }),
    vscode.commands.registerCommand(
      "ontocode.showEntityInspector",
      async (iri?: string) => {
        if (!iri) {
          iri = await vscode.window.showInputBox({
            prompt: "Entity IRI",
            placeHolder: "http://example.org/ontology#Class",
          });
        }
        if (iri) {
          try {
            await openInspector(context.extensionUri, iri);
          } catch (err) {
            const message = err instanceof Error ? err.message : String(err);
            void vscode.window.showErrorMessage(
              `OntoCode: could not open entity — ${message}`
            );
          }
        }
      }
    ),
    vscode.commands.registerCommand(
      "ontocode.openEntity",
      async (arg?: unknown) => {
        const iri = resolveEntityIri(arg);
        if (!iri) {
          return;
        }
        try {
          await openInspector(context.extensionUri, iri);
        } catch (err) {
          const message = err instanceof Error ? err.message : String(err);
          void vscode.window.showErrorMessage(
            `OntoCode: could not open entity — ${message}`
          );
        }
      }
    ),
    vscode.commands.registerCommand(
      "ontocode.jumpToSource",
      async (arg?: unknown) => {
        let iri = resolveEntityIri(arg);
        if (!iri) {
          iri = await vscode.window.showInputBox({ prompt: "Entity IRI" });
        }
        if (!iri) {
          return;
        }
        try {
          const { detail } = await getEntity(iri);
          if (!detail.source) {
            void vscode.window.showWarningMessage(
              `No source location found for ${iri}`
            );
            return;
          }
          const doc = await vscode.workspace.openTextDocument(
            vscode.Uri.file(detail.source.path)
          );
          const editor = await vscode.window.showTextDocument(doc);
          const lineText = doc.lineAt(
            Math.max(0, detail.source.line - 1)
          ).text;
          const line = Math.max(0, detail.source.line - 1);
          const col = byteColToUtf16(lineText, Math.max(0, detail.source.column));
          const pos = new vscode.Position(line, col);
          editor.selection = new vscode.Selection(pos, pos);
          editor.revealRange(
            new vscode.Range(pos, pos),
            vscode.TextEditorRevealType.InCenter
          );
        } catch (err) {
          const message = err instanceof Error ? err.message : String(err);
          void vscode.window.showErrorMessage(
            `OntoCode: jump to source failed — ${message}`
          );
        }
      }
    )
  );
}

async function runIndexAndRefresh(providers: {
  ontologies: ExplorerTreeProvider;
  classes: ExplorerTreeProvider;
  properties: ExplorerTreeProvider;
  individuals: ExplorerTreeProvider;
  diagnostics: ExplorerTreeProvider;
}): Promise<void> {
  await indexWorkspace();
  await refreshExplorer(providers);
}

export async function refreshExplorer(providers: {
  ontologies: ExplorerTreeProvider;
  classes: ExplorerTreeProvider;
  properties: ExplorerTreeProvider;
  individuals: ExplorerTreeProvider;
  diagnostics: ExplorerTreeProvider;
}): Promise<void> {
  try {
    const snapshot = await getCatalogSnapshot();
    providers.ontologies.setSnapshot(snapshot);
    providers.classes.setSnapshot(snapshot);
    providers.properties.setSnapshot(snapshot);
    providers.individuals.setSnapshot(snapshot);
    providers.diagnostics.setSnapshot(snapshot);
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    vscode.window.showErrorMessage(`OntoCode refresh failed: ${message}`);
  }
}

async function openInspector(
  extensionUri: vscode.Uri,
  iri: string
): Promise<void> {
  const { detail } = await getEntity(iri);
  EntityInspectorPanel.show(extensionUri, detail);
}
