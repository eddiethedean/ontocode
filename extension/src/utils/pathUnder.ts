import * as path from "path";

/**
 * Normalize a filesystem path for comparison and VS Code URI use.
 * Strips Windows extended-length / verbatim prefixes from Rust `canonicalize()`
 * (`\\?\C:\...` and `\\?\UNC\server\share\...`).
 */
export function normalizeFsPath(filePath: string): string {
  let normalized = filePath;
  if (normalized.startsWith("\\\\?\\UNC\\")) {
    normalized = `\\\\${normalized.slice("\\\\?\\UNC\\".length)}`;
  } else if (normalized.startsWith("\\\\?\\")) {
    normalized = normalized.slice("\\\\?\\".length);
  } else if (normalized.startsWith("//?/UNC/")) {
    normalized = `//${normalized.slice("//?/UNC/".length)}`;
  } else if (normalized.startsWith("//?/")) {
    normalized = normalized.slice("//?/".length);
  }
  return path.resolve(normalized);
}

/** True when two filesystem paths refer to the same location. */
export function pathsEqual(a: string, b: string): boolean {
  return pathIdentityKey(a) === pathIdentityKey(b);
}

/**
 * Stable map key for filesystem path identity (case-folded on Windows).
 * Prefer this over raw `normalizeFsPath` when looking up by path.
 */
export function pathIdentityKey(filePath: string): string {
  const normalized = normalizeFsPath(filePath);
  return process.platform === "win32" ? normalized.toLowerCase() : normalized;
}

/**
 * True when `filePath` is the folder itself or a descendant.
 * Uses a path-separator boundary so `/ws/proj` does not match `/ws/proj-other` (#151).
 */
export function isPathUnderFolder(filePath: string, folderFsPath: string): boolean {
  const file = normalizeFsPath(filePath);
  const folder = normalizeFsPath(folderFsPath);
  if (process.platform === "win32") {
    const fileKey = file.toLowerCase();
    const folderKey = folder.toLowerCase();
    if (fileKey === folderKey) {
      return true;
    }
    const prefix = folderKey.endsWith(path.sep) ? folderKey : folderKey + path.sep;
    return fileKey.startsWith(prefix);
  }
  if (file === folder) {
    return true;
  }
  const prefix = folder.endsWith(path.sep) ? folder : folder + path.sep;
  return file.startsWith(prefix);
}
