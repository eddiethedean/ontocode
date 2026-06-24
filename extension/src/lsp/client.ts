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
} from "./protocolGuards";
import {
  ApplyAxiomPatchParams,
  ApplyPatchResult,
  CatalogSnapshot,
  GetEntityResult,
  GetExplanationParams,
  GetExplanationResult,
  IndexWorkspaceResult,
  ParseManchesterParams,
  ParseManchesterResult,
  RunReasonerParams,
  RunReasonerResult,
  TabularQueryResult,
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
      { scheme: "file", pattern: "**/*.{ttl,owl,rdf,jsonld,nt,nq,trig}" },
    ],
    synchronize: {
      fileEvents: vscode.workspace.createFileSystemWatcher(
        "**/*.{ttl,owl,rdf,jsonld,nt,nq,trig}"
      ),
    },
    outputChannelName: "OntoIndex Language Server",
  };

  client = new LanguageClient(
    "ontoindex-lsp",
    "OntoIndex Language Server",
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
      `${detail} (server: ${serverPath}). See Output → OntoIndex Language Server.`
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

  return "ontoindex-lsp";
}

export async function indexWorkspace(
  workspaceUri?: string
): Promise<IndexWorkspaceResult> {
  const c = requireClient();
  const uri =
    workspaceUri ??
    vscode.workspace.workspaceFolders?.[0]?.uri.toString();
  const result = await c.sendRequest<unknown>("ontoindex/indexWorkspace", {
    workspace_uri: uri,
  });
  return assertIndexWorkspaceResult(result) as IndexWorkspaceResult;
}

export async function getCatalogSnapshot(): Promise<CatalogSnapshot> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>(
    "ontoindex/getCatalogSnapshot",
    null
  );
  return assertCatalogSnapshot(result);
}

export async function getEntity(iri: string): Promise<GetEntityResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>("ontoindex/getEntity", { iri });
  return assertGetEntityResult(result);
}

export async function applyAxiomPatch(
  params: ApplyAxiomPatchParams
): Promise<ApplyPatchResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>(
    "ontoindex/applyAxiomPatch",
    params
  );
  return assertApplyPatchResult(result);
}

export async function runSqlQuery(sql: string): Promise<TabularQueryResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>("ontoindex/query", { sql });
  return assertTabularQueryResult(result);
}

export async function runSparqlQuery(
  query: string
): Promise<TabularQueryResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>("ontoindex/sparql", { query });
  return assertTabularQueryResult(result);
}

export async function parseManchester(
  params: ParseManchesterParams
): Promise<ParseManchesterResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>(
    "ontoindex/parseManchester",
    params
  );
  return assertParseManchesterResult(result);
}

export async function runReasoner(
  params: RunReasonerParams
): Promise<RunReasonerResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>("ontoindex/runReasoner", params);
  return assertRunReasonerResult(result);
}

export async function getExplanation(
  params: GetExplanationParams
): Promise<GetExplanationResult> {
  const c = requireClient();
  const result = await c.sendRequest<unknown>(
    "ontoindex/getExplanation",
    params
  );
  return assertGetExplanationResult(result);
}

function requireClient(): LanguageClient {
  if (!client) {
    throw new Error("OntoIndex language server is not running");
  }
  return client;
}
