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

interface OntoCodeApi {
  getClient(): LanguageClient | undefined;
  indexWorkspace(workspaceUri?: string): Promise<{
    stats: { error_count: number; class_count: number };
  }>;
  getCatalogSnapshot(): Promise<{
    entities: Array<{ iri: string; kind: string }>;
  }>;
}

suite("OntoCode in VS Code", () => {
  let api: OntoCodeApi;

  suiteSetup(async function () {
    this.timeout(120_000);
    const ext = vscode.extensions.getExtension("ontocode.ontocode");
    assert.ok(ext, "OntoCode extension must be loaded");
    api = await ext.activate();
    assert.ok(ext.isActive, "extension should be active after activate()");
  });

  test("language client starts (bundled ontoindex-lsp)", () => {
    assert.ok(api.getClient(), "LSP client should be running");
    assert.ok(api.getClient()!.isRunning?.() ?? true, "client should report running");
  });

  test("indexes fixture workspace and returns Person in catalog", async function () {
    this.timeout(60_000);
    const workspaceUri = fixturesWorkspaceUri();

    const result = await api.indexWorkspace(workspaceUri);
    assert.equal(result.stats.error_count, 0);
    assert.ok(result.stats.class_count >= 3);

    const snapshot = await api.getCatalogSnapshot();
    const person = snapshot.entities.find(
      (e) => e.iri === "http://example.org/people#Person"
    );
    assert.ok(person, "Person class should be in catalog");
    assert.equal(person?.kind, "class");
  });
});
