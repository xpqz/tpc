import test from "node:test";
import assert from "node:assert/strict";

const load = () => import("../../vscode/src/report.mjs");
const texts = {
  "main.pas": ["program a;", "begin", "  x := 1;", "end."],
  "Helper.pas": ["unit Helper;", "interface", "var y: integer;"],
};
const linesOf = (name) => texts[name];

test("a compile without error reports nothing", async () => {
  const { buildReport } = await load();
  assert.deepEqual(buildReport({ ok: true, wasm: new Uint8Array(0) }, "main.pas", linesOf), []);
});

test("an error in the main file reads Error N: text at its range", async () => {
  const { buildReport } = await load();
  const error = { code: 3, line: 3, col: 3, file: "main.pas", message: "Unknown identifier" };
  assert.deepEqual(buildReport({ ok: false, error }, "main.pas", linesOf), [
    { file: "main.pas", message: "Error 3: Unknown identifier", range: { line: 2, start: 2, end: 3 } },
  ]);
});

test("an error in a unit goes to the unit's file", async () => {
  const { buildReport } = await load();
  const error = { code: 4, line: 3, col: 5, file: "Helper.pas", message: "Duplicate identifier" };
  const [entry] = buildReport({ ok: false, error }, "main.pas", linesOf);
  assert.equal(entry.file, "Helper.pas");
  assert.deepEqual(entry.range, { line: 2, start: 4, end: 5 });
});

test("an error reported at line 0 lands on the first position of its file", async () => {
  const { buildReport } = await load();
  const error = { code: 5, line: 0, col: 0, file: "main.pas", message: "Syntax error" };
  const [entry] = buildReport({ ok: false, error }, "main.pas", linesOf);
  assert.deepEqual(entry.range, { line: 0, start: 0, end: 0 });
});

test("an internal compiler error goes to the main file at line 0", async () => {
  const { buildReport } = await load();
  const [entry] = buildReport({ ok: false, internal: "unreachable executed" }, "main.pas", linesOf);
  assert.equal(entry.file, "main.pas");
  assert.equal(entry.message, "Internal compiler error: unreachable executed");
  assert.deepEqual(entry.range, { line: 0, start: 0, end: 0 });
});

test("an error in a file whose text is unknown falls back to an empty file", async () => {
  const { buildReport } = await load();
  const error = { code: 3, line: 9, col: 4, file: "Gone.inc", message: "Unknown identifier" };
  const [entry] = buildReport({ ok: false, error }, "main.pas", () => undefined);
  assert.deepEqual(entry.range, { line: 0, start: 0, end: 0 });
});
