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
} from "./protocolGuards";
import {
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
} from "./protocol";
import {
  bundledServerPath,
  ensureBundledServerExecutable,
} from "./bundledServer";

let client: LanguageClient | undefined;

export function getClient(): LanguageClient | undefined {
  return client;
}

export async function startLanguageClient(
  context: vscode.ExtensionContext
): Promise<LanguageClient> {
  if (client) {
    return client;
  }

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

  client = new LanguageClient(
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
    await client.start();
  } catch (err) {
    client = undefined;
    const detail = err instanceof Error ? err.message : String(err);
    throw new Error(
      `${detail} (server: ${serverPath}). See Output → OntoCore Language Server.`
    );
  }

  return client;
}

export async function stopLanguageClient(): Promise<void> {
  if (client) {
    await client.stop();
    client = undefined;
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
  const c = requireClient();
  const uri = workspaceUri ?? (await pickWorkspaceFolderUri());
  if (!uri) {
    throw new Error(
      "No workspace folder is open. Open a folder containing ontology files, then run Index Workspace."
    );
  }
  const diskCache = vscode.workspace
    .getConfiguration("ontocode")
    .get<boolean>("indexCache", false);
  const result = await c.sendRequest<unknown>("ontocore/indexWorkspace", {
    workspace_uri: uri,
    disk_cache: diskCache,
  });
  return assertIndexWorkspaceResult(result) as IndexWorkspaceResult;
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
  const c = requireClient();
  const result = await c.sendRequest<unknown>(
    "ontocore/getCatalogSnapshot",
    null
  );
  return assertCatalogSnapshot(result);
}

export async function getEntity(iri: string): Promise<GetEntityResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>("ontocore/getEntity", { iri });
  return assertGetEntityResult(result);
}

export async function applyAxiomPatch(
  params: ApplyAxiomPatchParams
): Promise<ApplyPatchResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>(
    "ontocore/applyAxiomPatch",
    params
  );
  const patch = assertApplyPatchResult(result);
  if (patch.applied && patch.workspace_edit) {
    const { applyLspWorkspaceEdit } = await import("./workspaceEdit");
    const synced = await applyLspWorkspaceEdit(patch.workspace_edit);
    if (!synced) {
      void vscode.window.showWarningMessage(
        "OntoCode: changes written to disk but editor sync was cancelled"
      );
    }
  }
  return patch;
}

export async function runSqlQuery(sql: string): Promise<TabularQueryResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>("ontocore/query", { sql });
  return assertTabularQueryResult(result);
}

export async function runSparqlQuery(
  query: string
): Promise<TabularQueryResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>("ontocore/sparql", { query });
  return assertTabularQueryResult(result);
}

export async function parseManchester(
  params: ParseManchesterParams
): Promise<ParseManchesterResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>(
    "ontocore/parseManchester",
    params
  );
  return assertParseManchesterResult(result);
}

export async function runReasoner(
  params: RunReasonerParams
): Promise<RunReasonerResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>("ontocore/runReasoner", params);
  return assertRunReasonerResult(result);
}

export async function getExplanation(
  params: GetExplanationParams
): Promise<GetExplanationResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>(
    "ontocore/getExplanation",
    params
  );
  return assertGetExplanationResult(result);
}

export async function getGraph(params: GetGraphParams): Promise<GetGraphResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>("ontocore/getGraph", params);
  return assertGetGraphResult(result);
}

export async function runRobot(params: RunRobotParams): Promise<RunRobotResult> {
  const c = requireClient();
  let robotPath =
    params.robot_path ??
    vscode.workspace.getConfiguration("ontocode").get<string>("robotPath");
  if (robotPath && robotPath.trim().length > 0 && !vscode.workspace.isTrusted) {
    void vscode.window.showWarningMessage(
      "OntoCode: ontocode.robotPath is ignored in Restricted Mode; using robot on PATH."
    );
    robotPath = undefined;
  }
  const result = await c.sendRequest<unknown>("ontocore/runRobot", {
    ...params,
    robot_path: robotPath || undefined,
  });
  return assertRunRobotResult(result);
}

export async function findUsages(iri: string): Promise<FindUsagesResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>("ontocore/findUsages", { iri });
  return assertFindUsagesResult(result);
}

export async function previewRefactor(
  request: RefactorRequest
): Promise<PreviewRefactorResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>(
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
  const c = requireClient();
  const result = await c.sendRequest<unknown>("ontocore/applyRefactor", {
    plan,
    request,
    preview_only: previewOnly,
  });
  return assertApplyRefactorResult(result);
}

export async function semanticDiff(
  params: SemanticDiffParams
): Promise<SemanticDiffResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>("ontocore/semanticDiff", params);
  return assertSemanticDiffResult(result);
}

function requireClient(): LanguageClient {
  if (!client) {
    throw new Error("OntoCore language server is not running");
  }
  return client;
}
