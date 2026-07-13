import * as fs from "fs";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";
import {
  assertCatalogSnapshot,
  assertGetEntityResult,
  assertIndexWorkspaceResult,
  assertApplyPatchResult,
  assertTabularQueryResult,
  assertParseManchesterResult,
  assertRunReasonerResult,
  assertGetExplanationResult,
  assertGetGraphResult,
  assertRunRobotResult,
  assertFindUsagesResult,
  assertPreviewRefactorResult,
  assertApplyRefactorResult,
  assertSemanticDiffResult,
  assertListPluginsResult,
  assertRunPluginResult,
} from "./protocolGuards";
import { patchSyncCancelledMessage } from "./patchFeedback";
import {
  ApplyAxiomPatchClientResult,
  ApplyAxiomPatchParams,
  ApplyPatchResult,
  CatalogSnapshot,
  GetEntityResult,
  GetExplanationParams,
  GetExplanationResult,
  GetGraphParams,
  GetGraphResult,
  IndexWorkspaceResult,
  ParseManchesterParams,
  ParseManchesterResult,
  RunReasonerParams,
  RunReasonerResult,
  RunRobotParams,
  RunRobotResult,
  TabularQueryResult,
  FindUsagesResult,
  RefactorRequest,
  PreviewRefactorResult,
  RefactorPlan,
  ApplyRefactorResult,
  SemanticDiffParams,
  SemanticDiffResult,
  ListPluginsResult,
  RunPluginParams,
  RunPluginResult,
  ListCommandsResult,
  WorkspaceUiStateParams,
  WorkspaceUiState,
  GetDialogSchemaResult,
  CreateOntologyParams,
  CreateOntologyResult,
  ExportOntologyParams,
  ExportOntologyResult,
  SetActiveOntologyParams,
  SetActiveOntologyResult,
  DeleteImpactParams,
  DeleteImpactResult,
} from "./protocol";
import { focusRelay } from "../focus/focusRelay";
import {
  bundledServerPath,
  ensureBundledServerExecutable,
} from "./bundledServer";

let client: LanguageClient | undefined;
let starting: Promise<LanguageClient> | undefined;
/** Bumped on stop so in-flight startups do not publish a superseded client (#91). */
let startGeneration = 0;

export function getClient(): LanguageClient | undefined {
  return client;
}

export async function startLanguageClient(
  context: vscode.ExtensionContext
): Promise<LanguageClient> {
  if (client) {
    return client;
  }
  if (starting) {
    return starting;
  }

  const generation = ++startGeneration;
  starting = (async () => {
    const serverPath = resolveServerPath(context);
    const serverOptions: ServerOptions = {
      run: { command: serverPath, transport: TransportKind.stdio },
      debug: { command: serverPath, transport: TransportKind.stdio },
    };

    const clientOptions: LanguageClientOptions = {
      documentSelector: [
        { scheme: "file", language: "turtle" },
        { scheme: "file", language: "obo" },
        { scheme: "file", pattern: "**/*.{ttl,owl,rdf,jsonld,json-ld,nt,nq,trig,obo}" },
      ],
      synchronize: {
        fileEvents: vscode.workspace.createFileSystemWatcher(
          "**/*.{ttl,owl,rdf,jsonld,json-ld,nt,nq,trig,obo}"
        ),
      },
      outputChannelName: "OntoCore Language Server",
    };

    const created = new LanguageClient(
      "ontocore-lsp",
      "OntoCore Language Server",
      serverOptions,
      clientOptions
    );

    context.subscriptions.push({
      dispose: () => {
        void stopLanguageClient();
      },
    });

    try {
      // vscode-languageclient v9: await start() — onReady() was removed.
      await created.start();
      if (generation !== startGeneration) {
        try {
          await created.stop();
        } catch {
          // Best-effort cleanup of a superseded startup.
        }
        throw new Error("OntoCore language server startup was cancelled");
      }
      client = created;
      return created;
    } catch (err) {
      if (client === created) {
        client = undefined;
      }
      const detail = err instanceof Error ? err.message : String(err);
      throw new Error(
        `${detail} (server: ${serverPath}). See Output → OntoCore Language Server.`
      );
    }
  })();

  try {
    return await starting;
  } finally {
    starting = undefined;
  }
}

