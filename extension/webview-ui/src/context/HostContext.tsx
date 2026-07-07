import { createContext, useContext, useMemo, type ReactNode } from "react";
import { createVscodeHost, type WorkspaceHost } from "../host";

const HostContext = createContext<WorkspaceHost | null>(null);

export function HostProvider({
  host,
  children,
}: {
  host?: WorkspaceHost;
  children: ReactNode;
}): JSX.Element {
  const value = useMemo(() => host ?? createVscodeHost(), [host]);
  return <HostContext.Provider value={value}>{children}</HostContext.Provider>;
}

export function useWorkspaceHost(): WorkspaceHost {
  const host = useContext(HostContext);
  if (!host) {
    throw new Error("useWorkspaceHost must be used within HostProvider");
  }
  return host;
}
