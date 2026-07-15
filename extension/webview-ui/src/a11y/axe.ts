import axe from "axe-core";

/** Run axe and fail only on serious/critical impact (jsdom contrast is noisy). */
export async function expectNoSeriousA11yViolations(
  container: HTMLElement
): Promise<void> {
  const results = await axe.run(container, {
    rules: {
      // VS Code theme CSS vars are unresolved in jsdom — skip color contrast noise.
      "color-contrast": { enabled: false },
    },
  });
  const blocking = results.violations.filter(
    (v) => v.impact === "serious" || v.impact === "critical"
  );
  if (blocking.length > 0) {
    const detail = blocking
      .map(
        (v) =>
          `${v.id} (${v.impact}): ${v.nodes
            .map((n) => n.target.join(" "))
            .join("; ")} — ${v.help}`
      )
      .join("\n");
    throw new Error(`Accessibility violations:\n${detail}`);
  }
}
