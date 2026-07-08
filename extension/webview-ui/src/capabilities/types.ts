/** Capability Provider types for OntoUI plugin platform (v0.14). */

export type CapabilityKind =
  | "reasoning"
  | "query"
  | "refactoring"
  | "diagnostics"
  | "import_export"
  | "documentation";

export interface CapabilityProvider {
  id: string;
  version: string;
  capabilities: CapabilityKind[];
}

export interface ReasoningProvider extends CapabilityProvider {
  capabilities: CapabilityKind[];
  classify?(profile: string): Promise<unknown>;
  explain?(entityIri: string): Promise<unknown>;
}

export interface QueryProvider extends CapabilityProvider {
  capabilities: CapabilityKind[];
  runSql?(sql: string): Promise<unknown>;
  runSparql?(query: string): Promise<unknown>;
}

export interface DiagnosticsProvider extends CapabilityProvider {
  capabilities: CapabilityKind[];
  validate?(): Promise<unknown>;
}
