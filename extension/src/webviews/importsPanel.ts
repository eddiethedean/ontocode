import * as path from "path";
import * as vscode from "vscode";
import { applyAxiomPatch, getCatalogSnapshot } from "../lsp/client";
import {
  hasPatchFailureDiagnostics,
  patchFailureMessage,
} from "../lsp/patchFeedback";
import { Entity, OntologyDocument, PatchOp } from "../lsp/protocol";
import { documentUriInWorkspace } from "../utils/workspacePath";
import {
  MISSING_ONTOLOGY_HEADER_MESSAGE,
  resolveOntologyIri,
} from "./importsOntology";
import { PanelHost } from "./panelHost";
import type { ImportsDocumentPayload, WebviewMessage } from "./messages";

type RefreshFn = () => Promise<void>;

function buildPayload(
  doc: OntologyDocument,
  allDocs: OntologyDocument[],
  entities: Entity[]
): ImportsDocumentPayload {
  const selfIri = resolveOntologyIri(doc, entities);
  const imported = new Set(doc.imports ?? []);
  const options = allDocs
    .filter((d) => d.format === "turtle" && d.path !== doc.path)
    .map((d) => {
      const iri = resolveOntologyIri(d, entities);
      if (!iri) {
        return undefined;
      }
      return {
        iri,
        path: d.path,
        label: path.basename(d.path),
      };
    })
    .filter((o): o is NonNullable<typeof o> => o !== undefined)
    .filter((o) => o.iri !== selfIri && !imported.has(o.iri));

  return {
    path: doc.path,
    ontology_iri: selfIri,
    imports_editable: selfIri !== undefined,
    error: selfIri ? undefined : MISSING_ONTOLOGY_HEADER_MESSAGE,
    imports: doc.imports ?? [],
    options,
  };
}

function errorPayload(filePath: string, message: string): ImportsDocumentPayload {
  return {
    path: filePath,
    imports_editable: false,
    error: message,
    imports: [],
    options: [],
  };
}

export class ImportsPanel {
  public static current: ImportsPanel | undefined;

  private documentUri: string | undefined;
  private ontologyIri: string | undefined;
  private importsEditable = false;
  private filePath: string | undefined;
  private loadGeneration = 0;

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
    const generation = ++this.loadGeneration;
    this.filePath = filePath;

    const snapshot = await getCatalogSnapshot();
    if (generation !== this.loadGeneration) {
      return;
    }

    const doc = snapshot.documents.find((d) => d.path === filePath);
    if (!doc) {
      this.clearEditableState();
      void vscode.window.showErrorMessage(
        `OntoCode: no indexed Turtle document at ${filePath}`
      );
      this.host.postMessage({
        type: "loadImports",
        payload: errorPayload(
          filePath,
          `No indexed Turtle document at ${filePath}`
        ),
      });
      return;
    }
    if (doc.format !== "turtle") {
      this.clearEditableState();
      void vscode.window.showErrorMessage(
        "OntoCode: imports management is only available for Turtle (.ttl) files"
      );
      this.host.postMessage({
        type: "loadImports",
        payload: errorPayload(
          filePath,
          "Imports management is only available for Turtle (.ttl) files"
        ),
      });
      return;
    }

    const payload = buildPayload(doc, snapshot.documents, snapshot.entities);
    this.documentUri = documentUriInWorkspace(doc.path);
    this.ontologyIri = payload.ontology_iri;
    this.importsEditable = payload.imports_editable;

    if (generation !== this.loadGeneration) {
      return;
    }

    this.host.panel.title = `Imports: ${path.basename(doc.path)}`;
    this.host.postMessage({ type: "loadImports", payload });
  }

  private clearEditableState(): void {
    this.documentUri = undefined;
    this.ontologyIri = undefined;
    this.importsEditable = false;
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (
      message.type !== "applyPatch" ||
      !this.documentUri ||
      !this.ontologyIri ||
      !this.importsEditable
    ) {
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
      if (previewOnly) {
        if (hasPatchFailureDiagnostics(result)) {
          void vscode.window.showErrorMessage(patchFailureMessage(result));
          return;
        }
        if (result.preview_text) {
          this.host.postMessage({ type: "preview", text: result.preview_text });
        }
        return;
      }
      if (!result.applied) {
        void vscode.window.showErrorMessage(patchFailureMessage(result));
        return;
      }
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
          payload: buildPayload(doc, snapshot.documents, snapshot.entities),
        });
      }
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      void vscode.window.showErrorMessage(`OntoCode: import patch failed — ${msg}`);
    }
  }
}
