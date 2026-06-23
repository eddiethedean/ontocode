/** Convert a UTF-8 byte column offset within `line` to a VS Code UTF-16 column. */
export function byteColToUtf16(line: string, byteCol: number): number {
  const encoder = new TextEncoder();
  const bytes = encoder.encode(line);
  const prefix = bytes.subarray(0, Math.min(byteCol, bytes.length));
  return new TextDecoder().decode(prefix).length;
}
