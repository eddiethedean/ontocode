import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";
import {
  CatalogSnapshot,
  GetEntityResult,
  IndexWorkspaceResult,
} from "./protocol";
import {
  assertCatalogSnapshot,
  assertIndexWorkspaceResult,
} from "./protocolGuards";

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
  };

  client = new LanguageClient(
    "ontoindex-lsp",
    "OntoIndex Language Server",
    serverOptions,
    clientOptions
  );

  context.subscriptions.push(client.start());
  await client.onReady();
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

  const platform = process.platform;
  const arch = process.arch;
  const bundled = path.join(
    context.extensionPath,
    "server",
    `${platform}-${arch}`,
    platform === "win32" ? "ontoindex-lsp.exe" : "ontoindex-lsp"
  );
  if (fs.existsSync(bundled)) {
    ensureBundledServerExecutable(bundled);
    return bundled;
  }

  return "ontoindex-lsp";
}

/** VSIX/Marketplace installs often drop the Unix executable bit on bundled binaries. */
function ensureBundledServerExecutable(serverPath: string): void {
  if (process.platform === "win32") {
    return;
  }
  try {
    const mode = fs.statSync(serverPath).mode;
    if ((mode & 0o111) === 0) {
      fs.chmodSync(serverPath, mode | 0o755);
    }
  } catch {
    // Spawn will fail with a clear error if this cannot be fixed.
  }
}

export async function indexWorkspace(
  workspaceUri?: string
): Promise<IndexWorkspaceResult> {
  const c = requireClient();
  const uri =
    workspaceUri ??
    vscode.workspace.workspaceFolders?.[0]?.uri.toString();
  const result = await c.sendRequest<unknown>("ontoindex/indexWorkspace", {
    workspaceUri: uri,
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
  return c.sendRequest<GetEntityResult>("ontoindex/getEntity", { iri });
}

function requireClient(): LanguageClient {
  if (!client) {
    throw new Error("OntoIndex language server is not running");
  }
  return client;
}
