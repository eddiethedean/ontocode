import * as path from "path";
import * as vscode from "vscode";
import {
  applyAxiomPatch,
  cancelActiveReasonerRequest,
  createOntology,
  exportOntology,
  getCatalogSnapshot,
  indexWorkspace,
  listPlugins,
  previewRefactor,
  runReasoner,
  setActiveOntology,
} from "../lsp/client";
import { requirePatchFullySynced } from "../lsp/patchFeedback";
import type {
  OntologyDocument,
  PatchOp,
  RefactorRequest,
  RunReasonerResult,
} from "../lsp/protocol";
import { focusRelay } from "../focus/focusRelay";
import { CommandRegistry } from "./registry";
import { getFocusedEntityIri } from "./uiState";
import { appendError, openErrorLog } from "../logging/errorLog";
import { normalizeFsPath, pathsEqual } from "../utils/pathUnder";
import { documentUriInWorkspace } from "../utils/workspacePath";
import {
  listPerspectives,
  persistPerspective,
  type Perspective,
} from "../webviews/layoutPersistence";
import { RefactorPreviewPanel } from "../webviews/refactorPreview";
import { ReasonerPanel } from "../webviews/reasonerPanel";
import {
  captureReasoningPreRun,
  reasoningStateForRunCancel,
  reasoningStateForRunStart,
  reasoningStateForRunSuccess,
} from "../webviews/reasonerPanelLogic";
import { PanelHost } from "../webviews/panelHost";

const ACTIVE_ONTOLOGY_KEY = "ontocode.activeOntology";
const ONTOLOGY_FILTERS: Record<string, string[]> = {
  "Ontology files": ["ttl", "owl", "rdf", "jsonld", "json-ld", "nt", "nq", "trig", "obo"],
};

/** Module context for test hooks that open dialogs without native pickers. */
let dialogRuntime:
  | { extensionUri: vscode.Uri; refresh?: () => Promise<void> }
  | undefined;

