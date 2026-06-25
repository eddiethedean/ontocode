import * as vscode from "vscode";
import { OntoCodeApi } from "./api";
import { registerCommands, refreshExplorer } from "./commands";
import {
  getCatalogSnapshot,
  getClient,
  getEntity,
  indexWorkspace,
  parseManchester,
  runSqlQuery,
  runSparqlQuery,
  startLanguageClient,
  stopLanguageClient,
} from "./lsp/client";
import { ExplorerTreeProvider } from "./treeviews/explorer";

let providers: {
  ontologies: ExplorerTreeProvider;
  classes: ExplorerTreeProvider;
  properties: ExplorerTreeProvider;
  individuals: ExplorerTreeProvider;
  diagnostics: ExplorerTreeProvider;
} | undefined;

let diagnosticsRefreshTimer: ReturnType<typeof setTimeout> | undefined;

export async function activate(
  context: vscode.ExtensionContext
): Promise<OntoCodeApi> {
  try {
    await startLanguageClient(context);

    providers = {
      ontologies: new ExplorerTreeProvider("ontologies"),
      classes: new ExplorerTreeProvider("classes"),
      properties: new ExplorerTreeProvider("properties"),
      individuals: new ExplorerTreeProvider("individuals"),
      diagnostics: new ExplorerTreeProvider("diagnostics"),
    };

    context.subscriptions.push(
      vscode.window.registerTreeDataProvider(
        "ontocode.explorer.ontologies",
        providers.ontologies
      ),
      vscode.window.registerTreeDataProvider(
        "ontocode.explorer.classes",
        providers.classes
      ),
      vscode.window.registerTreeDataProvider(
        "ontocode.explorer.properties",
        providers.properties
      ),
      vscode.window.registerTreeDataProvider(
        "ontocode.explorer.individuals",
        providers.individuals
      ),
      vscode.window.registerTreeDataProvider(
        "ontocode.explorer.diagnostics",
        providers.diagnostics
      ),
      vscode.languages.onDidChangeDiagnostics((event) => {
        if (!providers) {
          return;
        }
        const ours = event.uris.some((uri) =>
          vscode.languages
            .getDiagnostics(uri)
            .some((d) => d.source === "ontoindex")
        );
        if (!ours) {
          return;
        }
        if (diagnosticsRefreshTimer) {
          clearTimeout(diagnosticsRefreshTimer);
        }
        diagnosticsRefreshTimer = setTimeout(() => {
          void refreshExplorer(providers!);
        }, 300);
      })
    );

    registerCommands(context, providers);

    context.subscriptions.push(
      vscode.workspace.onDidChangeConfiguration((e) => {
        if (e.affectsConfiguration("ontocode.hierarchy.mode") && providers) {
          void refreshExplorer(providers);
        }
      })
    );

    // Server indexes on `initialized` (debounced); refresh explorer once it settles.
    setTimeout(() => {
      void refreshExplorer(providers!);
    }, 900);

    return {
      getClient,
      indexWorkspace,
      getCatalogSnapshot,
      getEntity,
      runSqlQuery,
      runSparqlQuery,
      parseManchester: (
        expression: string,
        axiomKind: string,
        entityIri?: string,
        documentUri?: string
      ) =>
        parseManchester({
          expression,
          axiom_kind: axiomKind,
          entity_iri: entityIri,
          document_uri: documentUri,
        }),
    };
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    void vscode.window.showErrorMessage(
      `OntoCode failed to start language server: ${message}`
    );
    throw err;
  }
}

export async function deactivate(): Promise<void> {
  if (diagnosticsRefreshTimer) {
    clearTimeout(diagnosticsRefreshTimer);
  }
  await stopLanguageClient();
  providers = undefined;
}
