import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
  AVAILABLE_PROFILES,
  captureReasoningPreRun,
  formatInferenceLine,
  reasoningStateForRunCancel,
  reasoningStateForRunStart,
  reasoningStateForRunSuccess,
  shortIri,
  summarizeResult,
} from "./reasonerPanelLogic";
import { RunReasonerResult } from "../lsp/protocol";

describe("reasonerPanelLogic", () => {
  it("enables dl and auto profiles", () => {
    const dl = AVAILABLE_PROFILES.find((p) => p.id === "dl");
    assert.equal(dl?.enabled, true);
    const auto = AVAILABLE_PROFILES.find((p) => p.id === "auto");
    assert.equal(auto?.enabled, true);
  });

  it("shortens IRIs for display", () => {
    assert.equal(shortIri("http://ex.org/onto#Dog"), "Dog");
  });

  it("formats inference lines", () => {
    assert.equal(
      formatInferenceLine("http://ex#A", "http://ex#B"),
      "A SubClassOf B"
    );
  });

  it("summarizes consistent results", () => {
    const result: RunReasonerResult = {
      profile_used: "el",
      consistent: true,
      unsatisfiable: [],
      inferred_edge_count: 1,
      new_inferences: [],
      warnings: [],
      duration_ms: 12,
      snapshot: {
        profile_used: "el",
        consistent: true,
        unsatisfiable: [],
        inferred: { edges: [], unsatisfiable: [], combined: { edges: [], parents: {}, children: {} } },
        new_inferences: [],
        warnings: [],
        duration_ms: 12,
        classified_at: 1,
      },
    };
    assert.match(summarizeResult(result), /Completed/);
  });

  it("run start preserves pre-run lastRunAt and dirty (#221)", () => {
    const preRun = captureReasoningPreRun({
      profile: "el",
      unsatisfiable: [],
      lastRunAt: 42,
      dirty: true,
      running: false,
    });
    const started = reasoningStateForRunStart("rl", preRun);
    assert.equal(started.lastRunAt, 42);
    assert.equal(started.dirty, true);
    assert.equal(started.running, true);
    assert.equal(started.profile, "rl");
  });

  it("cancel restores pre-run snapshot (#221)", () => {
    const preRun = captureReasoningPreRun({
      profile: "el",
      unsatisfiable: ["http://ex#Bad"],
      lastRunAt: 99,
      dirty: true,
      running: false,
    });
    const cancelled = reasoningStateForRunCancel(preRun);
    assert.equal(cancelled.lastRunAt, 99);
    assert.equal(cancelled.dirty, true);
    assert.equal(cancelled.running, false);
  });

  it("success advances lastRunAt and clears dirty (#221)", () => {
    const preRun = captureReasoningPreRun({
      profile: "el",
      unsatisfiable: [],
      lastRunAt: 10,
      dirty: true,
      running: false,
      hierarchyMode: "inferred",
    });
    const done = reasoningStateForRunSuccess("el", ["http://ex#Bad"], preRun);
    assert.ok(done.lastRunAt > 10);
    assert.equal(done.dirty, false);
    assert.equal(done.running, false);
    assert.equal(done.hierarchyMode, "inferred");
  });
});
