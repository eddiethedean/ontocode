/** Ignore stale async reveal calls; accept newer navigation request IDs. */
export function acceptInspectorRevealRequest(
  activeRequestId: number,
  requestId: number | undefined
): boolean {
  if (requestId === undefined) {
    return true;
  }
  return activeRequestId === 0 || requestId >= activeRequestId;
}
