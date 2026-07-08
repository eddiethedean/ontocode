import type { ReactNode } from "react";
export { Button, Card, Input } from "./primitives";
import { Card } from "./primitives";

export function Panel({
  children,
  className = "",
  wide = false,
}: {
  children: ReactNode;
  className?: string;
  wide?: boolean;
}): JSX.Element {
  return (
    <div className={`oc-panel${wide ? " oc-panel--wide" : ""} ${className}`.trim()}>
      {children}
    </div>
  );
}

export function PanelHeader({
  title,
  subtitle,
  badges,
}: {
  title: string;
  subtitle?: ReactNode;
  badges?: ReactNode;
}): JSX.Element {
  return (
    <header className="oc-panel-header">
      <div className="oc-panel-header-accent" aria-hidden="true" />
      <div className="oc-panel-header-inner">
        <div className="oc-panel-header-text">
          <h1 className="oc-title">{title}</h1>
          {subtitle ? <p className="oc-subtitle">{subtitle}</p> : null}
        </div>
        {badges ? <div className="oc-badge-row">{badges}</div> : null}
      </div>
    </header>
  );
}

export function Section({
  title,
  children,
  action,
  className = "",
  card = false,
}: {
  title?: string;
  children: ReactNode;
  action?: ReactNode;
  className?: string;
  card?: boolean;
}): JSX.Element {
  const body = (
    <>
      {title ? (
        <div className="oc-section-head">
          <h2 className="oc-section-title">{title}</h2>
          {action}
        </div>
      ) : null}
      <div className="oc-section-body">{children}</div>
    </>
  );

  if (card) {
    return (
      <section className={`oc-section ${className}`.trim()}>
        <Card>{body}</Card>
      </section>
    );
  }

  return <section className={`oc-section ${className}`.trim()}>{body}</section>;
}

export function Badge({
  children,
  variant = "default",
}: {
  children: ReactNode;
  variant?: "default" | "kind" | "danger" | "warning" | "success" | "accent";
}): JSX.Element {
  return <span className={`oc-badge oc-badge--${variant}`}>{children}</span>;
}

export function ButtonBar({ children }: { children: ReactNode }): JSX.Element {
  return <div className="oc-button-bar">{children}</div>;
}

export function StickyActions({ children }: { children: ReactNode }): JSX.Element {
  return (
    <div className="oc-sticky-actions">
      <div className="oc-sticky-actions-inner">{children}</div>
    </div>
  );
}

export function EmptyState({
  title,
  detail,
  icon,
}: {
  title: string;
  detail?: string;
  icon?: ReactNode;
}): JSX.Element {
  return (
    <div className="oc-empty">
      <div className="oc-empty-icon" aria-hidden="true">
        {icon ?? <span className="oc-empty-icon-default" />}
      </div>
      <p className="oc-empty-title">{title}</p>
      {detail ? <p className="oc-empty-detail">{detail}</p> : null}
    </div>
  );
}

export function LoadingState({ label = "Loading…" }: { label?: string }): JSX.Element {
  return (
    <div className="oc-loading" role="status" aria-live="polite">
      <span className="oc-spinner" aria-hidden="true" />
      <span className="oc-loading-label">{label}</span>
    </div>
  );
}

export function Callout({
  children,
  variant = "info",
}: {
  children: ReactNode;
  variant?: "info" | "error" | "warning" | "success";
}): JSX.Element {
  return <div className={`oc-callout oc-callout--${variant}`}>{children}</div>;
}

export function CodeBlock({
  children,
  mono = true,
}: {
  children: ReactNode;
  mono?: boolean;
}): JSX.Element {
  return (
    <pre className={`oc-code-block${mono ? " oc-code-block--mono" : ""}`}>
      {children}
    </pre>
  );
}

export function CodeEditor(
  props: React.TextareaHTMLAttributes<HTMLTextAreaElement> & { label?: string }
): JSX.Element {
  const { label, className = "", ...rest } = props;
  return (
    <div className="oc-code-editor">
      {label ? <div className="oc-code-editor-bar">{label}</div> : null}
      <textarea className={`oc-textarea oc-code-editor-input ${className}`.trim()} {...rest} />
    </div>
  );
}

export function InlineCode({ children }: { children: ReactNode }): JSX.Element {
  return <code className="oc-inline-code">{children}</code>;
}

export function FormField({
  label,
  children,
  hint,
  inline = false,
}: {
  label: string;
  children: ReactNode;
  hint?: string;
  inline?: boolean;
}): JSX.Element {
  return (
    <label className={`oc-field${inline ? " oc-field--inline" : ""}`}>
      <span className="oc-field-label">{label}</span>
      {children}
      {hint ? <span className="oc-field-hint">{hint}</span> : null}
    </label>
  );
}

