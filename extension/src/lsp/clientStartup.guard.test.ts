import assert from "node:assert/strict";
import * as fs from "node:fs";
import * as path from "path";
import { describe, it } from "node:test";

const srcDir = path.join(__dirname, "..", "..", "src", "lsp");
const clientSource = fs.readFileSync(path.join(srcDir, "client.ts"), "utf8");
const bundledServerSource = fs.readFileSync(
  path.join(srcDir, "bundledServer.ts"),
  "utf8"
);
const distBundle = path.join(__dirname, "..", "..", "dist", "extension.js");

describe("language client startup guard", () => {
  it("does not call removed vscode-languageclient v9 onReady()", () => {
    assert.doesNotMatch(clientSource, /\.onReady\s*\(/);
  });

  it("awaits LanguageClient.start() before returning the client", () => {
    assert.match(clientSource, /await client\.start\(\)/);
  });

  it("does not push client.start() promise onto subscriptions without awaiting", () => {
    assert.doesNotMatch(
      clientSource,
      /subscriptions\.push\s*\(\s*client\.start\s*\(\s*\)\s*\)/
    );
  });

  it("shipped extension bundle matches startup guard", () => {
    assert.ok(
      fs.existsSync(distBundle),
      "run `npm run compile` before tests — dist/extension.js missing"
    );
    const dist = fs.readFileSync(distBundle, "utf8");
    assert.doesNotMatch(dist, /\.onReady\s*\(/);
    assert.match(dist, /await client\.start\(\)/);
  });
});

describe("bundled server launch guard", () => {
  it("chmod-fixes non-executable bundled binaries before launch", () => {
    assert.match(bundledServerSource, /ensureBundledServerExecutable/);
    assert.match(bundledServerSource, /0o755|493/);
  });

  it("client resolves bundled path through ensureBundledServerExecutable", () => {
    assert.match(clientSource, /ensureBundledServerExecutable\s*\(\s*bundled\s*\)/);
  });
});
