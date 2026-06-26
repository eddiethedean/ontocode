import * as vscode from "vscode";
import { applyAxiomPatch, getEntity } from "../lsp/client";
import { EntityDetail, PatchOp } from "../lsp/protocol";
import { entityKindLabel } from "../utils/iri";
import { PanelHost } from "./panelHost";
import type { EntityDetailPayload, WebviewMessage } from "./messages";
import { GraphPanel } from "./graphPanel";

type RefreshFn = () => Promise<void>;

function toPayload(detail: EntityDetail): EntityDetailPayload {
  return {
    entity: {
      iri: detail.entity.iri,
      short_name: detail.entity.short_name,
      kind: detail.entity.kind,
      labels: detail.entity.labels,
      comments: detail.entity.comments,
      deprecated: detail.entity.deprecated,
      obo_id: (detail.entity as { obo_id?: string }).obo_id,
    },
    parents: detail.parents,
    children: detail.children,
    axioms: detail.axioms,
    editable: detail.editable,
    document_path: detail.document_path,
  };
}

export class EntityInspectorPanel {
  public static currentPanel: EntityInspectorPanel | undefined;
  private host: PanelHost;
  private readonly extensionUri: vscode.Uri;
  private iri: string | undefined;
  private documentUri: string | undefined;
  private classOptions: string[] = [];
  private activeRequestId = 0;

  private constructor(
    host: PanelHost,
    extensionUri: vscode.Uri,
    private readonly onRefresh?: RefreshFn
  ) {
    this.host = host;
    this.extensionUri = extensionUri;
    host.panel.onDidDispose(() => {
      EntityInspectorPanel.currentPanel = undefined;
    });
  }

  public static show(
    extensionUri: vscode.Uri,
    detail: EntityDetail,
    classOptions: string[] = [],
    onRefresh?: RefreshFn,
    requestId?: number
  ): EntityInspectorPanel {
    if (EntityInspectorPanel.currentPanel) {
      EntityInspectorPanel.currentPanel.reveal(detail, classOptions, requestId);
      return EntityInspectorPanel.currentPanel;
    }

    const host = PanelHost.create(extensionUri, {
      viewType: "ontocodeInspector",
      title: panelTitle(detail),
      panel: "inspector",
      onMessage: async (message: WebviewMessage) => {
        const panel = EntityInspectorPanel.currentPanel;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });

    const instance = new EntityInspectorPanel(host, extensionUri, onRefresh);
    EntityInspectorPanel.currentPanel = instance;
    instance.reveal(detail, classOptions, requestId);
    return instance;
  }

  private reveal(
    detail: EntityDetail,
    classOptions: string[] = [],
    requestId?: number
  ): void {
    if (requestId !== undefined && requestId !== this.activeRequestId && this.activeRequestId !== 0) {
      return;
    }
    if (requestId !== undefined) {
      this.activeRequestId = requestId;
    }
    this.iri = detail.entity.iri;
    this.classOptions = classOptions;
    this.documentUri = detail.document_path
      ? vscode.Uri.file(detail.document_path).toString()
      : undefined;
    this.host.panel.title = panelTitle(detail);
    this.host.postMessage({
      type: "loadEntity",
      detail: toPayload(detail),
      classOptions,
    });
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "jumpToSource" && this.iri) {
      await vscode.commands.executeCommand("ontocode.jumpToSource", this.iri);
    }
    if (message.type === "applyPatch" && this.documentUri) {
      await this.runPatch(message.patches as PatchOp[], message.previewOnly);
    }
    if (message.type === "openManchester" && this.iri && this.documentUri) {
      await vscode.commands.executeCommand("ontocode.openManchesterEditor", {
        iri: this.iri,
        documentUri: this.documentUri,
        axiomKind: message.axiom.kind,
        initialExpression: message.axiom.manchester ?? "",
        mode: message.axiom.manchester ? "edit" : "add",
      });
    }
    if (message.type === "addManchesterAxiom" && this.iri && this.documentUri) {
      await vscode.commands.executeCommand("ontocode.openManchesterEditor", {
        iri: this.iri,
        documentUri: this.documentUri,
        mode: "add",
      });
    }
    if (message.type === "openGraph") {
      await GraphPanel.show(this.extensionUri, {
        graphKind: "neighborhood",
        rootIri: message.rootIri ?? this.iri,
      });
    }
    if (message.type === "selectNode" || message.type === "openEntity") {
      await vscode.commands.executeCommand("ontocode.openEntity", message.iri);
    }
    if (message.type === "findUsages" && this.iri) {
      const { showEntityUsages } = await import("./refactorPreview");
      await showEntityUsages(this.iri);
    }
    if (message.type === "renameIri" && this.iri) {
      const { renameEntityIri } = await import("./refactorPreview");
      await renameEntityIri(this.extensionUri, this.iri);
    }
  }

  private async runPatch(
    patches: PatchOp[],
    previewOnly: boolean
  ): Promise<void> {
    if (!this.documentUri) {
      void vscode.window.showErrorMessage(
        "No editable document for this entity"
      );
      return;
    }
    const iriAtStart = this.iri;
    try {
      const result = await applyAxiomPatch({
        document_uri: this.documentUri,
        patches,
        preview_only: previewOnly,
      });
      if (iriAtStart !== this.iri) {
        return;
      }
      if (previewOnly && result.preview_text) {
        this.host.postMessage({ type: "preview", text: result.preview_text });
        return;
      }
      if (!previewOnly) {
        const deleted = patches.some((p) => p.op === "delete_entity");
        if (deleted && result.applied) {
          this.host.panel.dispose();
          EntityInspectorPanel.currentPanel = undefined;
          if (this.onRefresh) {
            await this.onRefresh();
          }
          void vscode.window.showInformationMessage("OntoCode: entity deleted");
          return;
        }
        if (result.reindex_warning) {
          void vscode.window.showWarningMessage(
            `OntoCode: changes saved but reindex failed — ${result.reindex_warning}`
          );
        }
      }
      if (result.entity_detail) {
        if (result.entity_detail.entity.iri !== this.iri) {
          return;
        }
        this.reveal(result.entity_detail, this.classOptions);
      } else if (this.iri) {
        const { detail } = await getEntity(this.iri);
        if (iriAtStart !== this.iri) {
          return;
        }
        this.reveal(detail, this.classOptions);
      }
      if (this.onRefresh) {
        await this.onRefresh();
      }
      if (!previewOnly) {
        void vscode.window.showInformationMessage("OntoCode: changes applied");
      }
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      void vscode.window.showErrorMessage(`OntoCode: patch failed — ${msg}`);
    }
  }
}

function panelTitle(detail: EntityDetail): string {
  return `${entityKindLabel(detail.entity.kind)}: ${
    detail.entity.labels[0] ?? detail.entity.short_name
  }`;
}
