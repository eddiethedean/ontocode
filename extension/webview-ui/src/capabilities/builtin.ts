import { register } from "./registry";
import type { DiagnosticsProvider, QueryProvider, ReasoningProvider } from "./types";

/** Register built-in OntoCore capability stubs (delegate to extension host / LSP). */
export function registerBuiltinProviders(): void {
  const ontocore: ReasoningProvider & QueryProvider & DiagnosticsProvider = {
    id: "ontocore",
    version: "0.14.0",
    capabilities: ["reasoning", "query", "diagnostics"],
    async classify() {
      return { delegated: true, provider: "ontocore" };
    },
    async runSql() {
      return { delegated: true, provider: "ontocore" };
    },
    async validate() {
      return { delegated: true, provider: "ontocore" };
    },
  };
  register(ontocore);
}
