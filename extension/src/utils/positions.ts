/** Convert a UTF-8 byte column offset within `line` to a VS Code UTF-16 column. */
export function byteColToUtf16(line: string, byteCol: number): number {
  const encoder = new TextEncoder();
  let utf16 = 0;
  let byteIdx = 0;
  for (const ch of line) {
    if (byteIdx >= byteCol) {
      break;
    }
    // `String.length` counts UTF-16 code units (including surrogate pairs).
    utf16 += ch.length;
    byteIdx += encoder.encode(ch).byteLength;
  }
  return utf16;
}
