/** Host adapter — bridges OntoUI to VS Code (or future OntoStudio). */

export type HostTheme = "light" | "dark" | "unknown";

export type HostNotificationLevel = "info" | "warning" | "error";

export interface WorkspaceHost {
  /** Post a message to the extension host / LSP bridge. */
  postToCore(message: unknown): void;
  /** Best-effort theme hint from the host environment. */
  getTheme(): HostTheme;
  /** Surface a non-blocking notification (host may no-op in tests). */
  showNotification(message: string, level?: HostNotificationLevel): void;
  /** Subscribe to messages from the extension host. Returns unsubscribe. */
  onMessage(handler: (message: unknown) => void): () => void;
}
