/** Focusable selector used for modal traps inside webviews. */
export const FOCUSABLE_SELECTOR = [
  "a[href]",
  "button:not([disabled])",
  "textarea:not([disabled])",
  "input:not([disabled])",
  "select:not([disabled])",
  "[tabindex]:not([tabindex='-1'])",
].join(", ");

export function getFocusableElements(container: HTMLElement): HTMLElement[] {
  return Array.from(container.querySelectorAll<HTMLElement>(FOCUSABLE_SELECTOR)).filter(
    (el) => !el.hasAttribute("disabled") && el.getAttribute("aria-hidden") !== "true"
  );
}

/**
 * Trap Tab / Shift+Tab inside `container`. Returns a cleanup function.
 * Optionally focuses the first focusable element (or `container` if focusable).
 */
export function installFocusTrap(
  container: HTMLElement,
  options?: { initialFocus?: HTMLElement | null; restoreFocus?: HTMLElement | null }
): () => void {
  const previouslyFocused =
    options?.restoreFocus ??
    (document.activeElement instanceof HTMLElement ? document.activeElement : null);

  const focusInitial = (): void => {
    const target =
      options?.initialFocus ??
      getFocusableElements(container)[0] ??
      (container.tabIndex >= 0 ? container : null);
    target?.focus();
  };

  // Defer so dialog content is mounted.
  const raf = window.requestAnimationFrame(focusInitial);

  const onKeyDown = (event: KeyboardEvent): void => {
    if (event.key !== "Tab") {
      return;
    }
    const focusable = getFocusableElements(container);
    if (focusable.length === 0) {
      event.preventDefault();
      container.focus();
      return;
    }
    const first = focusable[0];
    const last = focusable[focusable.length - 1];
    const active = document.activeElement;
    if (event.shiftKey) {
      if (active === first || !container.contains(active)) {
        event.preventDefault();
        last.focus();
      }
    } else if (active === last || !container.contains(active)) {
      event.preventDefault();
      first.focus();
    }
  };

  container.addEventListener("keydown", onKeyDown);

  return () => {
    window.cancelAnimationFrame(raf);
    container.removeEventListener("keydown", onKeyDown);
    if (previouslyFocused && document.contains(previouslyFocused)) {
      previouslyFocused.focus();
    }
  };
}
