import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
  AVAILABLE_PROFILES,
  formatInferenceLine,
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
});
