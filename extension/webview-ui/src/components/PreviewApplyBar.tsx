import { ButtonBar, Callout } from "./ui";

export function PreviewApplyBar({
  preview,
  onPreview,
  onApply,
  applyLabel = "Apply",
  previewLabel = "Preview",
  disabled = false,
}: {
  preview: string;
  onPreview: () => void;
  onApply: () => void;
  applyLabel?: string;
  previewLabel?: string;
  disabled?: boolean;
}): JSX.Element {
  return (
    <>
      <ButtonBar>
        <button type="button" className="secondary" disabled={disabled} onClick={onPreview}>
          {previewLabel}
        </button>
        <button type="button" disabled={disabled} onClick={onApply}>
          {applyLabel}
        </button>
      </ButtonBar>
    </>
  );
}