export async function stopLanguageClient(): Promise<void> {
  startGeneration += 1;
  const inFlight = starting;
  starting = undefined;
  if (inFlight) {
    try {
      const created = await inFlight;
      await created.stop();
    } catch {
      // Startup failed or was cancelled — nothing to stop.
    }
  }
  if (client) {
    const active = client;
    client = undefined;
    await active.stop();
  }
}

function resolveServerPath(context: vscode.ExtensionContext): string {
  const configured = vscode.workspace
    .getConfiguration("ontocode")
    .get<string>("lspPath");
  if (
    configured &&
    configured.trim().length > 0 &&
    fs.existsSync(configured) &&
    vscode.workspace.isTrusted
  ) {
    return configured;
  }
  if (
    configured &&
    configured.trim().length > 0 &&
    fs.existsSync(configured) &&
    !vscode.workspace.isTrusted
  ) {
    void vscode.window.showWarningMessage(
      "OntoCode: ontocode.lspPath is ignored in Restricted Mode; using the bundled language server."
    );
  }

  const bundled = bundledServerPath(context.extensionPath);
  if (fs.existsSync(bundled)) {
    ensureBundledServerExecutable(bundled);
    return bundled;
  }

  throw new Error(
    "Bundled ontocore-lsp binary not found. Rebuild the extension (npm run compile) or set ontocode.lspPath to a local ontocore-lsp binary."
  );
}

export async function indexWorkspace(
  workspaceUri?: string
): Promise<IndexWorkspaceResult> {
  const uri = workspaceUri ?? (await pickWorkspaceFolderUri());
  if (!uri) {
    throw new Error(
      "No workspace folder is open. Open a folder containing ontology files, then run Index Workspace."
    );
  }
  const diskCache = vscode.workspace
    .getConfiguration("ontocode")
    .get<boolean>("indexCache", false);
  const result = await ontcoreRequest<unknown>("ontocore/indexWorkspace", {
    workspace_uri: uri,
    disk_cache: diskCache,
  });
  const indexed = assertIndexWorkspaceResult(result) as IndexWorkspaceResult;
  const { focusRelay } = await import("../focus/focusRelay");
  focusRelay.setCatalogFingerprint({
    indexedAt: indexed.indexed_at,
  });
  return indexed;
}

async function pickWorkspaceFolderUri(): Promise<string | undefined> {
  const folders = vscode.workspace.workspaceFolders;
  if (!folders || folders.length === 0) {
    return undefined;
  }
  if (folders.length === 1) {
    return folders[0].uri.toString();
  }
  const picked = await vscode.window.showWorkspaceFolderPick({
    placeHolder: "Select workspace folder to index",
  });
  return picked?.uri.toString();
}

export async function getCatalogSnapshot(): Promise<CatalogSnapshot> {
  const result = await ontcoreRequest<unknown>(
    "ontocore/getCatalogSnapshot",
    null
  );
  return assertCatalogSnapshot(result);
}

export async function listCommands(): Promise<ListCommandsResult> {
  return ontcoreRequest<ListCommandsResult>("ontocore/listCommands", null);
}

export async function getWorkspaceUiState(
  params: WorkspaceUiStateParams = {}
): Promise<WorkspaceUiState> {
  return ontcoreRequest<WorkspaceUiState>("ontocore/getWorkspaceUiState", params);
}

export async function getDialogSchema(
  dialogId: string
): Promise<GetDialogSchemaResult> {
  return ontcoreRequest<GetDialogSchemaResult>("ontocore/getDialogSchema", {
    dialog_id: dialogId,
  });
}

export async function createOntology(
  params: CreateOntologyParams
): Promise<CreateOntologyResult> {
  return ontcoreRequest<CreateOntologyResult>("ontocore/createOntology", params);
}

