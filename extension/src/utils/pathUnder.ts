import * as path from "path";

/**
 * True when `filePath` is the folder itself or a descendant.
 * Uses a path-separator boundary so `/ws/proj` does not match `/ws/proj-other` (#151).
 */
export function isPathUnderFolder(filePath: string, folderFsPath: string): boolean {
  const file = path.resolve(filePath);
  let folder = path.resolve(folderFsPath);
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
