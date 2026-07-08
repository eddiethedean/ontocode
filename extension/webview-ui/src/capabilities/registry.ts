import type { CapabilityKind, CapabilityProvider } from "./types";

const providers = new Map<string, CapabilityProvider>();

export function register(provider: CapabilityProvider): void {
  providers.set(provider.id, provider);
}

export function list(kind?: CapabilityKind): CapabilityProvider[] {
  const all = Array.from(providers.values());
  if (!kind) {
    return all;
  }
  return all.filter((p) => p.capabilities.includes(kind));
}

export function get<T extends CapabilityProvider>(
  id: string
): T | undefined {
  return providers.get(id) as T | undefined;
}

export function clearRegistryForTests(): void {
  providers.clear();
}
