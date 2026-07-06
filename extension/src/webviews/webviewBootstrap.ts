import type { PanelKind } from "./messages";

/** Build the `?panel=…` query string embedded in webview HTML. */
export function formatWebviewQuery(
  panel: PanelKind,
  extraQuery?: Record<string, string>
): string {
  return new URLSearchParams({ panel, ...extraQuery }).toString();
}

/**
 * Inline script run before the React bundle so `window.location.search` matches
 * what App.tsx reads (VS Code webview pages do not inherit script-src queries).
 */
export function webviewLocationBootstrapScript(query: string): string {
  return `(function () {
      var q = ${JSON.stringify(query)};
      if (!q) return;
      var target = new URLSearchParams(q);
      var current = new URLSearchParams(window.location.search);
      target.forEach(function (value, key) {
        current.set(key, value);
      });
      var next = current.toString();
      history.replaceState(null, "", next ? "?" + next : "");
    })();`;
}

/** Regression guard for panel routing bugs (script-src query vs page location). */
export function assertWebviewHtmlRoutesPanel(
  html: string,
  panel: PanelKind
): void {
  if (/<script[^>]+src="[^"]*\?[^"]*panel=/.test(html)) {
    throw new Error(
      "webview HTML routes panel via script src query only; use location bootstrap"
    );
  }
  if (!html.includes("history.replaceState")) {
    throw new Error(
      "webview HTML missing location bootstrap (history.replaceState)"
    );
  }
  if (!html.includes(`panel=${panel}`)) {
    throw new Error(`webview HTML missing panel=${panel} in bootstrap query`);
  }
}
