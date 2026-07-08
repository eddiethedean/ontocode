import type { ReactNode, ButtonHTMLAttributes, InputHTMLAttributes } from "react";

export function Button({
  children,
  variant = "primary",
  className = "",
  ...rest
}: ButtonHTMLAttributes<HTMLButtonElement> & {
  children: ReactNode;
  variant?: "primary" | "secondary" | "ghost";
}): JSX.Element {
  return (
    <button
      type="button"
      className={`oc-btn oc-btn--${variant} ${className}`.trim()}
      {...rest}
    >
      {children}
    </button>
  );
}

export function Input({
  className = "",
  ...rest
}: InputHTMLAttributes<HTMLInputElement>): JSX.Element {
  return <input className={`oc-input ${className}`.trim()} {...rest} />;
}

export function Card({
  children,
  className = "",
  variant = "default",
}: {
  children: ReactNode;
  className?: string;
  variant?: "default" | "elevated" | "inset" | "danger";
}): JSX.Element {
  return (
    <div className={`oc-card oc-card--${variant} ${className}`.trim()}>
      {children}
    </div>
  );
}
