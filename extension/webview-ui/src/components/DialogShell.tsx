import { useEffect, type ReactNode } from "react";
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
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent): void => {
      if (event.key === "Escape") {
        event.preventDefault();
        onCancel();
      } else if (
        event.key === "Enter" &&
        !event.shiftKey &&
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
      <PanelHeader title={title} />
      <div role="dialog" aria-modal="true" aria-labelledby="dialog-title">
        <div id="dialog-title" className="oc-dialog-body">
          {children}
        </div>
        <div aria-live="polite">
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
