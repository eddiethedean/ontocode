import { describe, it, expect } from "vitest";
import { designTokenCssVars, designTokenStyleBlock } from "./cssVars";

describe("design tokens", () => {
  it("defines spacing and motion CSS variables", () => {
    expect(designTokenCssVars["--oc-space-4"]).toBe("16px");
    expect(designTokenCssVars["--oc-motion-fast"]).toBe("150ms");
    expect(designTokenCssVars["--oc-radius-sm"]).toBe("4px");
    expect(designTokenCssVars["--oc-space-8"]).toBe("64px");
  });

  it("generates a :root style block", () => {
    const block = designTokenStyleBlock();
    expect(block).toContain(":root");
    expect(block).toContain("--oc-space-1: 4px");
  });
});