export function registerV017Commands(
  context: vscode.ExtensionContext,
  refresh?: () => Promise<void>
): void {
  dialogRuntime = { extensionUri: context.extensionUri, refresh };
  const registry = new CommandRegistry(context);
  const command = (id: string, handler: (...args: never[]) => unknown): void => {
    registry.register(id, async (...args) => {
      try {
        return await handler(...(args as never[]));
      } catch (error) {
        appendError(error, id);
        const message = error instanceof Error ? error.message : String(error);
        void vscode.window.showErrorMessage(`OntoCode: ${message}`);
        return undefined;
      }
    });
  };

  command("ontocode.newOntology", async () => {
    const target = await vscode.window.showSaveDialog({
      title: "New Ontology",
      filters: { "Ontology files": ["ttl", "obo"] },
      defaultUri: defaultWorkspaceUri("ontology.ttl"),
    });
    if (!target) return;
    openNewOntologyPanel(context.extensionUri, target.fsPath, refresh);
  });
  command("ontocode.openOntology", async () => {
    const selected = await vscode.window.showOpenDialog({
      canSelectMany: false,
      filters: ONTOLOGY_FILTERS,
      openLabel: "Open Ontology",
    });
    if (selected?.[0]) {
      await vscode.window.showTextDocument(
        await vscode.workspace.openTextDocument(selected[0])
      );
    }
  });
  command("ontocode.openRecent", () =>
    vscode.commands.executeCommand("workbench.action.openRecent")
  );
  command("ontocode.save", () =>
    vscode.commands.executeCommand("workbench.action.files.save")
  );
  command("ontocode.saveAll", () =>
    vscode.commands.executeCommand("workbench.action.files.saveAll")
  );
  command("ontocode.undo", () =>
    vscode.commands.executeCommand("undo")
  );
  command("ontocode.redo", () =>
    vscode.commands.executeCommand("redo")
  );
  command("ontocode.closeProject", () =>
    vscode.commands.executeCommand("workbench.action.closeFolder")
  );
  command("ontocode.saveAs", () => runExport(true));
  command("ontocode.exportOntology", () => runExport(false));

  command("ontocode.searchEntities", async () => {
    const snapshot = await getCatalogSnapshot();
    const picked = await vscode.window.showQuickPick(
      snapshot.entities.map((entity) => ({
        label: entity.labels[0] || entity.short_name || entity.iri,
        description: entity.kind.replace(/_/g, " "),
        detail: entity.iri,
        iri: entity.iri,
      })),
      {
        title: "Search Ontology Entities",
        matchOnDescription: true,
        matchOnDetail: true,
      }
    );
    if (picked) {
      focusRelay.setEntityFocus(picked.iri, "search");
      await vscode.commands.executeCommand("ontocode.openEntity", picked.iri);
    }
  });

  command("ontocode.openPreferences", async () => {
    const choice = await vscode.window.showQuickPick(
      [
        {
          label: "General OntoCode Settings",
          description: "Index, hierarchy, diagnostics, LSP",
          value: "settings",
        },
        {
          label: "Reasoner Settings",
          description: "Default profile and auto-detect",
          value: "reasoner",
        },
        {
          label: "Query Settings",
          description: "History limit and workbench defaults",
          value: "query",
        },
        {
          label: "Plugin Preferences",
          description: "Pages contributed by workspace plugins",
          value: "plugins",
        },
        {
          label: "Keyboard Shortcuts",
          description: "OntoCode keybindings",
          value: "keys",
        },
      ],
      { title: "OntoCode Preferences", matchOnDescription: true }
    );
    if (!choice) return;
    if (choice.value === "plugins") {
      await vscode.commands.executeCommand("ontocode.plugins.openPreferences");
    } else if (choice.value === "keys") {
      await vscode.commands.executeCommand(
        "workbench.action.openGlobalKeybindings",
        "ontocode"
      );
    } else if (choice.value === "reasoner") {
      await vscode.commands.executeCommand(
        "workbench.action.openSettings",
        "@ext:ontocode.ontocode ontocode.reasoner"
      );
    } else if (choice.value === "query") {
      await vscode.commands.executeCommand(
        "workbench.action.openSettings",
        "@ext:ontocode.ontocode ontocode.query"
      );
    } else {
      await vscode.commands.executeCommand(
        "workbench.action.openSettings",
        "@ext:ontocode.ontocode"
      );
    }
  });

  command("ontocode.copyEntityIri", () => copyFocused(false));
  command("ontocode.copyEntityShortForm", () => copyFocused(true));

  command("ontocode.setActiveOntology", async () => {
    const document = await pickOntologyDocument("Set Active Ontology");
    if (!document) return;
    const result = await setActiveOntology({ ontology_id: document.id });
    await context.workspaceState.update(
      ACTIVE_ONTOLOGY_KEY,
      result.active_ontology_id
    );
    void vscode.window.showInformationMessage(
      `OntoCode: active ontology set to ${document.base_iri ?? path.basename(document.path)}`
    );
  });

  command("ontocode.editOntologyMetadata", async () => {
    const document = await pickOntologyDocument("Edit Ontology Metadata");
    if (!document) return;
    const ontologyIri = await requiredInput(
      "Ontology IRI",
      document.base_iri ?? "https://example.org/ontology"
    );
    if (!ontologyIri) return;
    const predicate = await vscode.window.showInputBox({
      prompt: "Annotation predicate IRI (optional)",
      placeHolder: "http://www.w3.org/2000/01/rdf-schema#label",
    });
    if (predicate === undefined) return;
    const patches: PatchOp[] = [
      { op: "set_ontology_iri", ontology_iri: ontologyIri },
    ];
    if (predicate.trim()) {
      const value = await requiredInput("Annotation value");
      if (!value) return;
      patches.push({
        op: "add_ontology_annotation",
        ontology_iri: ontologyIri,
        predicate: predicate.trim(),
        value,
      });
    }
    await applyDocumentPatches(document, patches);
    await refresh?.();
  });

  command("ontocode.managePrefixes", async () => {
    const document = await pickOntologyDocument("Manage Prefixes");
    if (!document) return;
    openPrefixManagerPanel(context.extensionUri, document, refresh);
  });
  command("ontocode.showMetrics", async () => {
    const snapshot = await getCatalogSnapshot();
    const host = PanelHost.create(context.extensionUri, {
      viewType: "ontocode.metrics",
      title: "Ontology Metrics",
      panel: "metrics",
    });
    host.postMessage({
      type: "loadMetrics",
      stats: snapshot.stats ?? {
        ontology_count: snapshot.documents.length,
        class_count: snapshot.entities.filter((entity) => entity.kind === "class").length,
        object_property_count: 0,
        data_property_count: 0,
        annotation_property_count: 0,
        individual_count: snapshot.entities.filter((entity) => entity.kind === "individual").length,
        axiom_count: 0,
        annotation_count: 0,
        triple_count: 0,
        error_count: 0,
        diagnostic_error_count: snapshot.diagnostics.length,
        diagnostic_warning_count: 0,
        diagnostic_info_count: 0,
      },
    });
  });

  command("ontocode.mergeEntities", () =>
    runEntityRefactor(context, "merge_entities", refresh)
  );
  command("ontocode.replaceEntity", () =>
    runEntityRefactor(context, "replace_entity", refresh)
  );

  command("ontocode.startReasoner", () => runReasoning(context, "start"));
  command("ontocode.synchronizeReasoner", () => runReasoning(context, "synchronize"));
  command("ontocode.classifyOntology", () => runReasoning(context, "classify"));
  command("ontocode.checkConsistency", async () => {
    const result = await runReasoning(context, "consistency");
    if (result) {
      void vscode.window.showInformationMessage(
        result.consistent
          ? "OntoCode: ontology is consistent"
          : `OntoCode: ontology is inconsistent (${result.unsatisfiable.length} unsatisfiable classes)`
      );
    }
  });
  command("ontocode.stopReasoner", () => {
    cancelActiveReasonerRequest();
    ReasonerPanel.current?.cancelActiveRun();
    void vscode.window.showInformationMessage(
      "OntoCode: reasoner cancelled; late server results will be ignored"
    );
  });
  command("ontocode.configureReasoner", () =>
    vscode.commands.executeCommand(
      "workbench.action.openSettings",
      "@ext:ontocode.ontocode ontocode.reasoner"
    )
  );

  command("ontocode.validateWorkspace", async () => {
    const result = await indexWorkspace();
    await refresh?.();
    void vscode.window.showInformationMessage(
      `OntoCode validation: ${result.stats.diagnostic_error_count} errors, ${result.stats.diagnostic_warning_count} warnings, ${result.stats.diagnostic_info_count} info`
    );
  });
  command("ontocode.runBatchTools", () =>
    vscode.window.withProgress(
      {
        location: vscode.ProgressLocation.Notification,
        title: "OntoCode: validating and collecting metrics",
      },
      async (progress) => {
        progress.report({ message: "Validating workspace…" });
        const result = await indexWorkspace();
        progress.report({ message: "Collecting metrics…" });
        const snapshot = await getCatalogSnapshot();
        await refresh?.();
        void vscode.window.showInformationMessage(
          `OntoCode batch complete: ${result.stats.diagnostic_error_count} errors, ${snapshot.stats?.axiom_count ?? 0} axioms`
        );
      }
    )
  );

  command("ontocode.switchPerspective", async () => {
    const picked = await vscode.window.showQuickPick(
      listPerspectives(context).map((perspective) => ({
        label: perspective.name,
        description: perspective.panels.join(", "),
        perspective,
      })),
      { title: "Switch OntoCode Perspective" }
    );
    if (picked) await openPerspective(context, picked.perspective);
  });
  command("ontocode.savePerspective", async () => {
    const name = await requiredInput("Perspective name");
    if (!name) return;
    const panels = await vscode.window.showQuickPick(
      PANEL_CHOICES,
      { canPickMany: true, title: "Panels in Perspective" }
    );
    if (!panels) return;
    await persistPerspective(context, {
      name,
      panels: panels.map((item) => item.value),
    });
  });
  for (const panel of PANEL_CHOICES) {
    command(`ontocode.show${panel.commandSuffix}`, () =>
      openPanel(context, panel.value)
    );
  }

  command("ontocode.showAbout", () => {
    PanelHost.create(context.extensionUri, {
      viewType: "ontocode.about",
      title: "About OntoCode",
      panel: "about",
    });
  });
  command("ontocode.showPluginInfo", async () => {
    const plugins = await listPlugins();
    const details =
      plugins.plugins.map((plugin) => `${plugin.name} ${plugin.version}`).join(", ") ||
      "No plugins loaded";
    void vscode.window.showInformationMessage(`OntoCode plugins: ${details}`);
  });
  command("ontocode.openErrorLog", () => openErrorLog());
  command("ontocode.exportDiagnostics", async () => {
    const snapshot = await getCatalogSnapshot();
    const target = await vscode.window.showSaveDialog({
      defaultUri: defaultWorkspaceUri("ontocode-diagnostics.json"),
      filters: { JSON: ["json"] },
    });
    if (target) {
      await vscode.workspace.fs.writeFile(
        target,
        Buffer.from(JSON.stringify(snapshot.diagnostics, null, 2), "utf8")
      );
    }
  });
  command("ontocode.openDocumentation", () =>
    vscode.env.openExternal(
      vscode.Uri.parse("https://ontocode-vs.readthedocs.io/en/latest/")
    )
  );
  command("ontocode.openSupport", () =>
    vscode.env.openExternal(
      vscode.Uri.parse("https://github.com/eddiethedean/ontocode/issues")
    )
  );
  command("ontocode.openKeyboardShortcuts", () =>
    vscode.commands.executeCommand(
      "workbench.action.openGlobalKeybindings",
      "ontocode"
    )
  );

  registry.startContextSync();
}