export function TextArea(props: React.TextareaHTMLAttributes<HTMLTextAreaElement>): JSX.Element {
  return <textarea className="oc-textarea" {...props} />;
}

export function Select(props: React.SelectHTMLAttributes<HTMLSelectElement>): JSX.Element {
  return <select className="oc-select" {...props} />;
}

export function CheckboxRow({
  label,
  checked,
  onChange,
}: {
  label: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
}): JSX.Element {
  return (
    <label className="oc-checkbox-row">
      <input
        type="checkbox"
        checked={checked}
        onChange={(e) => onChange(e.target.checked)}
      />
      <span className="oc-checkbox-row-label">{label}</span>
    </label>
  );
}

export function RangeField({
  label,
  value,
  min,
  max,
  onChange,
}: {
  label: string;
  value: number;
  min: number;
  max: number;
  onChange: (value: number) => void;
}): JSX.Element {
  return (
    <div className="oc-range-field">
      <div className="oc-range-field-head">
        <span className="oc-range-field-label">{label}</span>
        <span className="oc-range-field-value">{value}</span>
      </div>
      <input
        type="range"
        min={min}
        max={max}
        value={value}
        onChange={(e) => onChange(Number(e.target.value))}
      />
    </div>
  );
}

export function StatGrid({
  items,
}: {
  items: Array<{ label: string; value: number; variant?: "default" | "danger" | "accent" }>;
}): JSX.Element {
  return (
    <div className="oc-stat-grid">
      {items.map((item) => (
        <div
          key={item.label}
          className={`oc-stat${item.variant ? ` oc-stat--${item.variant}` : ""}`}
        >
          <span className="oc-stat-value">{item.value}</span>
          <span className="oc-stat-label">{item.label}</span>
        </div>
      ))}
    </div>
  );
}

export function IriList({
  items,
  onSelect,
}: {
  items: string[];
  onSelect?: (iri: string) => void;
}): JSX.Element {
  if (items.length === 0) {
    return <p className="oc-muted">None</p>;
  }
  return (
    <ul className="oc-iri-list">
      {items.map((iri) => (
        <li key={iri} className="oc-iri-item">
          {onSelect ? (
            <button type="button" className="oc-iri-btn" onClick={() => onSelect(iri)}>
              <InlineCode>{shortLabel(iri)}</InlineCode>
              <span className="oc-iri-full">{iri}</span>
            </button>
          ) : (
            <div className="oc-iri-static">
              <InlineCode>{shortLabel(iri)}</InlineCode>
              <span className="oc-iri-full">{iri}</span>
            </div>
          )}
        </li>
      ))}
    </ul>
  );
}

export function DiffColumns({
  before,
  after,
}: {
  before: string;
  after: string;
}): JSX.Element {
  return (
    <div className="oc-diff-grid">
      <div className="oc-diff-pane">
        <div className="oc-diff-pane-head">Before</div>
        <CodeBlock>{before}</CodeBlock>
      </div>
      <div className="oc-diff-pane">
        <div className="oc-diff-pane-head oc-diff-pane-head--after">After</div>
        <CodeBlock>{after}</CodeBlock>
      </div>
    </div>
  );
}

export function Toolbar({ children }: { children: ReactNode }): JSX.Element {
  return <div className="oc-toolbar">{children}</div>;
}

export function ToolbarGroup({ children }: { children: ReactNode }): JSX.Element {
  return <div className="oc-toolbar-group">{children}</div>;
}

export function ChangeList({
  items,
}: {
  items: Array<{ key: string; primary: ReactNode; secondary?: ReactNode; badge?: string }>;
}): JSX.Element {
  if (items.length === 0) {
    return null;
  }
  return (
    <ul className="oc-change-list">
      {items.map((item) => (
        <li key={item.key} className="oc-change-item">
          {item.badge ? <Badge variant="accent">{item.badge}</Badge> : null}
          <div className="oc-change-text">
            <span className="oc-change-primary">{item.primary}</span>
            {item.secondary ? (
              <span className="oc-change-secondary">{item.secondary}</span>
            ) : null}
          </div>
        </li>
      ))}
    </ul>
  );
}

export function Divider(): JSX.Element {
  return <hr className="oc-divider" />;
}

export function shortLabel(iri: string): string {
  const hash = iri.lastIndexOf("#");
  const slash = iri.lastIndexOf("/");
  const idx = Math.max(hash, slash);
  return idx >= 0 ? iri.slice(idx + 1) : iri;
}

export function kindLabel(kind: string): string {
  return kind.replace(/_/g, " ");
}
