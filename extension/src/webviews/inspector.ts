import * as vscode from "vscode";
import { applyAxiomPatch, getEntity } from "../lsp/client";
import { EntityDetail, PatchOp } from "../lsp/protocol";
import { entityKindLabel, shortLabel } from "../utils/iri";

type RefreshFn = () => Promise<void>;

export class EntityInspectorPanel {
  public static currentPanel: EntityInspectorPanel | undefined;
  private readonly panel: vscode.WebviewPanel;
  private iri: string | undefined;
  private documentUri: string | undefined;
  private classOptions: string[] = [];

  private constructor(
    panel: vscode.WebviewPanel,
    private readonly extensionUri: vscode.Uri,
    private readonly onRefresh?: RefreshFn
  ) {
    this.panel = panel;
    this.panel.onDidDispose(() => {
      EntityInspectorPanel.currentPanel = undefined;
    });
    this.panel.webview.onDidReceiveMessage(async (message) => {
      if (message.command === "jumpToSource" && this.iri) {
        await vscode.commands.executeCommand(
          "ontocode.jumpToSource",
          this.iri
        );
      }
      if (message.command === "applyPatch" && this.documentUri) {
        await this.runPatch(
          message.patches as PatchOp[],
          Boolean(message.previewOnly)
        );
      }
    });
  }

  public static show(
    extensionUri: vscode.Uri,
    detail: EntityDetail,
    classOptions: string[] = [],
    providers?: RefreshFn
  ): EntityInspectorPanel {
    if (EntityInspectorPanel.currentPanel) {
      EntityInspectorPanel.currentPanel.panel.reveal(
        vscode.ViewColumn.Beside
      );
      EntityInspectorPanel.currentPanel.update(detail, classOptions);
      return EntityInspectorPanel.currentPanel;
    }

    const panel = vscode.window.createWebviewPanel(
      "ontocodeInspector",
      `Entity: ${detail.entity.short_name || shortLabel(detail.entity.iri)}`,
      vscode.ViewColumn.Beside,
      { enableScripts: true, retainContextWhenHidden: true }
    );

    EntityInspectorPanel.currentPanel = new EntityInspectorPanel(
      panel,
      extensionUri,
      providers
    );
    EntityInspectorPanel.currentPanel.update(detail, classOptions);
    return EntityInspectorPanel.currentPanel;
  }

  public update(detail: EntityDetail, classOptions: string[] = []): void {
    this.iri = detail.entity.iri;
    this.classOptions = classOptions;
    if (detail.document_path) {
      this.documentUri = vscode.Uri.file(detail.document_path).toString();
    }
    this.panel.title = `${entityKindLabel(detail.entity.kind)}: ${
      detail.entity.labels[0] ?? detail.entity.short_name
    }`;
    this.panel.webview.html = this.renderHtml(detail);
  }

  private async runPatch(
    patches: PatchOp[],
    previewOnly: boolean
  ): Promise<void> {
    if (!this.documentUri) {
      void vscode.window.showErrorMessage("No editable Turtle document for this entity");
      return;
    }
    try {
      const result = await applyAxiomPatch({
        document_uri: this.documentUri,
        patches,
        preview_only: previewOnly,
      });
      if (previewOnly && result.preview_text) {
        this.panel.webview.postMessage({
          command: "preview",
          text: result.preview_text,
        });
        return;
      }
      if (result.entity_detail && this.iri) {
        this.update(result.entity_detail, this.classOptions);
      } else if (this.iri) {
        const { detail } = await getEntity(this.iri);
        this.update(detail, this.classOptions);
      }
      if (this.onRefresh) {
        await this.onRefresh();
      }
      if (!previewOnly) {
        void vscode.window.showInformationMessage("OntoCode: changes applied");
      }
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      void vscode.window.showErrorMessage(`OntoCode: patch failed — ${message}`);
    }
  }

