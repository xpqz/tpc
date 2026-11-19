// Checks host/screen.js (the cell grid and its dump), tests/keys.mjs and the virtual clock
// contract against docs/abi.md section 8. The golden dumps in tests/tools/fixtures/tp5/crt_*.out
// were produced by the real Turbo Pascal 5.0 (tools/tp5.sh); the grid must print the same
// bytes when the same cells are drawn. Exit 0 on success.
import assert from "node:assert/strict";
import fs from "node:fs";
import { Host, HostError, KEYS_EXHAUSTED } from "../../host/runtime.js";
import { Screen } from "../../host/screen.js";
import { KeyScript, KeyScriptError, parseKeys } from "../keys.mjs";

const golden = (name) =>
  fs.readFileSync(new URL(`../tools/fixtures/tp5/${name}.out`, import.meta.url), "latin1");

const tests = [];
const test = (name, body) => tests.push([name, body]);

const PUT = 0;
const SCROLL_UP = 1;
const SCROLL_DOWN = 2;
const CURSOR = 3;

// Draws `text` row `row` from column `col` the way Crt's Write does: put each cell.
function draw(screen, col, row, text, attr = 7) {
  for (const ch of text) screen.apply(PUT, col++, row, ch.charCodeAt(0) | (attr << 8));
}

const rows = (screen) => screen.dump(0).split("\n").slice(0, 25);

test("a new screen is 25 blank rows, cursor 1 1, exit line, no attribute runs", () => {
  assert.equal(new Screen().dump(0), `${"\n".repeat(25)}cursor 1 1\nexit 0\n`);
});

test("put stores the character and attribute; trailing blanks are cut, inner ones kept", () => {
  const screen = new Screen();
  draw(screen, 2, 0, "a b");
  assert.equal(rows(screen)[0], "  a b");
  draw(screen, 79, 24, "z");
  assert.equal(rows(screen)[24], `${" ".repeat(79)}z`);
});

test("attribute runs: 1-based row, column and length, in reading order, 07 omitted", () => {
  const screen = new Screen();
  draw(screen, 9, 11, "colour", 0x1e);
  draw(screen, 0, 11, "ab", 0x70);
  draw(screen, 77, 11, "xyz", 0x70);
  const lines = screen.dump(3).split("\n");
  assert.deepEqual(lines.slice(25), [
    "cursor 1 1", "exit 3", "attr 12 1 2 70", "attr 12 10 6 1E", "attr 12 78 3 70", "",
  ]);
});

test("a blank cell with a non-default attribute is an attribute run but no text", () => {
  const screen = new Screen();
  screen.apply(PUT, 4, 2, 0x20 | (0x4f << 8));
  const lines = screen.dump(0).split("\n");
  assert.equal(lines[2], "");
  assert.ok(lines.includes("attr 3 5 1 4F"));
});

test("the cursor is dumped 1-based", () => {
  const screen = new Screen();
  screen.apply(CURSOR, 79, 24, 0);
  assert.ok(screen.dump(0).includes("cursor 80 25\n"));
});

test("scroll up moves the rectangle's rows up and blanks the bottom with the fill attribute", () => {
  const screen = new Screen();
  for (let r = 0; r < 4; r++) draw(screen, 0, r, `row${r}`);
  // rectangle: left 0, top 1, right 79, bottom 3; one line; fill attribute 0x1f
  screen.apply(SCROLL_UP, 0 | (1 << 8), 79 | (3 << 8), 1 | (0x1f << 8));
  assert.deepEqual(rows(screen).slice(0, 4), ["row0", "row2", "row3", ""]);
  assert.ok(screen.dump(0).includes("attr 4 1 80 1F\n"));
});

test("scroll down moves the rectangle's rows down and blanks the top", () => {
  const screen = new Screen();
  for (let r = 0; r < 4; r++) draw(screen, 0, r, `row${r}`);
  screen.apply(SCROLL_DOWN, 0 | (1 << 8), 79 | (3 << 8), 1 | (0x07 << 8));
  assert.deepEqual(rows(screen).slice(0, 4), ["row0", "", "row1", "row2"]);
});

test("scroll by more than the height blanks the rectangle; by 0 changes nothing", () => {
  const screen = new Screen();
  for (let r = 0; r < 3; r++) draw(screen, 0, r, `row${r}`);
  screen.apply(SCROLL_UP, 0, 79 | (2 << 8), 0);
  assert.deepEqual(rows(screen).slice(0, 3), ["row0", "row1", "row2"]);
  screen.apply(SCROLL_UP, 0, 79 | (2 << 8), 3);
  assert.deepEqual(rows(screen).slice(0, 3), ["", "", ""]);
});

test("scroll touches only the columns of the rectangle", () => {
  const screen = new Screen();
  draw(screen, 0, 0, "abcdef");
  draw(screen, 0, 1, "ghijkl");
  // columns 2..3, rows 0..1, up by one
  screen.apply(SCROLL_UP, 2 | (0 << 8), 3 | (1 << 8), 1 | (7 << 8));
  assert.deepEqual(rows(screen).slice(0, 2), ["abijef", "gh  kl"]);
});

test("the whole screen scrolls up one line, as when output passes the last row", () => {
  const screen = new Screen();
  draw(screen, 0, 0, "first");
  draw(screen, 0, 24, "last");
  screen.apply(SCROLL_UP, 0, 79 | (24 << 8), 1 | (7 << 8));
  assert.equal(rows(screen)[0], "");
  assert.equal(rows(screen)[23], "last");
  assert.equal(rows(screen)[24], "");
});

