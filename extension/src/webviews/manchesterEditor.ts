import * as vscode from "vscode";
import {
  applyAxiomPatch,
  getEntity,
  parseManchester,
} from "../lsp/client";
import { EntityDetail, PatchOp } from "../lsp/protocol";
import {
  ManchesterAxiomKind,
  buildManchesterPatch,
} from "./manchesterEditorLogic";

export interface ManchesterEditorOptions {
  iri: string;
  documentUri: string;
  axiomKind?: ManchesterAxiomKind;
  initialExpression?: string;
  mode?: "add" | "edit";
  onRefresh?: () => Promise<void>;
}

export class ManchesterEditorPanel {
  public static current: ManchesterEditorPanel | undefined;
  private readonly panel: vscode.WebviewPanel;
  private options: ManchesterEditorOptions;
  private completions: {
    classes: string[];
    object_properties: string[];
    data_properties: string[];
    datatypes: string[];
  } = { classes: [], object_properties: [], data_properties: [], datatypes: [] };

  private constructor(
    panel: vscode.WebviewPanel,
    options: ManchesterEditorOptions
  ) {
    this.panel = panel;
    this.options = options;
    this.panel.onDidDispose(() => {
      ManchesterEditorPanel.current = undefined;
    });
    this.panel.webview.onDidReceiveMessage(async (msg) => {
      if (msg.command === "validate") {
        await this.validate(msg.expression, msg.axiomKind);
      }
      if (msg.command === "preview") {
        await this.apply(msg.expression, msg.axiomKind, true);
      }
      if (msg.command === "apply") {
        await this.apply(msg.expression, msg.axiomKind, false);
      }
    });
    void this.bootstrap();
  }

  public static async show(
    options: ManchesterEditorOptions
  ): Promise<ManchesterEditorPanel> {
    if (ManchesterEditorPanel.current) {
      ManchesterEditorPanel.current.options = options;
      ManchesterEditorPanel.current.panel.reveal(vscode.ViewColumn.Beside);
      await ManchesterEditorPanel.current.bootstrap();
      return ManchesterEditorPanel.current;
    }
    const panel = vscode.window.createWebviewPanel(
      "ontocodeManchesterEditor",
      `Manchester: ${options.iri.split(/[#/]/).pop() ?? "entity"}`,
      vscode.ViewColumn.Beside,
      { enableScripts: true, retainContextWhenHidden: true }
    );
    ManchesterEditorPanel.current = new ManchesterEditorPanel(panel, options);
    return ManchesterEditorPanel.current;
  }

  private async bootstrap(): Promise<void> {
    try {
      const expr = this.options.initialExpression ?? "";
      const axiomKind = this.options.axiomKind ?? "sub_class_of";
      if (expr) {
        const parsed = await parseManchester({
          expression: expr,
          axiom_kind: axiomKind,
          entity_iri: this.options.iri,
          document_uri: this.options.documentUri,
        });
        this.completions = parsed.completions;
      }
    } catch {
      // completions optional on open
    }
    this.panel.webview.html = this.renderHtml();
  }

  private async validate(
    expression: string,
    axiomKind: ManchesterAxiomKind
  ): Promise<void> {
    try {
      const result = await parseManchester({
        expression,
        axiom_kind: axiomKind,
        entity_iri: this.options.iri,
        document_uri: this.options.documentUri,
      });
      this.completions = result.completions;
      this.panel.webview.postMessage({
        command: "validated",
        result,
      });
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      this.panel.webview.postMessage({
        command: "validated",
        error: message,
      });
    }
  }

  private async apply(
    expression: string,
    axiomKind: ManchesterAxiomKind,
    previewOnly: boolean
  ): Promise<void> {
    const mode = this.options.mode ?? "add";
    const patch: PatchOp = buildManchesterPatch(
      axiomKind,
      this.options.iri,
      expression,
      mode === "edit" ? "set" : "add"
    );
    if (axiomKind === "sub_class_of" && mode === "add") {
      // use add_complex_sub_class_of via buildManchesterPatch
    }
    try {
      const result = await applyAxiomPatch({
        document_uri: this.options.documentUri,
        patches: [patch],
        preview_only: previewOnly,
      });
      if (previewOnly && result.preview_text) {
        this.panel.webview.postMessage({
          command: "preview",
          text: result.preview_text,
        });
        return;
      }
      if (!previewOnly) {
        if (this.options.onRefresh) {
          await this.options.onRefresh();
        }
        const { detail } = await getEntity(this.options.iri);
        this.options.initialExpression = expression;
        void vscode.window.showInformationMessage("OntoCode: Manchester axiom applied");
        this.panel.webview.postMessage({ command: "applied", detail });
      }
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      void vscode.window.showErrorMessage(`OntoCode: ${message}`);
    }
  }

