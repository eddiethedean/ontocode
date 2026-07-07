import { getVsCodeApi } from "../vscodeApi";
import type { HostNotificationLevel, HostTheme, WorkspaceHost } from "./types";

function detectTheme(): HostTheme {
  const body = document.body;
  if (body.classList.contains("vscode-dark") || body.classList.contains("vscode-high-contrast")) {
    return "dark";
  }
  if (body.classList.contains("vscode-light")) {
    return "light";
  }
  return "unknown";
}

/** VS Code webview implementation of WorkspaceHost. */
export function createVscodeHost(): WorkspaceHost {
  const api = getVsCodeApi();
  const listeners = new Set<(message: unknown) => void>();

  if (typeof window !== "undefined") {
    window.addEventListener("message", (event: MessageEvent) => {
      for (const listener of listeners) {
        listener(event.data);
      }
    });
  }

  return {
    postToCore(message: unknown): void {
      api.postMessage(message);
    },
    getTheme(): HostTheme {
      return detectTheme();
    },
    showNotification(message: string, level: HostNotificationLevel = "info"): void {
      api.postMessage({ type: "showNotification", message, level });
    },
    onMessage(handler: (message: unknown) => void): () => void {
      listeners.add(handler);
      return () => listeners.delete(handler);
    },
  };
}