/**
 * Open New Ontology dialog for a concrete path (skips showSaveDialog).
 * Used by VS Code e2e hooks when ONTOCODE_TEST_FIXTURES is set.
 */
export function openNewOntologyDialog(targetPath: string): void {
  if (!dialogRuntime) {
    throw new Error("OntoCode dialog commands are not registered");
  }
  openNewOntologyPanel(
    dialogRuntime.extensionUri,
    targetPath,
    dialogRuntime.refresh
  );
}

/**
 * Open Prefix Manager for a catalog document path (skips multi-doc quick pick).
 * Used by VS Code e2e hooks when ONTOCODE_TEST_FIXTURES is set.
 */
export async function openPrefixManager(documentPath: string): Promise<void> {
  if (!dialogRuntime) {
    throw new Error("OntoCode dialog commands are not registered");
  }
  const snapshot = await getCatalogSnapshot();
  const document = snapshot.documents.find((doc) =>
    pathsEqual(doc.path, documentPath)
  );
  if (!document) {
    throw new Error(`No indexed ontology document at ${documentPath}`);
  }
  openPrefixManagerPanel(
    dialogRuntime.extensionUri,
    document,
    dialogRuntime.refresh
  );
}

function openNewOntologyPanel(
  extensionUri: vscode.Uri,
  targetPath: string,
  refresh?: () => Promise<void>
): PanelHost {
  const host = PanelHost.create(extensionUri, {
    viewType: "ontocode.newOntology",
    title: "New Ontology",
    panel: "newOntology",
    onMessage: async (message, panel) => {
      if (message.type !== "submitNewOntology") return;
      const result = await createOntology({
        path: targetPath,
        ontology_iri: message.ontologyIri,
        version_iri: message.versionIri,
        format: formatForPath(targetPath),
      });
      await refresh?.();
      panel.dispose();
      await vscode.window.showTextDocument(
        await vscode.workspace.openTextDocument(result.path)
      );
    },
  });
  host.postMessage({
    type: "loadNewOntology",
    path: targetPath,
    defaultIri: "https://example.org/ontology",
  });
  return host;
}