  private renderHtml(): string {
    const expr = escapeHtml(this.options.initialExpression ?? "");
    const axiomKind = this.options.axiomKind ?? "sub_class_of";
    const pick = (items: string[]) =>
      items
        .slice(0, 40)
        .map((i) => `<option value="${escapeHtml(i)}">${escapeHtml(i)}</option>`)
        .join("");

    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <style>
    body { font-family: var(--vscode-font-family); color: var(--vscode-foreground); background: var(--vscode-editor-background); padding: 12px; }
    textarea { width: 100%; min-height: 80px; font-family: var(--vscode-editor-font-family); }
    button { margin: 4px 4px 4px 0; padding: 4px 10px; }
    pre { white-space: pre-wrap; border: 1px solid var(--vscode-panel-border); padding: 8px; max-height: 160px; overflow: auto; }
    .row { display: flex; gap: 8px; flex-wrap: wrap; margin: 8px 0; }
    .error { color: var(--vscode-errorForeground); }
  </style>
</head>
<body>
  <h2>Manchester Axiom Editor</h2>
  <p><code>${escapeHtml(this.options.iri)}</code></p>
  <label>Axiom type
    <select id="axiomKind">
      <option value="sub_class_of" ${axiomKind === "sub_class_of" ? "selected" : ""}>SubClassOf</option>
      <option value="equivalent_class" ${axiomKind === "equivalent_class" ? "selected" : ""}>EquivalentClasses</option>
    </select>
  </label>
  <h3>Expression</h3>
  <textarea id="expression">${expr}</textarea>
  <div class="row">
    <label>Insert class <select id="classPick"><option value="">—</option>${pick(this.completions.classes)}</select></label>
    <label>Object prop <select id="objPick"><option value="">—</option>${pick(this.completions.object_properties)}</select></label>
    <label>Data prop <select id="dataPick"><option value="">—</option>${pick(this.completions.data_properties)}</select></label>
    <label>Datatype <select id="dtPick"><option value="">—</option>${pick(this.completions.datatypes)}</select></label>
  </div>
  <button id="insertClass">Insert</button>
  <button id="validate">Validate</button>
  <button id="preview">Preview Turtle</button>
  <button id="apply">Apply</button>
  <div id="diag" class="error"></div>
  <h3>Expression tree</h3>
  <pre id="tree"></pre>
  <h3>Turtle preview</h3>
  <pre id="turtlePreview"></pre>
  <script>
    const vscode = acquireVsCodeApi();
    const exprEl = document.getElementById('expression');
    const axiomEl = document.getElementById('axiomKind');

    function insertFrom(selectId) {
      const sel = document.getElementById(selectId);
      if (!sel.value) return;
      const term = sel.value.includes('#') || sel.value.includes('/') ? '<' + sel.value + '>' : sel.value.split('/').pop();
      const short = sel.selectedOptions[0].textContent;
      const insert = short.includes(':') ? short : term;
      const start = exprEl.selectionStart;
      const end = exprEl.selectionEnd;
      exprEl.value = exprEl.value.slice(0, start) + insert + exprEl.value.slice(end);
    }
    document.getElementById('insertClass').addEventListener('click', () => {
      for (const id of ['classPick','objPick','dataPick','dtPick']) {
        const sel = document.getElementById(id);
        if (sel.value) { insertFrom(id); break; }
      }
    });

    let validateTimer;
    exprEl.addEventListener('input', () => {
      clearTimeout(validateTimer);
      validateTimer = setTimeout(() => {
        vscode.postMessage({ command: 'validate', expression: exprEl.value, axiomKind: axiomEl.value });
      }, 500);
    });

    document.getElementById('validate').addEventListener('click', () => {
      vscode.postMessage({ command: 'validate', expression: exprEl.value, axiomKind: axiomEl.value });
    });
    document.getElementById('preview').addEventListener('click', () => {
      vscode.postMessage({ command: 'preview', expression: exprEl.value, axiomKind: axiomEl.value });
    });
    document.getElementById('apply').addEventListener('click', () => {
      vscode.postMessage({ command: 'apply', expression: exprEl.value, axiomKind: axiomEl.value });
    });

    window.addEventListener('message', (event) => {
      const msg = event.data;
      if (msg.command === 'validated') {
        document.getElementById('diag').textContent = msg.error || (msg.result?.diagnostics?.map(d => d.message).join('; ') || '');
        document.getElementById('tree').textContent = msg.result ? JSON.stringify(msg.result.tree, null, 2) : '';
        document.getElementById('turtlePreview').textContent = msg.result?.turtle_fragment || '';
      }
      if (msg.command === 'preview') {
        document.getElementById('turtlePreview').textContent = msg.text || '';
      }
    });
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
