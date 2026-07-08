import * as vscode from "vscode";
import { listPlugins } from "../lsp/client";
import type { PluginDescriptor } from "../lsp/protocol";
import { WorkflowPanel } from "../webviews/workflowPanel";

const registered = new Map<string, vscode.Disposable>();

export async function refreshPluginCommands(
  context: vscode.ExtensionContext
): Promise<PluginDescriptor[]> {
  for (const d of registered.values()) {
    d.dispose();
  }
  registered.clear();

  let plugins: PluginDescriptor[] = [];
  try {
    const result = await listPlugins();
    plugins = result.plugins;
  } catch {
    return plugins;
  }

  for (const plugin of plugins) {
    for (const cmd of plugin.ui.commands) {
      const commandId = `ontocode.plugin.${cmd.id}`;
      const disposable = vscode.commands.registerCommand(commandId, async () => {
        if (plugin.id === "owlmake" || cmd.id.includes("owlmake")) {
          await WorkflowPanel.runOwlmake("qc");
          return;
        }
        void vscode.window.showInformationMessage(
          `OntoCode plugin: ${plugin.name} — ${cmd.title}`
        );
      });
      registered.set(commandId, disposable);
      context.subscriptions.push(disposable);
    }
  }

  return plugins;
}

export function disposePluginCommands(): void {
  for (const d of registered.values()) {
    d.dispose();
  }
  registered.clear();
}
