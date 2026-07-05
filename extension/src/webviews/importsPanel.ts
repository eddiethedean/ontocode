import * as path from "path";
import * as vscode from "vscode";
import { applyAxiomPatch, getCatalogSnapshot } from "../lsp/client";
import { OntologyDocument, PatchOp } from "../lsp/protocol";
import { documentUriInWorkspace } from "../utils/workspacePath";
import { PanelHost } from "./panelHost";
import type { ImportsDocumentPayload, WebviewMessage } from "./messages";

type RefreshFn = () => Promise<void>;

function ontologyIri(doc: OntologyDocument): string {
  return doc.base_iri ?? doc.id;
}

function buildPayload(
  doc: OntologyDocument,
  allDocs: OntologyDocument[]
): ImportsDocumentPayload {
  const selfIri = ontologyIri(doc);
  const imported = new Set(doc.imports ?? []);
  const options = allDocs
    .filter((d) => d.format === "turtle" && d.path !== doc.path)
    .map((d) => ({
      iri: ontologyIri(d),
      path: d.path,
      label: path.basename(d.path),
    }))
    .filter((o) => o.iri !== selfIri && !imported.has(o.iri));

  return {
    path: doc.path,
    ontology_iri: selfIri,
    imports: doc.imports ?? [],
    options,
  };
}

export class ImportsPanel {
  public static current: ImportsPanel | undefined;

  private documentUri: string | undefined;
  private ontologyIri: string | undefined;
  private filePath: string | undefined;

  private constructor(
    private readonly host: PanelHost,
    private readonly onRefresh?: RefreshFn
  ) {
    host.panel.onDidDispose(() => {
      ImportsPanel.current = undefined;
    });
  }

  public static async show(
    extensionUri: vscode.Uri,
    filePath: string,
    onRefresh?: RefreshFn
  ): Promise<ImportsPanel> {
    if (ImportsPanel.current) {
      ImportsPanel.current.host.panel.reveal(vscode.ViewColumn.Beside);
      await ImportsPanel.current.load(filePath);
      return ImportsPanel.current;
    }

    const host = PanelHost.create(extensionUri, {
      viewType: "ontocodeImports",
      title: "Manage Imports",
      panel: "imports",
      onMessage: async (message: WebviewMessage) => {
        const panel = ImportsPanel.current;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });

    const instance = new ImportsPanel(host, onRefresh);
    ImportsPanel.current = instance;
    await instance.load(filePath);
    return instance;
  }

  private async load(filePath: string): Promise<void> {
    this.filePath = filePath;
    const snapshot = await getCatalogSnapshot();
    const doc = snapshot.documents.find((d) => d.path === filePath);
    if (!doc) {
      void vscode.window.showErrorMessage(
        `OntoCode: no indexed Turtle document at ${filePath}`
      );
      return;
    }
    if (doc.format !== "turtle") {
      void vscode.window.showErrorMessage(
        "OntoCode: imports management is only available for Turtle (.ttl) files"
      );
      return;
    }

    this.documentUri = documentUriInWorkspace(doc.path);
    this.ontologyIri = ontologyIri(doc);
    this.host.panel.title = `Imports: ${path.basename(doc.path)}`;
    this.host.postMessage({
      type: "loadImports",
      payload: buildPayload(doc, snapshot.documents),
    });
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type !== "applyPatch" || !this.documentUri || !this.ontologyIri) {
      return;
    }
    const { parseApplyPatchMessage } = await import("./messages");
    const parsed = parseApplyPatchMessage(message, undefined);
    if (!parsed) {
      void vscode.window.showErrorMessage(
        "OntoCode: ignored invalid applyPatch message from imports panel"
      );
      return;
    }
    await this.runPatch(parsed.patches, parsed.previewOnly);
  }

  private async runPatch(
    patches: PatchOp[],
    previewOnly: boolean
  ): Promise<void> {
    if (!this.documentUri) {
      return;
    }
    try {
      const result = await applyAxiomPatch({
        document_uri: this.documentUri,
        patches,
        preview_only: previewOnly,
      });
      if (previewOnly && result.preview_text) {
        this.host.postMessage({ type: "preview", text: result.preview_text });
        return;
      }
      if (!previewOnly) {
        if (result.reindex_warning) {
          void vscode.window.showWarningMessage(
            `OntoCode: changes saved but reindex failed — ${result.reindex_warning}`
          );
        }
        void vscode.window.showInformationMessage("OntoCode: imports updated");
        if (this.onRefresh) {
          await this.onRefresh();
        }
        const snapshot = await getCatalogSnapshot();
        const doc = snapshot.documents.find((d) => d.path === this.filePath);
        if (doc) {
          this.host.postMessage({
            type: "loadImports",
            payload: buildPayload(doc, snapshot.documents),
          });
        }
      }
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      void vscode.window.showErrorMessage(`OntoCode: import patch failed — ${msg}`);
    }
  }
}
