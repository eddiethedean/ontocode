import assert from "node:assert/strict";
import { test } from "node:test";
import { byteColToUtf16 } from "./positions";

test("byteColToUtf16 counts UTF-16 units for emoji", () => {
  // "# 😀" — emoji is 4 UTF-8 bytes / 2 UTF-16 units. Mid-sequence byte 5 → 4.
  assert.equal(byteColToUtf16("# 😀", 5), 4);
});

test("byteColToUtf16 handles ascii", () => {
  assert.equal(byteColToUtf16("hello", 3), 3);
});
