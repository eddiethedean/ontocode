import assert from "node:assert/strict";
import { describe, it } from "node:test";
import {
  isExplanationStale,
  resolveExplanationProfile,
  stepsAsText,
} from "./explanationPanelLogic";

describe("explanationPanelLogic", () => {
  it("prefers explicit profile over last run and settings", () => {
    assert.equal(
      resolveExplanationProfile({
        explicit: "dl",
        lastRunProfile: "rl",
        settingsDefault: "el",
      }),
      "dl"
    );
  });

  it("uses last-run profile before settings default", () => {
    assert.equal(
      resolveExplanationProfile({
        lastRunProfile: "rl",
        settingsDefault: "el",
      }),
      "rl"
    );
  });

  it("falls back to settings then el", () => {
    assert.equal(
      resolveExplanationProfile({ settingsDefault: "rdfs" }),
      "rdfs"
    );
    assert.equal(resolveExplanationProfile({}), "el");
  });

  it("does not mark matching fingerprints as stale", () => {
    assert.equal(
      isExplanationStale({
        shownContentHash: "abc",
        shownIndexedAt: 1,
        currentContentHash: "abc",
        currentIndexedAt: 1,
      }),
      false
    );
  });

  it("marks stale only when catalog fingerprint diverges from shown explanation", () => {
    assert.equal(
      isExplanationStale({
        shownContentHash: "old",
        shownIndexedAt: 1,
        currentContentHash: "new",
        currentIndexedAt: 2,
      }),
      true
    );
  });

  it("formats steps as text", () => {
    assert.equal(
      stepsAsText([{ index: 1, display: "A SubClassOf B", rule: "r" }]),
      "1. A SubClassOf B"
    );
  });
});