export async function exportOntology(
  params: ExportOntologyParams
): Promise<ExportOntologyResult> {
  return ontcoreRequest<ExportOntologyResult>("ontocore/exportOntology", params);
}

export async function setActiveOntology(
  params: SetActiveOntologyParams
): Promise<SetActiveOntologyResult> {
  return ontcoreRequest<SetActiveOntologyResult>(
    "ontocore/setActiveOntology",
    params
  );
}

export async function deleteImpact(
  params: DeleteImpactParams
): Promise<DeleteImpactResult> {
  return ontcoreRequest<DeleteImpactResult>("ontocore/deleteImpact", params);
}

export async function listPlugins(): Promise<ListPluginsResult> {
  const result = await ontcoreRequest<unknown>("ontocore/listPlugins", null);
  return assertListPluginsResult(result);
}

export async function runPlugin(params: RunPluginParams): Promise<RunPluginResult> {
  const result = await ontcoreRequest<unknown>("ontocore/runPlugin", {
    plugin_id: params.plugin_id,
    action: params.action ?? "validate",
    step: params.step,
    view_id: params.view_id,
  });
  return assertRunPluginResult(result);
}

export async function getEntity(iri: string): Promise<GetEntityResult> {
  const result = await ontcoreRequest<unknown>("ontocore/getEntity", { iri });
  return assertGetEntityResult(result);
}

export async function applyAxiomPatch(
  params: ApplyAxiomPatchParams
): Promise<ApplyAxiomPatchClientResult> {
  const result = await ontcoreRequest<unknown>(
    "ontocore/applyAxiomPatch",
    params
  );
  const patch = assertApplyPatchResult(result);
  if (patch.applied) {
    focusRelay.markReasoningDirty();
  }
  if (patch.applied && patch.workspace_edit) {
    const { applyLspWorkspaceEdit } = await import("./workspaceEdit");
    const synced = await applyLspWorkspaceEdit(patch.workspace_edit);
    if (!synced) {
      void vscode.window.showWarningMessage(patchSyncCancelledMessage());
      return { ...patch, editor_synced: false };
    }
    return { ...patch, editor_synced: true };
  }
  return { ...patch, editor_synced: true };
}

type SqlTableSchema = {
  name: string;
  columns: Array<{ name: string; type: string }>;
};

function parseListSqlSchemaResult(result: unknown): SqlTableSchema[] {
  if (Array.isArray(result)) {
    return result as SqlTableSchema[];
  }
  if (
    result &&
    typeof result === "object" &&
    Array.isArray((result as { tables?: unknown }).tables)
  ) {
    return (result as { tables: SqlTableSchema[] }).tables;
  }
  throw new Error("Invalid listSqlSchema response");
}

export async function listSqlSchema(): Promise<SqlTableSchema[]> {
  const result = await ontcoreRequest<unknown>("ontocore/listSqlSchema", {});
  return parseListSqlSchemaResult(result);
}

export async function runSqlQuery(sql: string): Promise<TabularQueryResult> {
  const result = await ontcoreRequest<unknown>("ontocore/query", { sql });
  return assertTabularQueryResult(result);
}

export async function runSparqlQuery(
  query: string
): Promise<TabularQueryResult> {
  const result = await ontcoreRequest<unknown>("ontocore/sparql", { query });
  return assertTabularQueryResult(result);
}

export async function parseManchester(
  params: ParseManchesterParams
): Promise<ParseManchesterResult> {
  const result = await ontcoreRequest<unknown>(
    "ontocore/parseManchester",
    params
  );
  return assertParseManchesterResult(result);
}

/** Active reasoner RPC cancellation (Stop / progress Cancel). */
let reasonerCancelSource: vscode.CancellationTokenSource | undefined;

export function cancelActiveReasonerRequest(): void {
  reasonerCancelSource?.cancel();
  reasonerCancelSource?.dispose();
  reasonerCancelSource = undefined;
}

export function isReasonerRequestActive(): boolean {
  return reasonerCancelSource !== undefined;
}

