import type { PanelHost } from "../webviews/panelHost";
import type { WorkspaceEvent, WorkspaceEventType } from "./types";

type WorkspaceListener = (event: WorkspaceEvent) => void;

export class WorkspaceEventBus {
  private listeners = new Set<WorkspaceListener>();
  private hosts = new Set<PanelHost>();

  subscribe(listener: WorkspaceListener): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  registerHost(host: PanelHost): () => void {
    this.hosts.add(host);
    return () => this.hosts.delete(host);
  }

  publish<T>(type: WorkspaceEventType, payload?: T): WorkspaceEvent<T> {
    const event: WorkspaceEvent<T> = {
      type,
      payload,
      timestamp: Date.now(),
    };
    for (const listener of this.listeners) {
      listener(event);
    }
    const message = {
      type: "workspaceEvent" as const,
      event: {
        type: event.type,
        payload: event.payload,
        timestamp: event.timestamp,
      },
    };
    for (const host of this.hosts) {
      if (!host.isDisposed) {
        host.postMessage(message);
      }
    }
    return event;
  }

  resetForTests(): void {
    this.listeners.clear();
    this.hosts.clear();
  }
}

export const workspaceEventBus = new WorkspaceEventBus();
