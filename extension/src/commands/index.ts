import * as vscode from "vscode";
import {
  applyAxiomPatch,
  getCatalogSnapshot,
  getEntity,
  indexWorkspace,
} from "../lsp/client";
import { PatchEntityKind, PatchOp } from "../lsp/protocol";
import { EntityInspectorPanel } from "../webviews/inspector";
import { GraphPanel } from "../webviews/graphPanel";
import { QueryWorkbenchPanel } from "../webviews/queryWorkbenchReact";
import {
  ManchesterEditorPanel,
  ManchesterEditorOptions,
} from "../webviews/manchesterEditorReact";
import { ReasonerPanel } from "../webviews/reasonerPanel";
import { ExplanationPanel } from "../webviews/explanationPanel";
import { SemanticDiffPanel } from "../webviews/semanticDiffPanel";
import {
  extractModule,
  migrateNamespace,
  moveEntity,
  renameEntityIri,
  showEntityUsages,
} from "../webviews/refactorPreview";
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
            await openInspector(context.extensionUri, iri, async () =>
              refreshExplorer(providers)
            );
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
          await openInspector(context.extensionUri, iri, async () =>
            refreshExplorer(providers)
          );
        } catch (err) {
          const message = err instanceof Error ? err.message : String(err);
          void vscode.window.showErrorMessage(
            `OntoCode: could not open entity — ${message}`
          );
        }
      }
    ),
    vscode.commands.registerCommand(
      "ontocode.openDiagnostic",
      async (diagnostic: import("../lsp/protocol").DiagnosticSummary) => {
        if (!diagnostic?.file || typeof diagnostic.file !== "string") {
          return;
        }
        const uri = vscode.Uri.file(diagnostic.file);
        if (!vscode.workspace.getWorkspaceFolder(uri)) {
          void vscode.window.showErrorMessage(
            "OntoCode: diagnostic path is outside the workspace"
          );
          return;
        }
        const doc = await vscode.workspace.openTextDocument(uri);
        const editor = await vscode.window.showTextDocument(doc);
        if (diagnostic.line != null) {
          const line = Math.max(0, diagnostic.line - 1);
          const lineText = doc.lineAt(line).text;
          const col = byteColToUtf16(lineText, diagnostic.column ?? 0);
          const pos = new vscode.Position(line, col);
          editor.selection = new vscode.Selection(pos, pos);
          editor.revealRange(
            new vscode.Range(pos, pos),
            vscode.TextEditorRevealType.InCenter
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
    ),
    vscode.commands.registerCommand("ontocode.createClass", async () => {
      await createEntity(context, providers, "class");
    }),
    vscode.commands.registerCommand("ontocode.createProperty", async () => {
      const kind = await vscode.window.showQuickPick(
        [
          { label: "Object property", value: "object_property" as PatchEntityKind },
          { label: "Data property", value: "data_property" as PatchEntityKind },
          { label: "Annotation property", value: "annotation_property" as PatchEntityKind },
        ],
        { placeHolder: "Property kind" }
      );
      if (kind) {
        await createEntity(context, providers, kind.value);
      }
    }),
    vscode.commands.registerCommand("ontocode.createIndividual", async () => {
      await createEntity(context, providers, "individual");
    }),
    vscode.commands.registerCommand("ontocode.deleteEntity", async (arg?: unknown) => {
      const iri = resolveEntityIri(arg);
      if (!iri) {
        return;
      }
      const confirm = await vscode.window.showWarningMessage(
        `Delete entity ${iri}?`,
        { modal: true },
        "Delete"
      );
      if (confirm !== "Delete") {
        return;
      }
      try {
        const { detail } = await getEntity(iri);
        if (!detail.document_path) {
          void vscode.window.showErrorMessage("Entity is not in an editable Turtle file");
          return;
        }
        await applyAxiomPatch({
          document_uri: vscode.Uri.file(detail.document_path).toString(),
          patches: [{ op: "delete_entity", entity_iri: iri }],
          preview_only: false,
        });
        await refreshExplorer(providers);
        void vscode.window.showInformationMessage("Entity deleted");
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        void vscode.window.showErrorMessage(`Delete failed: ${message}`);
      }
    }),
    vscode.commands.registerCommand(
      "ontocode.findEntityUsages",
      async (iri?: string) => {
        const target =
          resolveEntityIri(iri) ??
          (await vscode.window.showInputBox({ prompt: "Entity IRI" }));
        if (!target) {
          return;
        }
        try {
          await showEntityUsages(target);
        } catch (err) {
          const message = err instanceof Error ? err.message : String(err);
          void vscode.window.showErrorMessage(message);
        }
      }
    ),
    vscode.commands.registerCommand(
      "ontocode.renameEntityIri",
      async (iri?: string) => {
        try {
          await renameEntityIri(
            context.extensionUri,
            resolveEntityIri(iri) ?? iri,
            () => refreshExplorer(providers)
          );
        } catch (err) {
          const message = err instanceof Error ? err.message : String(err);
          void vscode.window.showErrorMessage(message);
        }
      }
    ),
    vscode.commands.registerCommand("ontocode.migrateNamespace", async () => {
      try {
        await migrateNamespace(context.extensionUri, () =>
          refreshExplorer(providers)
        );
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        void vscode.window.showErrorMessage(message);
      }
    }),
    vscode.commands.registerCommand(
      "ontocode.moveEntity",
      async (iri?: string) => {
        try {
          await moveEntity(
            context.extensionUri,
            resolveEntityIri(iri) ?? iri,
            () => refreshExplorer(providers)
          );
        } catch (err) {
          const message = err instanceof Error ? err.message : String(err);
          void vscode.window.showErrorMessage(message);
        }
      }
    ),
    vscode.commands.registerCommand("ontocode.extractModule", async () => {
      try {
        await extractModule(context.extensionUri, () =>
          refreshExplorer(providers)
        );
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        void vscode.window.showErrorMessage(message);
      }
    }),
    vscode.commands.registerCommand("ontocode.openQueryWorkbench", () => {
      QueryWorkbenchPanel.show(context);
    }),
    vscode.commands.registerCommand(
      "ontocode.openManchesterEditor",
      async (arg?: ManchesterEditorOptions) => {
        if (!arg?.iri || !arg.documentUri) {
          void vscode.window.showErrorMessage(
            "OntoCode: Manchester editor requires entity IRI and document URI"
          );
          return;
        }
        await ManchesterEditorPanel.show(context.extensionUri, {
          ...arg,
          onRefresh: async () => refreshExplorer(providers),
        });
      }
    ),
    vscode.commands.registerCommand(
      "ontocode.addManchesterAxiom",
      async (arg?: ManchesterEditorOptions) => {
        await vscode.commands.executeCommand(
          "ontocode.openManchesterEditor",
          arg
        );
      }
    ),
    vscode.commands.registerCommand("ontocode.runReasoner", async () => {
      const panel = ReasonerPanel.show();
      await panel.runWithDefaults();
    }),
    vscode.commands.registerCommand("ontocode.semanticDiff", async () => {
      try {
        const leftRef = await vscode.window.showInputBox({
          prompt: "Left git ref (or WORKSPACE)",
          value: "HEAD",
        });
        if (leftRef === undefined) {
          return;
        }
        const rightRef = await vscode.window.showInputBox({
          prompt: "Right git ref (WORKTREE or WORKSPACE)",
          value: "WORKSPACE",
        });
        if (rightRef === undefined) {
          return;
        }
        const left = leftRef.trim();
        const right = rightRef.trim();
        if (!left || !right) {
          void vscode.window.showErrorMessage(
            "OntoCode: semantic diff requires non-empty left and right refs"
          );
          return;
        }
        await SemanticDiffPanel.show(context.extensionUri, {
          leftRef: left,
          rightRef: right,
        });
      } catch (err) {
        const message = err instanceof Error ? err.message : String(err);
        void vscode.window.showErrorMessage(`OntoCode: semantic diff failed — ${message}`);
      }
    }),
    vscode.commands.registerCommand(
      "ontocode.showExplanation",
      async (classIri?: string) => {
        if (!classIri) {
          classIri = await vscode.window.showInputBox({
            prompt: "Unsatisfiable class IRI",
          });
        }
        if (!classIri) {
          return;
        }
        try {
          await ExplanationPanel.show(classIri);
        } catch (err) {
          const message = err instanceof Error ? err.message : String(err);
          void vscode.window.showErrorMessage(
            `OntoCode: explanation failed — ${message}`
          );
        }
      }
    ),
    vscode.commands.registerCommand("ontocode.setHierarchyMode", async () => {
      const pick = await vscode.window.showQuickPick(
        [
          { label: "Asserted hierarchy", value: "asserted" },
          { label: "Inferred hierarchy", value: "inferred" },
          { label: "Combined hierarchy", value: "combined" },
        ],
        { placeHolder: "Class hierarchy display mode" }
      );
      if (!pick) {
        return;
      }
      await vscode.workspace
        .getConfiguration("ontocode")
        .update(
          "hierarchy.mode",
          pick.value,
          vscode.ConfigurationTarget.Workspace
        );
      await refreshExplorer(providers);
    }),
    vscode.commands.registerCommand("ontocode.openClassGraph", async () => {
      await GraphPanel.show(context.extensionUri, { graphKind: "class" }, "Class Graph");
    }),
    vscode.commands.registerCommand("ontocode.openPropertyGraph", async () => {
      await GraphPanel.show(context.extensionUri, { graphKind: "property" }, "Property Graph");
    }),
    vscode.commands.registerCommand("ontocode.openImportGraph", async () => {
      await GraphPanel.show(context.extensionUri, { graphKind: "import" }, "Import Graph");
    }),
    vscode.commands.registerCommand(
      "ontocode.openNeighborhoodGraph",
      async (arg?: unknown) => {
        const iri = resolveEntityIri(arg);
        await GraphPanel.show(
          context.extensionUri,
          { graphKind: "neighborhood", rootIri: iri },
          iri ? `Neighborhood: ${iri}` : "Neighborhood Graph"
        );
      }
    ),
    vscode.commands.registerCommand(
      "ontocode.openGraph",
      async (arg?: unknown) => {
        const iri = resolveEntityIri(arg);
        await GraphPanel.show(
          context.extensionUri,
          { graphKind: "neighborhood", rootIri: iri },
          "Ontology Graph"
        );
      }
    ),
    vscode.commands.registerCommand("ontocode.openSmokePanel", async () => {
      if (context.extensionMode !== vscode.ExtensionMode.Development) {
        void vscode.window.showWarningMessage(
          "OntoCode smoke panel is only available in development mode."
        );
        return;
      }
      const { PanelHost } = await import("../webviews/panelHost");
      PanelHost.create(context.extensionUri, {
        viewType: "ontocodeSmoke",
        title: "OntoCode React Smoke",
        panel: "smoke",
      });
    })
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

let inspectorRequestSeq = 0;

async function openInspector(
  extensionUri: vscode.Uri,
  iri: string,
  onRefresh?: () => Promise<void>
): Promise<void> {
  const requestId = ++inspectorRequestSeq;
  const snapshot = await getCatalogSnapshot();
  if (requestId !== inspectorRequestSeq) {
    return;
  }
  const classOptions = snapshot.entities
    .filter((e) => e.kind === "class")
    .map((e) => e.iri);
  const { detail } = await getEntity(iri);
  if (requestId !== inspectorRequestSeq) {
    return;
  }
  EntityInspectorPanel.show(extensionUri, detail, classOptions, onRefresh, requestId);
}

async function createEntity(
  context: vscode.ExtensionContext,
  providers: Parameters<typeof refreshExplorer>[0],
  kind: PatchEntityKind
): Promise<void> {
  const localName = await vscode.window.showInputBox({
    prompt: "Local name (e.g. Employee)",
  });
  if (!localName?.trim()) {
    return;
  }
  const snapshot = await getCatalogSnapshot();
  let ttlDocs = snapshot.documents.filter((d) => d.format === "turtle");
  const activeEditor = vscode.window.activeTextEditor;
  if (activeEditor) {
    const folder = vscode.workspace.getWorkspaceFolder(activeEditor.document.uri);
    if (folder) {
      const prefix = folder.uri.fsPath;
      const inFolder = ttlDocs.filter((d) => d.path.startsWith(prefix));
      if (inFolder.length > 0) {
        ttlDocs = inFolder;
      }
    }
  }
  if (ttlDocs.length === 0) {
    void vscode.window.showErrorMessage("No Turtle (.ttl) ontology in workspace");
    return;
  }
  const docPick =
    ttlDocs.length === 1
      ? ttlDocs[0]
      : await vscode.window
          .showQuickPick(
            ttlDocs.map((d) => ({ label: d.path, doc: d })),
            { placeHolder: "Target ontology file" }
          )
          .then((p) => p?.doc);
  if (!docPick) {
    return;
  }
  const base =
    docPick.base_iri ??
    Object.values(docPick.namespaces ?? {}).find(
      (ns) => ns.endsWith("#") || ns.endsWith("/")
    ) ??
    "http://example.org/ontology#";
  const entity_iri = `${base}${localName.trim()}`;
  const patches: PatchOp[] = [
    { op: "create_entity", entity_iri, kind },
    { op: "add_label", entity_iri, value: localName.trim() },
  ];
  try {
    await applyAxiomPatch({
      document_uri: vscode.Uri.file(docPick.path).toString(),
      patches,
      preview_only: false,
    });
    await refreshExplorer(providers);
    await openInspector(
      context.extensionUri,
      entity_iri,
      async () => refreshExplorer(providers)
    );
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    void vscode.window.showErrorMessage(`Create failed: ${message}`);
  }
}
