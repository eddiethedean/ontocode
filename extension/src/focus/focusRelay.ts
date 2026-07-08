import type { PanelHost } from "../webviews/panelHost";
import type { CurrentFocus, ReasoningStatePayload } from "./types";

type FocusListener = (focus: CurrentFocus | null) => void;

class FocusRelayService {
  private focus: CurrentFocus | null = null;
  private reasoning: ReasoningStatePayload | null = null;
  private hosts = new Set<PanelHost>();
  private listeners = new Set<FocusListener>();

  getFocus(): CurrentFocus | null {
    return this.focus;
  }

  getReasoning(): ReasoningStatePayload | null {
    return this.reasoning;
  }

  registerHost(host: PanelHost): () => void {
    this.hosts.add(host);
    return () => {
      this.hosts.delete(host);
    };
  }

  subscribe(listener: FocusListener): () => void {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  setFocus(
    input: Omit<CurrentFocus, "timestamp"> & { timestamp?: number },
    options?: { broadcast?: boolean }
  ): CurrentFocus {
    const focus: CurrentFocus = {
      ...input,
      timestamp: input.timestamp ?? Date.now(),
    };
    this.focus = focus;
    for (const listener of this.listeners) {
      listener(focus);
    }
    if (options?.broadcast !== false) {
      this.broadcastFocus();
    }
    return focus;
  }

  setEntityFocus(iri: string, source: string): CurrentFocus {
    return this.setFocus({ kind: "entity", id: iri, source });
  }

  setReasoningState(state: ReasoningStatePayload): void {
    this.reasoning = state;
    this.broadcastReasoning();
  }

  markReasoningDirty(): void {
    if (!this.reasoning) {
      return;
    }
    this.reasoning = { ...this.reasoning, dirty: true };
    this.broadcastReasoning();
  }

  setReasoningRunning(running: boolean): void {
    if (!this.reasoning) {
      return;
    }
    this.reasoning = { ...this.reasoning, running };
    this.broadcastReasoning();
  }

  /** Push current focus + reasoning to a single host (e.g. on webview ready). */
  syncHost(host: PanelHost): void {
    if (this.focus) {
      host.postMessage({ type: "focusState", focus: this.focus });
    }
    if (this.reasoning) {
      host.postMessage({ type: "reasoningState", reasoning: this.reasoning });
    }
  }

  private broadcastFocus(): void {
    if (!this.focus) {
      return;
    }
    const message = { type: "focusState" as const, focus: this.focus };
    for (const host of this.hosts) {
      if (!host.isDisposed) {
        host.postMessage(message);
      }
    }
  }

  private broadcastReasoning(): void {
    if (!this.reasoning) {
      return;
    }
    const message = { type: "reasoningState" as const, reasoning: this.reasoning };
    for (const host of this.hosts) {
      if (!host.isDisposed) {
        host.postMessage(message);
      }
    }
  }

  /** Tests only. */
  resetForTests(): void {
    this.focus = null;
    this.reasoning = null;
    this.hosts.clear();
    this.listeners.clear();
  }
}

export const focusRelay = new FocusRelayService();