function openPrefixManagerPanel(
  extensionUri: vscode.Uri,
  document: OntologyDocument,
  refresh?: () => Promise<void>
): PanelHost {
  const host = PanelHost.create(extensionUri, {
    viewType: "ontocode.prefixManager",
    title: "Prefix Manager",
    panel: "prefixManager",
    onMessage: async (message, panel) => {
      if (message.type !== "submitPrefix") return;
      const namespaces = document.namespaces ?? {};
      const patch: PatchOp =
        message.action === "remove"
          ? { op: "remove_prefix", prefix: message.prefix }
          : Object.prototype.hasOwnProperty.call(namespaces, message.prefix)
            ? {
                op: "set_prefix",
                prefix: message.prefix,
                namespace_iri: message.namespaceIri ?? "",
              }
            : {
                op: "add_prefix",
                prefix: message.prefix,
                namespace_iri: message.namespaceIri ?? "",
              };
      await applyDocumentPatches(document, [patch]);
      await refresh?.();
      panel.dispose();
    },
  });
  host.postMessage({
    type: "loadPrefixes",
    path: document.path,
    prefixes: document.namespaces ?? {},
  });
  return host;
}

async function runExport(saveAs: boolean): Promise<void> {
  const document = await pickOntologyDocument(
    saveAs ? "Save Ontology As" : "Export Ontology"
  );
  if (!document) return;
  const target = await vscode.window.showSaveDialog({
    title: saveAs ? "Save Ontology As" : "Export Ontology",
    defaultUri: vscode.Uri.file(normalizeFsPath(document.path)),
    filters: ONTOLOGY_FILTERS,
  });
  if (!target) return;
  const result = await exportOntology({
    source_path: document.path,
    output_path: target.fsPath,
    format: formatForPath(target.fsPath),
  });
  if (result.success) {
    void vscode.window.showInformationMessage(
      `OntoCode: exported ${path.basename(result.output_path)}`
    );
  } else {
    const detail = result.logs?.trim();
    void vscode.window.showErrorMessage(
      detail
        ? `OntoCode: export failed — ${detail.slice(0, 300)}`
        : `OntoCode: export failed for ${path.basename(result.output_path)}`
    );
  }
}