  private renderHtml(detail: EntityDetail): string {
    const { entity, parents, children, axioms, editable } = detail;
    const list = (items: string[]) =>
      items.length > 0
        ? `<ul>${items.map((i) => `<li><code>${escapeHtml(shortLabel(i))}</code> <span class="muted">${escapeHtml(i)}</span></li>`).join("")}</ul>`
        : `<p class="muted">None</p>`;

    const parentOptions = this.classOptions
      .filter((c) => c !== entity.iri)
      .map(
        (c) =>
          `<option value="${escapeHtml(c)}">${escapeHtml(shortLabel(c))}</option>`
      )
      .join("");

    const editSection = editable
      ? `
  <h2>Edit</h2>
  <div class="form">
    <label>Add label <input id="newLabel" type="text" /></label>
    <button id="addLabel">Add Label</button>
    <label>Add comment <input id="newComment" type="text" /></label>
    <button id="addComment">Add Comment</button>
    <label>Add parent
      <select id="parentPick"><option value="">—</option>${parentOptions}</select>
    </label>
    <button id="addParent">Add Parent (SubClassOf)</button>
    <button id="previewPatch" class="secondary">Preview</button>
    <button id="deleteEntity" class="danger">Delete Entity</button>
    <pre id="preview" class="preview muted"></pre>
  </div>`
      : `<p class="muted">Editing is available for Turtle (.ttl) documents only.</p>`;

    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <style>
    body { font-family: var(--vscode-font-family); color: var(--vscode-foreground); background: var(--vscode-editor-background); padding: 16px; }
    h1 { font-size: 1.2rem; margin-bottom: 0.25rem; }
    h2 { font-size: 0.95rem; margin-top: 1.25rem; border-bottom: 1px solid var(--vscode-panel-border); padding-bottom: 4px; }
    code { font-family: var(--vscode-editor-font-family); }
    .muted { color: var(--vscode-descriptionForeground); font-size: 0.85rem; }
    .iri { word-break: break-all; }
    button { background: var(--vscode-button-background); color: var(--vscode-button-foreground); border: none; padding: 6px 12px; cursor: pointer; margin: 4px 4px 4px 0; }
    button:hover { background: var(--vscode-button-hoverBackground); }
    button.secondary { background: var(--vscode-button-secondaryBackground); color: var(--vscode-button-secondaryForeground); }
    button.danger { background: var(--vscode-inputValidation-errorBackground); }
    .deprecated { color: var(--vscode-errorForeground); font-weight: bold; }
    .form label { display: block; margin: 8px 0 4px; }
    .form input, .form select { width: 100%; box-sizing: border-box; }
    .preview { max-height: 200px; overflow: auto; white-space: pre-wrap; border: 1px solid var(--vscode-panel-border); padding: 8px; margin-top: 8px; }
  </style>
</head>
<body>
  <h1>${escapeHtml(entity.labels[0] ?? entity.short_name)}</h1>
  <p class="muted">${escapeHtml(entityKindLabel(entity.kind))}${entity.deprecated ? ' <span class="deprecated">(deprecated)</span>' : ""}</p>

  <h2>IRI</h2>
  <p class="iri"><code>${escapeHtml(entity.iri)}</code></p>

  <h2>Labels</h2>
  ${list(entity.labels)}

  <h2>Comments</h2>
  ${list(entity.comments)}

  <h2>Parents</h2>
  ${list(parents)}

  <h2>Children</h2>
  ${list(children)}

  <h2>Axioms</h2>
  ${list(axioms)}

  ${editSection}

  <button id="jump">Jump to Source</button>
  <script>
    const vscode = acquireVsCodeApi();
    const entityIri = ${JSON.stringify(entity.iri)};

    document.getElementById('jump').addEventListener('click', () => {
      vscode.postMessage({ command: 'jumpToSource' });
    });

    function apply(patches, previewOnly) {
      vscode.postMessage({ command: 'applyPatch', patches, previewOnly });
    }

    window.addEventListener('message', (event) => {
      if (event.data.command === 'preview') {
        document.getElementById('preview').textContent = event.data.text || '';
      }
    });

    const addLabel = document.getElementById('addLabel');
    if (addLabel) {
      addLabel.addEventListener('click', () => {
        const value = document.getElementById('newLabel').value.trim();
        if (!value) return;
        apply([{ op: 'add_label', entity_iri: entityIri, value }], false);
      });
    }
    const addComment = document.getElementById('addComment');
    if (addComment) {
      addComment.addEventListener('click', () => {
        const value = document.getElementById('newComment').value.trim();
        if (!value) return;
        apply([{ op: 'add_comment', entity_iri: entityIri, value }], false);
      });
    }
    const addParent = document.getElementById('addParent');
    if (addParent) {
      addParent.addEventListener('click', () => {
        const parent = document.getElementById('parentPick').value;
        if (!parent) return;
        apply([{ op: 'add_sub_class_of', entity_iri: entityIri, parent_iri: parent }], false);
      });
    }
    const previewPatch = document.getElementById('previewPatch');
    if (previewPatch) {
      previewPatch.addEventListener('click', () => {
        const value = document.getElementById('newLabel').value.trim();
        if (!value) return;
        apply([{ op: 'add_label', entity_iri: entityIri, value }], true);
      });
    }
    const deleteEntity = document.getElementById('deleteEntity');
    if (deleteEntity) {
      deleteEntity.addEventListener('click', () => {
        if (confirm('Delete this entity from the ontology file?')) {
          apply([{ op: 'delete_entity', entity_iri: entityIri }], false);
        }
      });
    }
  </script>
</body>
</html>`;
  }
}

function escapeHtml(s: string): string {
  return s
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}
