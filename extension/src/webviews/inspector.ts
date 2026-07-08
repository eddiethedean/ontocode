import * as vscode from "vscode";
import { applyAxiomPatch, getEntity } from "../lsp/client";
import {
  hasPatchFailureDiagnostics,
  isPatchFullySynced,
  patchFailureMessage,
} from "../lsp/patchFeedback";
import { EntityDetail, PatchOp } from "../lsp/protocol";
import { entityKindLabel } from "../utils/iri";
import { documentUriInWorkspace } from "../utils/workspacePath";
import { PanelHost } from "./panelHost";
import type { EntityDetailPayload, WebviewMessage } from "./messages";
import { GraphPanel } from "./graphPanel";
import { acceptInspectorRevealRequest } from "./inspectorReveal";

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
    annotations: detail.annotations,
    characteristics: detail.characteristics,
    editable: detail.editable,
    document_path: detail.document_path,
  };
}

export class EntityInspectorPanel {
  public static currentPanel: EntityInspectorPanel | undefined;
  private host: PanelHost;
  private readonly extensionUri: vscode.Uri;
  private iri: string | undefined;
  private oboId: string | undefined;
  private documentUri: string | undefined;
  private classOptions: string[] = [];
  private objectPropertyOptions: string[] = [];
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
    objectPropertyOptions: string[] = [],
    onRefresh?: RefreshFn,
    requestId?: number
  ): EntityInspectorPanel {
    if (EntityInspectorPanel.currentPanel) {
      const existing = EntityInspectorPanel.currentPanel;
      if (!existing.isWebviewReady()) {
        existing.disposeForTests();
        EntityInspectorPanel.currentPanel = undefined;
      } else {
        existing.reveal(detail, classOptions, objectPropertyOptions, requestId);
        existing.host.panel.reveal();
        return existing;
      }
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
    instance.reveal(detail, classOptions, objectPropertyOptions, requestId);
    return instance;
  }

  private reveal(
    detail: EntityDetail,
    classOptions: string[] = [],
    objectPropertyOptions: string[] = [],
    requestId?: number
  ): void {
    if (!acceptInspectorRevealRequest(this.activeRequestId, requestId)) {
      return;
    }
    if (requestId !== undefined) {
      this.activeRequestId = requestId;
    }
    this.iri = detail.entity.iri;
    this.oboId = detail.entity.obo_id;
    this.classOptions = classOptions;
    this.objectPropertyOptions = objectPropertyOptions;
    this.documentUri = detail.document_path
      ? documentUriInWorkspace(detail.document_path)
      : undefined;
    this.host.panel.title = panelTitle(detail);
    this.host.postMessage({
      type: "loadEntity",
      detail: toPayload(detail),
      classOptions,
      objectPropertyOptions,
    });
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "jumpToSource" && this.iri) {
      await vscode.commands.executeCommand("ontocode.jumpToSource", this.iri);
    }
    if (message.type === "applyPatch" && this.documentUri) {
      const { parseApplyPatchMessage } = await import("./messages");
      const parsed = parseApplyPatchMessage(message, this.iri, this.oboId);
      if (!parsed) {
        void vscode.window.showErrorMessage(
          "OntoCode: ignored invalid applyPatch message from webview"
        );
        return;
      }
      await this.runPatch(parsed.patches, parsed.previewOnly);
    }
    if (message.type === "openManchester" && this.iri && this.documentUri) {
      const axiomKind = message.axiom.kind;
      const initialExpression =
        axiomKind === "disjoint_class"
          ? (message.axiom.other_iri ?? message.axiom.manchester ?? "")
          : (message.axiom.manchester ?? "");
      await vscode.commands.executeCommand("ontocode.openManchesterEditor", {
        iri: this.iri,
        documentUri: this.documentUri,
        axiomKind,
        initialExpression,
        mode: initialExpression ? "edit" : "add",
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
      const fromIri = this.iri;
      await renameEntityIri(this.extensionUri, fromIri, async (newIri) => {
        if (newIri) {
          await vscode.commands.executeCommand("ontocode.openEntity", newIri);
        } else if (fromIri) {
          await this.loadEntity(fromIri);
        }
      });
    }
  }

  private async loadEntity(iri: string): Promise<void> {
    const iriAtStart = this.iri;
    const requestId = ++this.activeRequestId;
    const { detail } = await getEntity(iri);
    if (iriAtStart !== this.iri) {
      return;
    }
    this.reveal(detail, this.classOptions, this.objectPropertyOptions, requestId);
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
    const requestId = ++this.activeRequestId;
    try {
      const result = await applyAxiomPatch({
        document_uri: this.documentUri,
        patches,
        preview_only: previewOnly,
      });
      if (iriAtStart !== this.iri) {
        return;
      }
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
      if (!isPatchFullySynced(result)) {
        return;
      }
      const deleted = patches.some((p) => p.op === "delete_entity");
      if (deleted) {
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
      if (result.entity_detail) {
        if (result.entity_detail.entity.iri !== this.iri) {
          return;
        }
        this.reveal(result.entity_detail, this.classOptions, this.objectPropertyOptions, requestId);
      } else if (this.iri) {
        const { detail } = await getEntity(this.iri);
        if (iriAtStart !== this.iri) {
          return;
        }
        this.reveal(detail, this.classOptions, this.objectPropertyOptions, requestId);
      }
      if (this.onRefresh) {
        await this.onRefresh();
      }
      void vscode.window.showInformationMessage("OntoCode: changes applied");
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      void vscode.window.showErrorMessage(`OntoCode: patch failed — ${msg}`);
    }
  }

  isWebviewReady(): boolean {
    return this.host.isWebviewReady();
  }

  /** @internal VS Code integration tests */
  getWebviewHtmlForTests(): string {
    return this.host.getWebviewHtml();
  }

  isWebviewReadyForTests(): boolean {
    return this.host.isWebviewReady();
  }

    disposeForTests(): void {
    if (!this.host.isDisposed) {
      this.host.panel.dispose();
    }
  }

  /** @internal VS Code integration tests */
  getLoadedIriForTests(): string | undefined {
    return this.iri;
  }

  /** @internal VS Code integration tests */
  getPanelTitleForTests(): string {
    return this.host.panel.title;
  }
}

function panelTitle(detail: EntityDetail): string {
  return `${entityKindLabel(detail.entity.kind)}: ${
    detail.entity.labels[0] ?? detail.entity.short_name
  }`;
}
