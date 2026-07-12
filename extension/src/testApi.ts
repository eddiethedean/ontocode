import * as vscode from "vscode";
import type {
  DialogPanelKind,
  InjectablePanelKind,
  OntoCodeTestHooks,
} from "./api";
import {
  openNewOntologyDialog as openNewOntologyDialogAt,
  openPrefixManager as openPrefixManagerAt,
} from "./commands/v017Commands";
import { focusRelay } from "./focus/focusRelay";
import { EntityInspectorPanel } from "./webviews/inspector";
import { PanelHost } from "./webviews/panelHost";
import { QueryWorkbenchPanel } from "./webviews/queryWorkbenchReact";
import { assertWebviewHtmlRoutesPanel } from "./webviews/webviewBootstrap";
import type { PanelKind } from "./webviews/messages";

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function waitForWebviewReady(
  isReady: () => boolean,
  timeoutMs: number
): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  while (Date.now() < deadline) {
    if (isReady()) {
      return;
    }
    await sleep(100);
  }
  throw new Error("webview did not report ready before timeout");
}

function requireDialogHost(panel: DialogPanelKind): PanelHost {
  const host = PanelHost.getOpen(panel);
  if (!host || host.isDisposed) {
    throw new Error(`${panel} webview is not open`);
  }
  return host;
}

/** Test hooks exposed when ONTOCODE_TEST_FIXTURES is set (VS Code e2e). */
export function createOntoCodeTestHooks(): OntoCodeTestHooks {
  return {
    async openEntityInspector(iri: string): Promise<void> {
      await vscode.commands.executeCommand("ontocode.showEntityInspector", iri);
    },

    getInspectorWebviewHtml(): string | undefined {
      return EntityInspectorPanel.currentPanel?.getWebviewHtmlForTests();
    },

    assertInspectorHtmlRoutesPanel(): void {
      const html = EntityInspectorPanel.currentPanel?.getWebviewHtmlForTests();
      if (!html) {
        throw new Error("entity inspector webview is not open");
      }
      assertWebviewHtmlRoutesPanel(html, "inspector");
    },

    async waitForInspectorReady(timeoutMs = 20_000): Promise<void> {
      const panel = EntityInspectorPanel.currentPanel;
      if (!panel) {
        throw new Error("entity inspector webview is not open");
      }
      await waitForWebviewReady(
        () => panel.isWebviewReadyForTests(),
        timeoutMs
      );
    },

    async openQueryWorkbench(): Promise<void> {
      await vscode.commands.executeCommand("ontocode.openQueryWorkbench");
    },

    getQueryWorkbenchWebviewHtml(): string | undefined {
      return QueryWorkbenchPanel.current?.getWebviewHtmlForTests();
    },

    assertQueryWorkbenchHtmlRoutesPanel(): void {
      const html = QueryWorkbenchPanel.current?.getWebviewHtmlForTests();
      if (!html) {
        throw new Error("query workbench webview is not open");
      }
      assertWebviewHtmlRoutesPanel(html, "queryWorkbench");
    },

    async waitForQueryWorkbenchReady(timeoutMs = 20_000): Promise<void> {
      const panel = QueryWorkbenchPanel.current;
      if (!panel) {
        throw new Error("query workbench webview is not open");
      }
      await waitForWebviewReady(
        () => panel.isWebviewReadyForTests(),
        timeoutMs
      );
    },

    async openEntity(iri: string): Promise<void> {
      await vscode.commands.executeCommand("ontocode.openEntity", iri);
    },

    getInspectorLoadedIri(): string | undefined {
      return EntityInspectorPanel.currentPanel?.getLoadedIriForTests();
    },

    getInspectorPanelTitle(): string | undefined {
      return EntityInspectorPanel.currentPanel?.getPanelTitleForTests();
    },

    async waitForInspectorIri(iri: string, timeoutMs = 20_000): Promise<void> {
      const deadline = Date.now() + timeoutMs;
      while (Date.now() < deadline) {
        if (EntityInspectorPanel.currentPanel?.getLoadedIriForTests() === iri) {
          return;
        }
        await sleep(100);
      }
      const loaded = EntityInspectorPanel.currentPanel?.getLoadedIriForTests();
      throw new Error(
        `inspector did not load ${iri} before timeout (last IRI: ${loaded ?? "none"})`
      );
    },

    getInspectorPanelRef(): object | undefined {
      return EntityInspectorPanel.currentPanel;
    },

    getCanonicalFocus() {
      return focusRelay.getFocus();
    },

    async openNewOntologyDialog(targetPath: string): Promise<void> {
      openNewOntologyDialogAt(targetPath);
    },

    async openPrefixManager(documentPath: string): Promise<void> {
      await openPrefixManagerAt(documentPath);
    },

    async postWebviewMessage(
      panel: InjectablePanelKind,
      msg: unknown
    ): Promise<void> {
      const host = PanelHost.getOpen(panel);
      if (!host || host.isDisposed) {
        throw new Error(`${panel} webview is not open`);
      }
      await host.deliverMessageForTests(msg);
    },

    async waitForPanelReady(
      panel: DialogPanelKind,
      timeoutMs = 20_000
    ): Promise<void> {
      const host = requireDialogHost(panel);
      await waitForWebviewReady(() => host.isWebviewReady(), timeoutMs);
    },

    getPanelHtml(panel: DialogPanelKind): string | undefined {
      return PanelHost.getOpen(panel)?.getWebviewHtml();
    },

    async disposePanel(panel: DialogPanelKind): Promise<void> {
      PanelHost.disposeKinds([panel]);
      await sleep(50);
    },

    async disposeAllPanels(): Promise<void> {
      EntityInspectorPanel.currentPanel?.disposeForTests();
      QueryWorkbenchPanel.current?.disposeForTests();
      PanelHost.disposeKinds(["newOntology", "prefixManager"]);
      await sleep(100);
    },

    assertWebviewHtmlRoutesPanel(html: string, panel: PanelKind): void {
      assertWebviewHtmlRoutesPanel(html, panel);
    },
  };
}