async function copyFocused(shortForm: boolean): Promise<void> {
  const iri = getFocusedEntityIri();
  if (!iri) {
    void vscode.window.showWarningMessage("OntoCode: no entity is selected");
    return;
  }
  const value = shortForm ? iri.slice(Math.max(iri.lastIndexOf("#"), iri.lastIndexOf("/")) + 1) : iri;
  await vscode.env.clipboard.writeText(value);
}

async function pickOntologyDocument(title: string): Promise<OntologyDocument | undefined> {
  const snapshot = await getCatalogSnapshot();
  const active = snapshot.active_ontology_id;
  if (snapshot.documents.length === 1) return snapshot.documents[0];
  const picked = await vscode.window.showQuickPick(
    snapshot.documents.map((document) => ({
      label: document.base_iri ?? path.basename(document.path),
      description: document.format,
      detail: document.path,
      document,
    })),
    { title, placeHolder: active ? `Active: ${active}` : undefined }
  );
  return picked?.document;
}

async function applyDocumentPatches(
  document: OntologyDocument,
  patches: PatchOp[]
): Promise<void> {
  const documentUri = documentUriInWorkspace(document.path);
  if (!documentUri) {
    throw new Error(`document path is outside the workspace: ${document.path}`);
  }
  const result = await applyAxiomPatch({
    document_uri: documentUri,
    patches,
    preview_only: false,
  });
  requirePatchFullySynced(result);
}

async function runEntityRefactor(
  context: vscode.ExtensionContext,
  kind: "merge_entities" | "replace_entity",
  refresh?: () => Promise<void>
): Promise<void> {
  const focused = getFocusedEntityIri();
  const from = focused ?? (await requiredInput(kind === "merge_entities" ? "Entity to merge" : "Entity to replace"));
  if (!from) return;
  const to = await requiredInput(
    kind === "merge_entities" ? "Entity to keep" : "Replacement entity IRI"
  );
  if (!to || to === from) return;
  const request: RefactorRequest =
    kind === "merge_entities"
      ? { kind, keep_iri: to, merge_iri: from }
      : { kind, from_iri: from, to_iri: to };
  const plan = await previewRefactor(request);
  await RefactorPreviewPanel.show(context.extensionUri, plan, request, refresh);
}