export async function runReasoner(
  params: RunReasonerParams,
  token?: vscode.CancellationToken
): Promise<RunReasonerResult> {
  cancelActiveReasonerRequest();
  const source = new vscode.CancellationTokenSource();
  reasonerCancelSource = source;
  if (token) {
    if (token.isCancellationRequested) {
      source.cancel();
    } else {
      token.onCancellationRequested(() => source.cancel());
    }
  }
  try {
    const result = await ontcoreRequest<unknown>(
      "ontocore/runReasoner",
      params,
      source.token
    );
    return assertRunReasonerResult(result);
  } finally {
    if (reasonerCancelSource === source) {
      reasonerCancelSource = undefined;
    }
    source.dispose();
  }
}

export async function getExplanation(
  params: GetExplanationParams
): Promise<GetExplanationResult> {
  const result = await ontcoreRequest<unknown>(
    "ontocore/getExplanation",
    params
  );
  const explained = assertGetExplanationResult(result);
  return explained;
}

export async function getGraph(params: GetGraphParams): Promise<GetGraphResult> {
  const result = await ontcoreRequest<unknown>("ontocore/getGraph", params);
  return assertGetGraphResult(result);
}

export async function runRobot(params: RunRobotParams): Promise<RunRobotResult> {
  let robotPath =
    params.robot_path ??
    vscode.workspace.getConfiguration("ontocode").get<string>("robotPath");
  if (robotPath && robotPath.trim().length > 0 && !vscode.workspace.isTrusted) {
    void vscode.window.showWarningMessage(
      "OntoCode: ontocode.robotPath is ignored in Restricted Mode; using robot on PATH."
    );
    robotPath = undefined;
  }
  const result = await ontcoreRequest<unknown>("ontocore/runRobot", {
    ...params,
    robot_path: robotPath || undefined,
  });
  return assertRunRobotResult(result);
}

export async function findUsages(iri: string): Promise<FindUsagesResult> {
  const result = await ontcoreRequest<unknown>("ontocore/findUsages", { iri });
  return assertFindUsagesResult(result);
}

export async function previewRefactor(
  request: RefactorRequest
): Promise<PreviewRefactorResult> {
  const result = await ontcoreRequest<unknown>(
    "ontocore/previewRefactor",
    request
  );
  return assertPreviewRefactorResult(result);
}

export async function applyRefactor(
  plan: RefactorPlan,
  request: RefactorRequest,
  previewOnly = false
): Promise<ApplyRefactorResult> {
  const result = await ontcoreRequest<unknown>("ontocore/applyRefactor", {
    plan,
    request,
    preview_only: previewOnly,
  });
  return assertApplyRefactorResult(result);
}

export async function semanticDiff(
  params: SemanticDiffParams
): Promise<SemanticDiffResult> {
  const result = await ontcoreRequest<unknown>("ontocore/semanticDiff", params);
  return assertSemanticDiffResult(result);
}

interface LspErrorPayload {
  code?: string;
  message?: string;
  user_action?: string;
  recoverable?: boolean;
}

function formatOntocoreRpcError(err: unknown): Error {
  if (err && typeof err === "object") {
    const rpc = err as {
      code?: number;
      message?: string;
      data?: LspErrorPayload;
    };
    const data = rpc.data;
    if (data?.message) {
      const action = data.user_action ? ` ${data.user_action}` : "";
      return new Error(`${data.code ?? "LSP_ERROR"}: ${data.message}${action}`);
    }
    if (rpc.message) {
      return new Error(rpc.message);
    }
  }
  if (err instanceof Error) {
    return err;
  }
  return new Error(String(err));
}

async function ontcoreRequest<T>(
  method: string,
  params: unknown,
  token?: vscode.CancellationToken
): Promise<T> {
  try {
    if (token) {
      return await requireClient().sendRequest<T>(method, params, token);
    }
    return await requireClient().sendRequest<T>(method, params);
  } catch (err) {
    throw formatOntocoreRpcError(err);
  }
}

function requireClient(): LanguageClient {
  if (!client) {
    throw new Error("OntoCore language server is not running");
  }
  return client;
}
