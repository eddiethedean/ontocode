import { focusRelay } from "../focus/focusRelay";
import { workspaceEventBus } from "./eventBus";
import type { NavigationEntry } from "./types";

const MAX_NAV = 50;

export class NavigationManager {
  private stack: NavigationEntry[] = [];
  private index = -1;
  private context: { workspaceState: { get<T>(key: string): T | undefined; update(key: string, value: unknown): Thenable<void> } } | undefined;

  bindContext(context: {
    workspaceState: {
      get<T>(key: string): T | undefined;
      update(key: string, value: unknown): Thenable<void>;
    };
  }): void {
    this.context = context;
    const saved = context.workspaceState.get<{
      stack: NavigationEntry[];
      index: number;
    }>("ontocode.navigation");
    if (saved?.stack?.length) {
      this.stack = saved.stack.slice(-MAX_NAV);
      this.index = Math.min(saved.index, this.stack.length - 1);
    }
  }

  getStack(): NavigationEntry[] {
    return [...this.stack];
  }

  getIndex(): number {
    return this.index;
  }

  canGoBack(): boolean {
    return this.index > 0;
  }

  canGoForward(): boolean {
    return this.index >= 0 && this.index < this.stack.length - 1;
  }

  push(entry: NavigationEntry): NavigationEntry {
    if (this.index < this.stack.length - 1) {
      this.stack = this.stack.slice(0, this.index + 1);
    }
    const last = this.stack[this.stack.length - 1];
    if (last && last.kind === entry.kind && last.id === entry.id) {
      return last;
    }
    this.stack.push(entry);
    if (this.stack.length > MAX_NAV) {
      this.stack.shift();
    }
    this.index = this.stack.length - 1;
    void this.persist();
    workspaceEventBus.publish("NavigationChanged", {
      stack: this.getStack(),
      index: this.index,
    });
    return entry;
  }

  back(): NavigationEntry | undefined {
    if (!this.canGoBack()) {
      return undefined;
    }
    this.index -= 1;
    const entry = this.stack[this.index];
    void this.persist();
    this.applyEntry(entry);
    workspaceEventBus.publish("NavigationChanged", {
      stack: this.getStack(),
      index: this.index,
    });
    return entry;
  }

  forward(): NavigationEntry | undefined {
    if (!this.canGoForward()) {
      return undefined;
    }
    this.index += 1;
    const entry = this.stack[this.index];
    void this.persist();
    this.applyEntry(entry);
    workspaceEventBus.publish("NavigationChanged", {
      stack: this.getStack(),
      index: this.index,
    });
    return entry;
  }

  trackEntityFocus(iri: string, source: string, label?: string): void {
    this.push({ kind: "entity", id: iri, source, label });
  }

  hydrate(stack: NavigationEntry[], index: number): void {
    this.stack = stack.slice(-MAX_NAV);
    this.index = Math.min(index, this.stack.length - 1);
    void this.persist();
  }

  resetForTests(): void {
    this.stack = [];
    this.index = -1;
    this.context = undefined;
  }

  private applyEntry(entry: NavigationEntry | undefined): void {
    if (!entry) {
      return;
    }
    if (entry.kind === "entity") {
      focusRelay.setEntityFocus(entry.id, entry.source);
    }
  }

  private async persist(): Promise<void> {
    if (!this.context) {
      return;
    }
    await this.context.workspaceState.update("ontocode.navigation", {
      stack: this.stack,
      index: this.index,
    });
  }
}

export const navigationManager = new NavigationManager();
