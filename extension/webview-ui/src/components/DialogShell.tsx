import { useEffect, useId, useRef, type ReactNode } from "react";
import { installFocusTrap } from "../a11y";
import { Button, ButtonBar, Callout, Panel, PanelHeader } from "./ui";

export interface DialogShellProps {
  title: string;
  children: ReactNode;
  primaryLabel?: string;
  cancelLabel?: string;
  validationMessage?: string;
  primaryDisabled?: boolean;
  onPrimary: () => void;
  onCancel: () => void;
}

export function DialogShell({
  title,
  children,
  primaryLabel = "OK",
  cancelLabel = "Cancel",
  validationMessage,
  primaryDisabled = false,
  onPrimary,
  onCancel,
}: DialogShellProps): JSX.Element {
  const titleId = useId();
  const descId = useId();
  const dialogRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const node = dialogRef.current;
    if (!node) {
      return;
    }
    return installFocusTrap(node);
  }, []);

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent): void => {
      if (event.key === "Escape") {
        event.preventDefault();
        onCancel();
      } else if (
        event.key === "Enter" &&
        !event.shiftKey &&
        !(event.target instanceof HTMLTextAreaElement) &&
        !primaryDisabled
      ) {
        event.preventDefault();
        onPrimary();
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [onCancel, onPrimary, primaryDisabled]);

  return (
    <Panel>
      <div
        ref={dialogRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby={titleId}
        aria-describedby={validationMessage ? descId : undefined}
        tabIndex={-1}
        className="oc-dialog"
      >
        <PanelHeader title={title} titleId={titleId} />
        <div className="oc-dialog-body">{children}</div>
        <div id={descId} aria-live="assertive">
          {validationMessage ? (
            <Callout variant="error">{validationMessage}</Callout>
          ) : null}
        </div>
        <ButtonBar>
          <Button variant="secondary" onClick={onCancel}>
            {cancelLabel}
          </Button>
          <Button onClick={onPrimary} disabled={primaryDisabled}>
            {primaryLabel}
          </Button>
        </ButtonBar>
      </div>
    </Panel>
  );
}
