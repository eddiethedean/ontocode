import { useEffect, useId, useState, type ReactNode } from "react";

export type LivePoliteness = "polite" | "assertive";

/**
 * Visually subtle / screen-reader region that announces `message` when it changes.
 * Prefer mounting once per panel and updating the message string.
 */
export function LiveAnnouncer({
  message,
  politeness = "polite",
  className = "oc-sr-only",
}: {
  message: string;
  politeness?: LivePoliteness;
  className?: string;
}): JSX.Element {
  const id = useId();
  // Re-announce identical text by toggling a tick character after clear.
  const [text, setText] = useState(message);
  useEffect(() => {
    if (!message) {
      setText("");
      return;
    }
    setText("");
    const t = window.setTimeout(() => setText(message), 30);
    return () => window.clearTimeout(t);
  }, [message]);

  return (
    <div
      id={id}
      className={className}
      role="status"
      aria-live={politeness}
      aria-atomic="true"
    >
      {text}
    </div>
  );
}

/** Landmark wrapper for panel main content. */
export function PanelMain({
  label,
  children,
  className = "",
}: {
  label: string;
  children: ReactNode;
  className?: string;
}): JSX.Element {
  return (
    <main className={className} aria-label={label}>
      {children}
    </main>
  );
}
