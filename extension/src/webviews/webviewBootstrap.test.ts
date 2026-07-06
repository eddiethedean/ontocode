import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
  assertWebviewHtmlRoutesPanel,
  formatWebviewQuery,
  webviewLocationBootstrapScript,
} from "./webviewBootstrap";

describe("webviewBootstrap", () => {
  it("formats panel and extra query params", () => {
    const q = formatWebviewQuery("graph", { graphKind: "class", root: "ex:A" });
    assert.ok(q.includes("panel=graph"));
    assert.ok(q.includes("graphKind=class"));
    assert.ok(q.includes("root=ex%3AA") || q.includes("root=ex:A"));
  });

  it("bootstrap script sets location.search when empty", () => {
    const script = webviewLocationBootstrapScript("panel=inspector");
    assert.match(script, /history\.replaceState/);
    assert.match(script, /panel=inspector/);
  });

  it("accepts HTML with location bootstrap for inspector", () => {
    const html = `<script nonce="n">
${webviewLocationBootstrapScript("panel=inspector")}
</script>
<script type="module" src="vscode-webview://x/assets/index.js"></script>`;
    assert.doesNotThrow(() =>
      assertWebviewHtmlRoutesPanel(html, "inspector")
    );
  });

  it("rejects script-src-only panel routing (regression)", () => {
    const html = `<script type="module" src="vscode-webview://x/assets/index.js?panel=inspector"></script>`;
    assert.throws(
      () => assertWebviewHtmlRoutesPanel(html, "inspector"),
      /script src query only/
    );
  });

  it("rejects HTML without panel query", () => {
    const html = `<script>history.replaceState(null, "", "?panel=smoke");</script>`;
    assert.throws(
      () => assertWebviewHtmlRoutesPanel(html, "inspector"),
      /missing panel=inspector/
    );
  });
});
