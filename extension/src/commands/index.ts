import * as vscode from "vscode";
import {
  applyAxiomPatch,
  getCatalogSnapshot,
  getEntity,
  indexWorkspace,
  listPlugins,
} from "../lsp/client";
import { isPatchFullySynced, patchFailureMessage } from "../lsp/patchFeedback";
import { PatchEntityKind, PatchOp } from "../lsp/protocol";
import { EntityInspectorPanel } from "../webviews/inspector";
import { focusRelay } from "../focus/focusRelay";
import { GraphPanel } from "../webviews/graphPanel";
import { QueryWorkbenchPanel } from "../webviews/queryWorkbenchReact";
import {
  ManchesterEditorPanel,
  ManchesterEditorOptions,
} from "../webviews/manchesterEditorReact";
import { ReasonerPanel } from "../webviews/reasonerPanel";
import { ExplanationPanel } from "../webviews/explanationPanel";
import { SemanticDiffPanel } from "../webviews/semanticDiffPanel";
import { ImportsPanel } from "../webviews/importsPanel";
import {
  extractModule,
  migrateNamespace,
  moveEntity,
  RefactorPreviewPanel,
  renameEntityIri,
  showEntityUsages,
} from "../webviews/refactorPreview";
import { ExplorerTreeProvider, OntologyTreeItem } from "../treeviews/explorer";
import { resolveEntityIri } from "../utils/resolveEntityIri";
import { byteColToUtf16 } from "../utils/positions";
import { documentUriInWorkspace, isPathUnderFolder, openWorkspaceTextDocument } from "../utils/workspacePath";
import { refreshPluginCommands } from "./pluginCommands";
import { WorkflowPanel } from "../webviews/workflowPanel";
import { PluginViewPanel } from "../webviews/pluginViewPanel";
import { registerV017Commands } from "./v017Commands";

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
    vscode.commands.registerCommand("ontocode.runOwlmakeWorkflow", async () => {
      await WorkflowPanel.runOwlmake("qc");
    }),
    vscode.commands.registerCommand("ontocode.indexWorkspace", async () => {
      await runIndexAndRefresh(context, providers);
      vscode.window.showInformationMessage("OntoCode: workspace indexed");
    }),
    vscode.commands.registerCommand("ontocode.refreshExplorer", async () => {
      await refreshExplorer(providers);
    }),
    vscode.commands.registerCommand("ontocode.plugins.runCommand", async () => {
      const plugins = await listPlugins().then((r) => r.plugins).catch(() => []);
      const items: Array<{
        label: string;
        description: string;
        plugin: import("../lsp/protocol").PluginDescriptor;
        cmd: import("../lsp/protocol").PluginCommandContribution;
      }> = [];
      for (const plugin of plugins) {
        for (const cmd of plugin.ui.commands ?? []) {
          items.push({
            label: cmd.title,
            description: plugin.name,
            plugin,
            cmd,
          });
        }
      }
      const picked = await vscode.window.showQuickPick(items, {
        title: "OntoCode Plugin Commands",
        matchOnDescription: true,
      });
      if (!picked) {
        return;
      }
      const commandId = `ontocode.plugin.${picked.cmd.id}`;
      await vscode.commands.executeCommand(commandId);
    }),
    vscode.commands.registerCommand("ontocode.plugins.openView", async () => {
      const plugins = await listPlugins().then((r) => r.plugins).catch(() => []);
      const items: Array<{
        label: string;
        description: string;
        plugin: import("../lsp/protocol").PluginDescriptor;
        view: import("../lsp/protocol").PluginViewContribution;
      }> = [];
      for (const plugin of plugins) {
        for (const view of plugin.ui.views ?? []) {
          items.push({
            label: view.title,
            description: plugin.name,
            plugin,
            view,
          });
        }
      }
      const picked = await vscode.window.showQuickPick(items, {
        title: "OntoCode Plugin Views",
        matchOnDescription: true,
      });
      if (!picked) {
        return;
      }
      await PluginViewPanel.open(context.extensionUri, picked.plugin, picked.view);
    }),
    vscode.commands.registerCommand(
      "ontocode.plugins.openPreferences",
      async () => {
        const plugins = await listPlugins().then((r) => r.plugins).catch(() => []);
        const items: Array<{
          label: string;
          description: string;
          plugin: import("../lsp/protocol").PluginDescriptor;
          page: import("../lsp/protocol").PluginPreferencePageContribution;
        }> = [];

        for (const plugin of plugins) {
          for (const page of plugin.ui.preferences_pages ?? []) {
            items.push({
              label: page.title,
              description: `${plugin.name}${page.category ? ` · ${page.category}` : ""}`,
              plugin,
              page,
            });
          }
        }

        const picked = await vscode.window.showQuickPick(items, {
          title: "OntoCode Plugin Preferences",
          matchOnDescription: true,
        });
        if (!picked) {
          return;
        }

        // Preferences pages are hosted as a plugin view (ui_view) for now.
        await PluginViewPanel.open(context.extensionUri, picked.plugin, {
          id: picked.page.id,
          title: picked.page.title,
          kind: "preferences",
        });
      }
    ),
    vscode.commands.registerCommand(
      "ontocode.plugins.runContextAction",
      async () => {
        const focus = focusRelay.getFocus();
        if (!focus || focus.kind !== "entity") {
          void vscode.window.showWarningMessage(
            "OntoCode: no focused entity. Open an entity in the inspector first."
          );
          return;
        }

        const plugins = await listPlugins().then((r) => r.plugins).catch(() => []);
        const items: Array<{
          label: string;
          description: string;
          plugin: import("../lsp/protocol").PluginDescriptor;
          action: import("../lsp/protocol").PluginContextActionContribution;
        }> = [];

        for (const plugin of plugins) {
          for (const action of plugin.ui.context_actions ?? []) {
            if (action.scope && action.scope !== "entity") {
              continue;
            }
            items.push({
              label: action.title,
              description: plugin.name,
              plugin,
              action,
            });
          }
        }

        const picked = await vscode.window.showQuickPick(items, {
          title: "OntoCode Plugin Context Actions",
          matchOnDescription: true,
        });
        if (!picked) {
          return;
        }

        // For now, context actions execute the referenced plugin command contribution.
        const commandId = `ontocode.plugin.${picked.action.command}`;
        await vscode.commands.executeCommand(commandId, {
          focus: { kind: focus.kind, id: focus.id, source: focus.source },
        });
      }
    ),
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
          if (line >= doc.lineCount) {
            void vscode.window.showWarningMessage(
              `OntoCode: diagnostic line ${diagnostic.line} is out of range`
            );
            return;
          }
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
          const doc = await openWorkspaceTextDocument(detail.source.path);
          if (!doc) {
            return;
          }
          const editor = await vscode.window.showTextDocument(doc);
          const line = Math.max(0, detail.source.line - 1);
          if (line >= doc.lineCount) {
            void vscode.window.showWarningMessage(
              `OntoCode: source line ${detail.source.line} is out of range for ${iri}`
            );
            return;
          }
          const lineText = doc.lineAt(line).text;
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
      let impactSummary = `Delete entity ${iri}?`;
      try {
        const { deleteImpact } = await import("../lsp/client");
        const impact = await deleteImpact({ entity_iri: iri });
        const refs = impact.referencing_entities.slice(0, 5).join(", ");
        impactSummary =
          `Delete entity ${iri}?\n\n` +
          `Usages: ${impact.usage_count}\n` +
          `Axioms: ${impact.axiom_count}\n` +
          (refs ? `Referencing: ${refs}\n` : "") +
          (impact.warnings.length ? `Warnings: ${impact.warnings.join("; ")}\n` : "");
      } catch {
        // Fall back to simple confirm when impact API is unavailable.
      }
      const confirm = await vscode.window.showWarningMessage(
        impactSummary,
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
        const documentUri = documentUriInWorkspace(detail.document_path);
        if (!documentUri) {
          void vscode.window.showErrorMessage(
            "OntoCode: entity document path is outside the workspace"
          );
          return;
        }
        const result = await applyAxiomPatch({
          document_uri: documentUri,
          patches: [{ op: "delete_entity", entity_iri: iri }],
          preview_only: false,
        });
        if (!result.applied) {
          void vscode.window.showErrorMessage(patchFailureMessage(result));
          return;
        }
        if (!isPatchFullySynced(result)) {
          return;
        }
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
      await vscode.window.withProgress(
        {
          location: vscode.ProgressLocation.Notification,
          title: "OntoCode: Running reasoner",
          cancellable: true,
        },
        async (_progress, token) => {
          const { cancelActiveReasonerRequest } = await import("../lsp/client");
          const run = panel.runWithDefaults();
          await Promise.race([
            run,
            new Promise<void>((resolve) => {
              token.onCancellationRequested(() => resolve());
            }),
          ]);
          if (token.isCancellationRequested) {
            cancelActiveReasonerRequest();
            panel.cancelActiveRun();
            void vscode.window.showWarningMessage(
              "OntoCode: reasoner run cancelled (late server results will be ignored)"
            );
          }
          // Ensure we don't leave an unhandled rejection if cancel won the race.
          void run.catch(() => undefined);
        }
      );
    }),
    vscode.commands.registerCommand("ontocode.semanticDiff", async () => {
      try {
        const leftRef = await vscode.window.showInputBox({
          prompt: "Left git ref (or INDEXED / CATALOG for indexed catalog)",
          value: "HEAD",
        });
        if (leftRef === undefined) {
          return;
        }
        const rightRef = await vscode.window.showInputBox({
          prompt: "Right git ref (WORKTREE, INDEXED, or CATALOG)",
          value: "WORKTREE",
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
      async (classIri?: string, profile?: string) => {
        if (!classIri) {
          classIri = await vscode.window.showInputBox({
            prompt: "Unsatisfiable class IRI",
          });
        }
        if (!classIri) {
          return;
        }
        try {
          await ExplanationPanel.show(classIri, profile);
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
      "ontocode.manageImports",
      async (item?: OntologyTreeItem) => {
        const filePath = item?.filePath;
        if (!filePath) {
          void vscode.window.showErrorMessage(
            "OntoCode: select a Turtle ontology in the Ontologies tree"
          );
          return;
        }
        try {
          await ImportsPanel.show(
            context.extensionUri,
            filePath,
            async () => refreshExplorer(providers)
          );
        } catch (err) {
          const message = err instanceof Error ? err.message : String(err);
          void vscode.window.showErrorMessage(
            `OntoCode: could not open imports panel — ${message}`
          );
        }
      }
    ),
    vscode.commands.registerCommand("ontocode.reloadImports", async () => {
      await runIndexAndRefresh(context, providers);
      if (ImportsPanel.current) {
        await ImportsPanel.current.refresh();
      }
      void vscode.window.showInformationMessage("OntoCode: imports reloaded");
    }),
    vscode.commands.registerCommand("ontocode.resetLayout", async () => {
      EntityInspectorPanel.currentPanel?.dispose();
      GraphPanel.currentPanel?.dispose();
      QueryWorkbenchPanel.current?.dispose();
      ImportsPanel.current?.dispose();
      ReasonerPanel.current?.dispose();
      ExplanationPanel.current?.dispose();
      SemanticDiffPanel.current?.dispose();
      RefactorPreviewPanel.current?.dispose();
      ManchesterEditorPanel.current?.dispose();
      void vscode.window.showInformationMessage("OntoCode: layout reset");
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
  registerV017Commands(context, () => refreshExplorer(providers));
}

async function runIndexAndRefresh(
  context: vscode.ExtensionContext,
  providers: {
  ontologies: ExplorerTreeProvider;
  classes: ExplorerTreeProvider;
  properties: ExplorerTreeProvider;
  individuals: ExplorerTreeProvider;
  diagnostics: ExplorerTreeProvider;
}): Promise<void> {
  await indexWorkspace();
  await refreshExplorer(providers);
  await refreshPluginCommands(context);
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
  focusRelay.setEntityFocus(iri, "explorer");
  const snapshot = await getCatalogSnapshot();
  if (requestId !== inspectorRequestSeq) {
    return;
  }
  const classOptions = snapshot.entities
    .filter((e) => e.kind === "class")
    .map((e) => e.iri);
  const objectPropertyOptions = snapshot.entities
    .filter((e) => e.kind === "object_property")
    .map((e) => e.iri);
  const { detail } = await getEntity(iri);
  if (requestId !== inspectorRequestSeq) {
    return;
  }
  EntityInspectorPanel.show(
    extensionUri,
    detail,
    classOptions,
    objectPropertyOptions,
    onRefresh,
    requestId
  );
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
      const inFolder = ttlDocs.filter((d) => isPathUnderFolder(d.path, prefix));
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
    const result = await applyAxiomPatch({
      document_uri: vscode.Uri.file(docPick.path).toString(),
      patches,
      preview_only: false,
    });
    if (!result.applied) {
      void vscode.window.showErrorMessage(patchFailureMessage(result));
      return;
    }
    if (!isPatchFullySynced(result)) {
      return;
    }
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