test("operations outside the screen, or unknown, are host errors", () => {
  const screen = new Screen();
  assert.throws(() => screen.apply(PUT, 80, 0, 0x41), HostError);
  assert.throws(() => screen.apply(PUT, 0, 25, 0x41), HostError);
  assert.throws(() => screen.apply(CURSOR, -1, 0, 0), HostError);
  assert.throws(() => screen.apply(SCROLL_UP, 0, 80 | (24 << 8), 1), HostError);
  assert.throws(() => screen.apply(9, 0, 0, 0), HostError);
});

test("high characters are dumped as single bytes", () => {
  const screen = new Screen();
  screen.apply(PUT, 0, 0, 0xe9 | (7 << 8));
  assert.equal(screen.dump(0).charCodeAt(0), 0xe9);
});

test("same bytes as the real TP 5.0: output without ClrScr after cls", () => {
  const screen = new Screen();
  draw(screen, 0, 0, "first");
  draw(screen, 0, 1, "second");
  screen.apply(CURSOR, 6, 1, 0);
  assert.equal(screen.dump(0), golden("crt_noclear"));
});

test("same bytes as the real TP 5.0: keys, ReadLn echo and a coloured write", () => {
  const screen = new Screen();
  ["TRUE", "97", "0", "72 72", "0", "59", " hi", "[ hi]", "FALSE"].forEach((line, row) =>
    draw(screen, 0, row, line));
  draw(screen, 9, 11, "colour", 0x1e);
  screen.apply(CURSOR, 15, 11, 0);
  assert.equal(screen.dump(0), golden("crt_keys"));
});

test("same bytes as the real TP 5.0: exit 255 after the keys ran out", () => {
  const screen = new Screen();
  draw(screen, 0, 0, "q");
  screen.apply(CURSOR, 1, 0, 0);
  assert.equal(screen.dump(255), golden("crt_exhausted"));
});

// ---- key scripts ----

test("a key script: names in any case, one printable character per other token", () => {
  assert.deepEqual(parseKeys("a Up F1 Esc Space h i Enter x"),
    [0x61, 0x4800, 0x3b00, 0x011b, 0x3920, 0x68, 0x69, 0x1c0d, 0x78]);
  assert.deepEqual(parseKeys("ENTER enter\n  f10\tPgDn\n"), [0x1c0d, 0x1c0d, 0x4400, 0x5100]);
  assert.deepEqual(parseKeys(""), []);
  assert.deepEqual(parseKeys("# $ ; ~"), [0x23, 0x24, 0x3b, 0x7e]);
});

test("a key script rejects unknown names and non-printable characters", () => {
  for (const bad of ["Bogus", "ab", "F11", "é"]) {
    assert.throws(() => parseKeys(`a ${bad}`), KeyScriptError, bad);
  }
});

test("every key of tools/crt/KEYS.TBL parses", () => {
  const table = fs.readFileSync(new URL("../../tools/crt/KEYS.TBL", import.meta.url), "latin1");
  const names = table.split("\n").filter(Boolean).map((line) => line.split(" ")[0]);
  assert.ok(names.length >= 25);
  assert.equal(parseKeys(names.join(" ")).length, names.length);
});

// ---- the host imports on top of the model ----

const clock = (start) => {
  let now = start;
  return { nowMs: () => now, advance: (ms) => (now += ms) };
};

function host(words, start = 0) {
  return new Host({ write() {}, input: null, clock: clock(start), keys: new KeyScript(words) });
}

test("readkey serves the keys in order; keypressed does not consume", () => {
  const h = host([0x61, 0x4800]);
  assert.equal(h.imports.keypressed(), 1);
  assert.equal(h.imports.keypressed(), 1);
  assert.equal(h.imports.readkey(), 0x61);
  assert.equal(h.imports.readkey(), 0x4800);
  assert.equal(h.imports.keypressed(), 0);
});

test("readkey with no keys left halts with the marker code", () => {
  const h = host([]);
  assert.throws(() => h.imports.readkey(), (e) => e.code === KEYS_EXHAUSTED);
  assert.equal(KEYS_EXHAUSTED, 255);
});

test("sleep advances the clock that time reads, and does not wait", () => {
  const h = host([], 5 * 3600000 + 20 * 1000);
  h.imports.time(0);
  assert.equal(h.imports.time(1), 5 * 3600 + 20);
  const before = Date.now();
  h.imports.sleep(2500);
  assert.ok(Date.now() - before < 500);
  h.imports.time(0);
  assert.equal(h.imports.time(1), 5 * 3600 + 22);
  assert.equal(h.imports.time(2), 50);
});

test("the screen import applies to the host's grid", () => {
  const h = host([]);
  h.imports.screen(PUT, 0, 0, 0x5a | (0x0f << 8));
  assert.ok(h.screen.dump(0).startsWith("Z\n"));
});

let failed = 0;
for (const [name, body] of tests) {
  try {
    body();
    console.log(`ok   ${name}`);
  } catch (e) {
    failed++;
    console.log(`FAIL ${name}: ${e.message.split("\n")[0]}`);
  }
}
process.exitCode = failed === 0 ? 0 : 1;
