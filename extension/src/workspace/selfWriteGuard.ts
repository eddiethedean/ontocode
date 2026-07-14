import { normalizeFsPath } from "../utils/pathUnder";

/** Default window where FS watcher events for OntoCode's own writes are ignored (#293). */
export const SELF_WRITE_TTL_MS = 2500;

const selfWriteUntil = new Map<string, number>();

/** Mark a filesystem path as written by OntoCode (patch/save) for a short TTL. */
export function noteSelfWrite(
  fsPath: string,
  ttlMs: number = SELF_WRITE_TTL_MS,
  now: number = Date.now()
): void {
  const key = normalizeFsPath(fsPath);
  const until = now + ttlMs;
  const previous = selfWriteUntil.get(key) ?? 0;
  if (until > previous) {
    selfWriteUntil.set(key, until);
  }
}

/** True when `fsPath` was recently written by OntoCode and should not trigger recovery. */
export function isSelfWrite(fsPath: string, now: number = Date.now()): boolean {
  const key = normalizeFsPath(fsPath);
  const until = selfWriteUntil.get(key);
  if (until === undefined) {
    return false;
  }
  if (now >= until) {
    selfWriteUntil.delete(key);
    return false;
  }
  return true;
}

export function clearSelfWritesForTests(): void {
  selfWriteUntil.clear();
}