async function runReasoning(
  context: vscode.ExtensionContext,
  action: string
): Promise<RunReasonerResult | undefined> {
  const config = vscode.workspace.getConfiguration("ontocode");
  const profile = config.get<string>("reasoner.default", "el");
  const autoDetect = config.get<boolean>("reasoner.autoProfile", true);
  const titles: Record<string, string> = {
    start: "OntoCode: Starting reasoner",
    synchronize: "OntoCode: Synchronizing reasoner",
    classify: "OntoCode: Classifying ontology",
    consistency: "OntoCode: Checking consistency",
  };
  const title = titles[action] ?? "OntoCode: Running reasoner";

  if (action === "start") {
    ReasonerPanel.show(context.extensionUri);
  }

  return vscode.window.withProgress(
    {
      location: vscode.ProgressLocation.Notification,
      title,
      cancellable: true,
    },
    async (progress, token) => {
      const preRun = captureReasoningPreRun(focusRelay.getReasoning());
      focusRelay.setReasoningState(reasoningStateForRunStart(profile, preRun));
      try {
        if (action === "synchronize") {
          progress.report({ message: "Reindexing workspace…" });
          await indexWorkspace();
          if (token.isCancellationRequested) {
            cancelActiveReasonerRequest();
            focusRelay.setReasoningState(reasoningStateForRunCancel(preRun));
            return undefined;
          }
        }
        progress.report({
          message:
            action === "consistency" ? "Checking consistency…" : "Classifying…",
        });
        const result = await runReasoner(
          { profile, auto_detect: autoDetect },
          token
        );
        if (token.isCancellationRequested) {
          ReasonerPanel.current?.cancelActiveRun();
          focusRelay.setReasoningState(reasoningStateForRunCancel(preRun));
          return undefined;
        }
        focusRelay.setReasoningState(
          reasoningStateForRunSuccess(
            result.profile_used,
            result.unsatisfiable,
            preRun
          )
        );
        if (action === "start" || action === "classify" || action === "synchronize") {
          const panel = ReasonerPanel.show(context.extensionUri);
          panel.presentResult(result);
        }
        void vscode.commands.executeCommand("ontocode.refreshExplorer");
        return result;
      } catch (error) {
        if (token.isCancellationRequested) {
          focusRelay.setReasoningState(reasoningStateForRunCancel(preRun));
          return undefined;
        }
        focusRelay.setReasoningRunning(false);
        throw error;
      }
    }
  );
}

const PANEL_CHOICES = [
  { label: "Entity Inspector", value: "inspector", commandSuffix: "InspectorPanel" },
  { label: "Query Workbench", value: "query", commandSuffix: "QueryPanel" },
  { label: "Reasoner", value: "reasoner", commandSuffix: "ReasonerPanel" },
  { label: "Explanation", value: "explanation", commandSuffix: "ExplanationPanel" },
  { label: "Graph", value: "graph", commandSuffix: "GraphPanel" },
  { label: "Semantic Diff", value: "semanticDiff", commandSuffix: "SemanticDiffPanel" },
  { label: "Imports", value: "imports", commandSuffix: "ImportsPanel" },
] as const;

async function openPerspective(
  context: vscode.ExtensionContext,
  perspective: Perspective
): Promise<void> {
  for (const panel of perspective.panels) await openPanel(context, panel);
}

async function openPanel(
  context: vscode.ExtensionContext,
  panel: string
): Promise<void> {
  if (panel === "reasoner") {
    ReasonerPanel.show(context.extensionUri);
    return;
  }
  const commandByPanel: Record<string, string> = {
    inspector: "ontocode.showEntityInspector",
    query: "ontocode.openQueryWorkbench",
    explanation: "ontocode.showExplanation",
    graph: "ontocode.openClassGraph",
    semanticDiff: "ontocode.semanticDiff",
    imports: "ontocode.manageImports",
  };
  const command = commandByPanel[panel];
  if (command) await vscode.commands.executeCommand(command);
  else void vscode.window.showWarningMessage(`OntoCode: unknown panel "${panel}"`);
}

async function requiredInput(
  prompt: string,
  value?: string
): Promise<string | undefined> {
  return vscode.window.showInputBox({
    prompt,
    value,
    ignoreFocusOut: true,
    validateInput: (input) => (input.trim() ? undefined : `${prompt} is required`),
  }).then((input) => input?.trim() || undefined);
}

function defaultWorkspaceUri(fileName: string): vscode.Uri | undefined {
  const folder = vscode.workspace.workspaceFolders?.[0];
  return folder ? vscode.Uri.joinPath(folder.uri, fileName) : undefined;
}

function formatForPath(filePath: string): string | undefined {
  const extension = path.extname(filePath).slice(1).toLowerCase();
  const formats: Record<string, string> = {
    ttl: "turtle",
    owl: "rdfxml",
    rdf: "rdfxml",
    jsonld: "jsonld",
    "json-ld": "jsonld",
    nt: "ntriples",
    nq: "nquads",
    trig: "trig",
    obo: "obo",
  };
  return formats[extension];
}
