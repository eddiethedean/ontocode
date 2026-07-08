import { describe, it, beforeEach } from "node:test";
import assert from "node:assert/strict";
import { focusRelay } from "./focusRelay";
import type { PanelHost } from "../webviews/panelHost";

function mockHost(): PanelHost & { messages: unknown[] } {
  const messages: unknown[] = [];
  return {
    messages,
    isDisposed: false,
    postMessage(msg: unknown) {
      messages.push(msg);
    },
  } as PanelHost & { messages: unknown[] };
}

describe("focusRelay", () => {
  beforeEach(() => {
    focusRelay.resetForTests();
  });

  it("broadcasts focusState to registered hosts", () => {
    const host = mockHost();
    focusRelay.registerHost(host);
    const iri = "http://example.org/test#Person";
    focusRelay.setEntityFocus(iri, "explorer");
    assert.equal(focusRelay.getFocus()?.id, iri);
    assert.ok(
      host.messages.some(
        (m) =>
          typeof m === "object" &&
          m !== null &&
          (m as { type?: string }).type === "focusState" &&
          (m as { focus?: { id?: string } }).focus?.id === iri
      )
    );
  });

  it("syncHost replays current focus on ready", () => {
    const host = mockHost();
    focusRelay.setEntityFocus("http://example.org/test#Org", "inspector");
    focusRelay.registerHost(host);
    host.messages.length = 0;
    focusRelay.syncHost(host);
    assert.equal(host.messages.length, 1);
    assert.deepEqual((host.messages[0] as { type: string }).type, "focusState");
  });

  it("broadcasts reasoningState to registered hosts", () => {
    const host = mockHost();
    focusRelay.registerHost(host);
    focusRelay.setReasoningState({
      unsatisfiable: ["http://example.org#Bad"],
      profile: "el",
      hierarchyMode: "inferred",
      lastRunAt: 1,
    });
    assert.ok(
      host.messages.some(
        (m) =>
          typeof m === "object" &&
          m !== null &&
          (m as { type?: string }).type === "reasoningState"
      )
    );
  });

  it("delivers focus updates to multiple hosts", () => {
    const hostA = mockHost();
    const hostB = mockHost();
    focusRelay.registerHost(hostA);
    focusRelay.registerHost(hostB);
    focusRelay.setEntityFocus("http://example.org/test#Multi", "graph");
    assert.equal(hostA.messages.length, 1);
    assert.equal(hostB.messages.length, 1);
  });
});
