import * as assert from "assert";
import * as path from "path";
import * as vscode from "vscode";
import { LanguageClient } from "vscode-languageclient/node";

function fixturesWorkspaceUri(): string {
  const fromEnv = process.env.ONTOCODE_TEST_FIXTURES;
  if (fromEnv) {
    return vscode.Uri.file(fromEnv).toString();
  }
  return vscode.Uri.file(path.resolve(__dirname, "..", "..", "..", "..", "fixtures")).toString();
}

interface OntoCodeTestHooks {
  openEntityInspector(iri: string): Promise<void>;
  getInspectorWebviewHtml(): string | undefined;
  assertInspectorHtmlRoutesPanel(): void;
  waitForInspectorReady(timeoutMs?: number): Promise<void>;
  openQueryWorkbench(): Promise<void>;
  getQueryWorkbenchWebviewHtml(): string | undefined;
  assertQueryWorkbenchHtmlRoutesPanel(): void;
  waitForQueryWorkbenchReady(timeoutMs?: number): Promise<void>;
  disposeAllPanels(): Promise<void>;
}

interface OntoCodeTestApi {
  getClient(): LanguageClient | undefined;
  indexWorkspace(workspaceUri?: string): Promise<{
    stats: { error_count: number; class_count: number };
  }>;
  __test: OntoCodeTestHooks;
}

suite("OntoCode React webviews", () => {
  let api: OntoCodeTestApi;

  suiteSetup(async function () {
    this.timeout(120_000);
    const ext = vscode.extensions.getExtension("ontocode.ontocode");
    assert.ok(ext, "OntoCode extension must be loaded");
    const activated = await ext.activate();
    assert.ok(activated.__test, "ONTOCODE_TEST_FIXTURES must enable __test hooks");
    api = activated as OntoCodeTestApi;
  });

  suiteTeardown(async () => {
    await api.__test.disposeAllPanels();
  });

  test("entity inspector HTML routes React to inspector panel", async function () {
    this.timeout(60_000);
    const workspaceUri = fixturesWorkspaceUri();
    const personIri = "http://example.org/people#Person";

    await api.indexWorkspace(workspaceUri);
    await api.__test.openEntityInspector(personIri);

    const html = api.__test.getInspectorWebviewHtml();
    assert.ok(html, "inspector webview should be open");
    api.__test.assertInspectorHtmlRoutesPanel();
    assert.match(html!, /panel=inspector/, "bootstrap must include panel=inspector");
    assert.doesNotMatch(
      html!,
      /<script[^>]+src="[^"]*\?[^"]*panel=/,
      "panel query must not live only on script src"
    );
  });

  test("entity inspector webview reports ready for inspector panel", async function () {
    this.timeout(60_000);
    const workspaceUri = fixturesWorkspaceUri();
    const personIri = "http://example.org/people#Person";

    await api.indexWorkspace(workspaceUri);
    await api.__test.disposeAllPanels();
    await api.__test.openEntityInspector(personIri);
    await api.__test.waitForInspectorReady();
  });

  test("query workbench HTML routes React to queryWorkbench panel", async function () {
    this.timeout(60_000);
    await api.__test.disposeAllPanels();
    await api.__test.openQueryWorkbench();

    const html = api.__test.getQueryWorkbenchWebviewHtml();
    assert.ok(html, "query workbench webview should be open");
    api.__test.assertQueryWorkbenchHtmlRoutesPanel();
  });

  test("query workbench webview reports ready for queryWorkbench panel", async function () {
    this.timeout(60_000);
    await api.__test.disposeAllPanels();
    await api.__test.openQueryWorkbench();
    await api.__test.waitForQueryWorkbenchReady();
  });
});
