import test from "node:test";
import assert from "node:assert/strict";

const load = () => import("../../vscode/src/positions.mjs");

test("splitLines handles LF, CRLF and a final line without a newline", async () => {
  const { splitLines } = await load();
  assert.deepEqual(splitLines("a\nb\r\nc"), ["a", "b", "c"]);
  assert.deepEqual(splitLines(""), [""]);
  assert.deepEqual(splitLines("a\n"), ["a", ""]);
});

test("the range starts at the 1-based column and covers the identifier there", async () => {
  const { errorRange } = await load();
  const lines = ["program a;", "begin", "  x := foo + 1;", "end."];
  assert.deepEqual(errorRange(lines, 3, 3), { line: 2, start: 2, end: 3 }); // x
  assert.deepEqual(errorRange(lines, 3, 8), { line: 2, start: 7, end: 10 }); // foo
  assert.deepEqual(errorRange(lines, 3, 5), { line: 2, start: 4, end: 5 }, "a symbol is one char");
});

test("a byte above 127 is one column, as in the editor's text", async () => {
  const { errorRange } = await load();
  const lines = ["  writeln('café'); x"];
  assert.deepEqual(errorRange(lines, 1, 20), { line: 0, start: 19, end: 20 });
});

test("a column past the end of the line marks the last character", async () => {
  const { errorRange } = await load();
  assert.deepEqual(errorRange(["abc"], 1, 4), { line: 0, start: 2, end: 3 });
  assert.deepEqual(errorRange(["abc"], 1, 99), { line: 0, start: 2, end: 3 });
});

test("an empty line gives an empty range at column 0", async () => {
  const { errorRange } = await load();
  assert.deepEqual(errorRange(["", "x"], 1, 1), { line: 0, start: 0, end: 0 });
});

test("a line past the end of the text is clamped to the last line", async () => {
  const { errorRange } = await load();
  assert.deepEqual(errorRange(["program a;", "begin"], 9, 1), { line: 1, start: 0, end: 5 });
});

test("line 0 (an embedded runtime unit) is the first position of the file", async () => {
  const { errorRange } = await load();
  assert.deepEqual(errorRange(["program a;"], 0, 0), { line: 0, start: 0, end: 0 });
  assert.deepEqual(errorRange(["program a;"], 0, 5), { line: 0, start: 0, end: 0 });
});

test("a tab counts as one column", async () => {
  const { errorRange } = await load();
  assert.deepEqual(errorRange(["\t\tx := 1;"], 1, 3), { line: 0, start: 2, end: 3 });
});
