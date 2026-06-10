import * as vscode from "vscode";
import { registerCommands, refreshExplorer } from "./commands";
import {
  indexWorkspace,
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

export async function activate(
  context: vscode.ExtensionContext
): Promise<void> {
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
    )
  );

  try {
    await startLanguageClient(context);
    registerCommands(context, providers);

    const autoIndex = vscode.workspace
      .getConfiguration("ontocode")
      .get<boolean>("autoIndexOnOpen", true);
    if (autoIndex) {
      await indexWorkspace();
      await refreshExplorer(providers);
    }
  } catch (err) {
    const message = err instanceof Error ? err.message : String(err);
    vscode.window.showErrorMessage(
      `OntoCode failed to start language server: ${message}`
    );
  }
}

export async function deactivate(): Promise<void> {
  await stopLanguageClient();
  providers = undefined;
}
