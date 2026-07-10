import * as vscode from "vscode";
import {
  applyAxiomPatch,
  getEntity,
  parseManchester,
} from "../lsp/client";
import {
  hasPatchFailureDiagnostics,
  isPatchFullySynced,
  patchFailureMessage,
} from "../lsp/patchFeedback";
import { PanelHost } from "./panelHost";
import type { WebviewMessage } from "./messages";
import {
  ManchesterAxiomKind,
  buildManchesterPatches,
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
  private host: PanelHost;
  private options: ManchesterEditorOptions;
  private activeValidateSeq = 0;
  private bootstrapGeneration = 0;

  private constructor(host: PanelHost, options: ManchesterEditorOptions) {
    this.host = host;
    this.options = options;
    host.panel.onDidDispose(() => {
      ManchesterEditorPanel.current = undefined;
    });
  }

  public dispose(): void {
    this.host.panel.dispose();
  }

  public static async show(
    extensionUri: vscode.Uri,
    options: ManchesterEditorOptions
  ): Promise<ManchesterEditorPanel> {
    if (ManchesterEditorPanel.current) {
      ManchesterEditorPanel.current.options = options;
      ManchesterEditorPanel.current.host.panel.title = `Manchester: ${
        options.iri.split(/[#/]/).pop() ?? "entity"
      }`;
      ManchesterEditorPanel.current.host.panel.reveal(vscode.ViewColumn.Beside);
      await ManchesterEditorPanel.current.bootstrap();
      return ManchesterEditorPanel.current;
    }
    const host = PanelHost.create(extensionUri, {
      viewType: "ontocodeManchesterEditor",
      title: `Manchester: ${options.iri.split(/[#/]/).pop() ?? "entity"}`,
      panel: "manchesterEditor",
      onMessage: async (message: WebviewMessage) => {
        const panel = ManchesterEditorPanel.current;
        if (!panel) {
          return;
        }
        await panel.handleMessage(message);
      },
    });
    const instance = new ManchesterEditorPanel(host, options);
    ManchesterEditorPanel.current = instance;
    await instance.bootstrap();
    return instance;
  }

  private async bootstrap(): Promise<void> {
    const generation = ++this.bootstrapGeneration;
    const axiomKind = this.options.axiomKind ?? "sub_class_of";
    const expression = this.options.initialExpression ?? "";
    let completions = {
      classes: [] as string[],
      object_properties: [] as string[],
      data_properties: [] as string[],
      datatypes: [] as string[],
    };
    if (expression) {
      try {
        const parsed = await parseManchester({
          expression,
          axiom_kind: axiomKind,
          entity_iri: this.options.iri,
          document_uri: this.options.documentUri,
        });
        if (generation !== this.bootstrapGeneration) {
          return;
        }
        completions = parsed.completions;
      } catch {
        // optional
      }
    }
    if (generation !== this.bootstrapGeneration) {
      return;
    }
    this.host.postMessage({
      type: "manchesterInit",
      entityIri: this.options.iri,
      axiomKind,
      expression,
      completions,
    });
  }

  private async handleMessage(message: WebviewMessage): Promise<void> {
    if (message.type === "validateManchester") {
      await this.validate(message.expression, message.axiomKind, message.seq);
    }
    if (message.type === "applyManchester") {
      const parsed = (await import("./messages")).parseApplyManchesterMessage(message);
      if (!parsed) {
        void vscode.window.showErrorMessage(
          "OntoCode: ignored invalid applyManchester message from webview"
        );
        return;
      }
      await this.apply(
        parsed.expression,
        parsed.axiomKind,
        parsed.previewOnly
      );
    }
  }

  private async validate(
    expression: string,
    axiomKind: string,
    seq: number
  ): Promise<void> {
    this.activeValidateSeq = seq;
    if (axiomKind === "disjoint_class") {
      if (seq !== this.activeValidateSeq) {
        return;
      }
      this.host.postMessage({
        type: "manchesterValidation",
        seq,
        result: {
          normalized: expression,
          turtle_fragment: `    owl:disjointWith ${expression} ;\n`,
          tree: { kind: "DisjointClasses", other: expression },
          diagnostics: [],
        },
      });
      return;
    }
    try {
      const result = await parseManchester({
        expression,
        axiom_kind: axiomKind,
        entity_iri: this.options.iri,
        document_uri: this.options.documentUri,
      });
      if (seq !== this.activeValidateSeq) {
        return;
      }
      this.host.postMessage({
        type: "manchesterValidation",
        seq,
        result: {
          normalized: result.normalized,
          turtle_fragment: result.turtle_fragment,
          tree: result.tree,
          diagnostics: result.diagnostics,
        },
      });
    } catch (err) {
      const msg = err instanceof Error ? err.message : String(err);
      if (seq !== this.activeValidateSeq) {
        return;
      }
      this.host.postMessage({ type: "manchesterValidation", seq, error: msg });
    }
  }

  private async apply(
    expression: string,
    axiomKind: string,
    previewOnly: boolean
  ): Promise<void> {
    const mode = this.options.mode ?? "add";
    let patches;
    if (axiomKind === "disjoint_class") {
      patches = buildManchesterPatches(
        "disjoint_class",
        this.options.iri,
        expression,
        mode,
        this.options.initialExpression
      );
    } else {
      patches = buildManchesterPatches(
        axiomKind as ManchesterAxiomKind,
        this.options.iri,
        expression,
        mode,
        this.options.initialExpression
      );
    }
    try {
      const result = await applyAxiomPatch({
        document_uri: this.options.documentUri,
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
      if (!isPatchFullySynced(result)) {
        return;
      }
      if (this.options.onRefresh) {
        await this.options.onRefresh();
      }
      this.options.initialExpression = expression;
      void vscode.window.showInformationMessage(
        "OntoCode: Manchester axiom applied"
      );
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      void vscode.window.showErrorMessage(`OntoCode: ${message}`);
    }
  }
}
