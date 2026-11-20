// The renderer that turns the Crt grid into terminal text: colours, CP437, the diff between
// frames, the cursor, and the window-size handling.
import test from "node:test";
import assert from "node:assert/strict";
import { COLS, ROWS, CURSOR, GRID_BYTES, cellOffset, clearScreen } from "../../host/terminal.js";
import {
  ENTER_SCREEN, LEAVE_SCREEN, attributeCode, createScreenWriter,
} from "../../vscode/src/screen_ansi.mjs";
import { AnsiScreen } from "./ansi_screen.mjs";

const HIDE = "\x1b[?25l";
const SHOW = "\x1b[?25h";
const goto = (row, col) => `\x1b[${row};${col}H`;

function blankGrid() {
  const grid = new Uint8Array(GRID_BYTES);
  clearScreen(grid);
  return grid;
}

function setCell(grid, col, row, char, attr = 7) {
  [grid[cellOffset(col, row)], grid[cellOffset(col, row) + 1]] = [char.charCodeAt(0), attr];
}

const setCursor = (grid, col, row) => ([grid[CURSOR], grid[CURSOR + 1]] = [col, row]);

// Applies `texts` to a terminal that is on the alternate screen and returns its cells.
function shown(...texts) {
  const terminal = new AnsiScreen();
  terminal.feed(ENTER_SCREEN);
  for (const text of texts) terminal.feed(text);
  return terminal;
}

test("entering and leaving the alternate screen", () => {
  assert.equal(ENTER_SCREEN, "\x1b[?1049h");
  assert.ok(LEAVE_SCREEN.startsWith("\x1b[?1049l"));
  const terminal = new AnsiScreen();
  terminal.feed(ENTER_SCREEN);
  assert.equal(terminal.onAlternate, true);
  terminal.feed(LEAVE_SCREEN);
  assert.equal(terminal.onAlternate, false);
  assert.equal(terminal.cursorVisible, true, "the cursor is shown again after the program");
  assert.equal(terminal.attr, 7, "the colours are reset after the program");
});

test("attributeCode: DOS colour order becomes ANSI order (1 is blue = 34, 4 is red = 31)", () => {
  const ansi = [30, 34, 32, 36, 31, 35, 33, 37]; // DOS colour 0..7 as ANSI foreground codes
  assert.equal(attributeCode(0x07), "\x1b[0;37;40m");
  assert.equal(attributeCode(0x01), "\x1b[0;34;40m");
  assert.equal(attributeCode(0x04), "\x1b[0;31;40m");
  assert.equal(attributeCode(0x06), "\x1b[0;33;40m", "brown is the ANSI yellow");
  assert.equal(attributeCode(0x1e), "\x1b[0;93;44m", "yellow on blue");
  assert.equal(attributeCode(0x70), "\x1b[0;30;47m");
  assert.equal(attributeCode(0x0c), "\x1b[0;91;40m", "light red uses the bright range");
  for (let dos = 0; dos < 8; dos++) {
    assert.equal(attributeCode(dos), `\x1b[0;${ansi[dos]};40m`);
    assert.equal(attributeCode(dos | 8), `\x1b[0;${ansi[dos] + 60};40m`);
    assert.equal(attributeCode(dos << 4), `\x1b[0;30;${ansi[dos] + 10}m`);
  }
});

test("attributeCode: the blink bit is a bright background, never the blink attribute", () => {
  assert.equal(attributeCode(0x87), "\x1b[0;37;100m");
  assert.equal(attributeCode(0xff), "\x1b[0;97;107m");
  assert.equal(attributeCode(0x9e), "\x1b[0;93;104m");
  for (let attr = 0; attr < 256; attr++) assert.doesNotMatch(attributeCode(attr), /[;\[]5[;m]/);
});

test("attributeCode: every attribute survives a round trip through the emulator", () => {
  for (let attr = 0; attr < 256; attr++) {
    const terminal = new AnsiScreen();
    terminal.feed(attributeCode(attr));
    assert.equal(terminal.attr, attr, `attribute ${attr}`);
  }
});

test("the first frame draws every cell and puts the cursor where the grid has it", () => {
  const grid = blankGrid();
  "Hello".split("").forEach((char, col) => setCell(grid, col, 0, char, 0x1e));
  setCell(grid, 79, 24, "Z", 0x4f);
  setCursor(grid, 5, 0);
  const text = createScreenWriter().frame(grid);
  assert.ok(text.startsWith(HIDE), "the cursor is hidden while drawing");
  assert.ok(text.endsWith(`${goto(1, 6)}${SHOW}`), JSON.stringify(text.slice(-30)));
  const terminal = shown(text);
  assert.deepEqual(terminal.grid(), grid);
  assert.equal(terminal.cursorVisible, true);
});

test("a frame of an unchanged grid is empty", () => {
  const writer = createScreenWriter();
  const grid = blankGrid();
  assert.notEqual(writer.frame(grid), "");
  assert.equal(writer.frame(grid), "");
  assert.equal(writer.frame(grid), "");
});

test("only the changed cell is drawn", () => {
  const writer = createScreenWriter();
  const grid = blankGrid();
  writer.frame(grid);
  setCell(grid, 5, 3, "X", 0x1e);
  assert.equal(
    writer.frame(grid),
    `${HIDE}${goto(4, 6)}${attributeCode(0x1e)}X${goto(1, 1)}${SHOW}`,
  );
});

test("a changed attribute with the same character is drawn", () => {
  const writer = createScreenWriter();
  const grid = blankGrid();
  setCell(grid, 2, 2, "A", 7);
  writer.frame(grid);
  setCell(grid, 2, 2, "A", 0x4f);
  assert.equal(
    writer.frame(grid),
    `${HIDE}${goto(3, 3)}${attributeCode(0x4f)}A${goto(1, 1)}${SHOW}`,
  );
});

test("a run of changed cells is one move and one colour change", () => {
  const writer = createScreenWriter();
  const grid = blankGrid();
  writer.frame(grid);
  "XYZ".split("").forEach((char, i) => setCell(grid, 5 + i, 3, char, 0x1e));
  assert.equal(
    writer.frame(grid),
    `${HIDE}${goto(4, 6)}${attributeCode(0x1e)}XYZ${goto(1, 1)}${SHOW}`,
  );
});

test("cells that did not change between two changed cells are skipped with a move", () => {
  const writer = createScreenWriter();
  const grid = blankGrid();
  writer.frame(grid);
  setCell(grid, 0, 0, "A", 0x1e);
  setCell(grid, 40, 0, "B", 0x1e);
  assert.equal(
    writer.frame(grid),
    `${HIDE}${goto(1, 1)}${attributeCode(0x1e)}A${goto(1, 41)}B${goto(1, 1)}${SHOW}`,
  );
});

test("a colour change inside a run is emitted only where the attribute changes", () => {
  const writer = createScreenWriter();
  const grid = blankGrid();
  writer.frame(grid);
  setCell(grid, 0, 0, "A", 0x1e);
  setCell(grid, 1, 0, "B", 0x1e);
  setCell(grid, 2, 0, "C", 0x4f);
  const text = writer.frame(grid);
  assert.equal(text.split("\x1b[0;").length - 1, 2);
  assert.deepEqual(shown(createScreenWriter().frame(grid)).grid(), grid);
});

test("a write in the last column is followed by a move, never by the terminal's wrap", () => {
  const writer = createScreenWriter();
  const grid = blankGrid();
  writer.frame(grid);
  setCell(grid, 79, 0, "R", 0x1e);
  setCell(grid, 0, 1, "S", 0x1e);
  const text = writer.frame(grid);
  assert.ok(text.includes(`R${goto(2, 1)}`), JSON.stringify(text));
  assert.equal(shown(createScreenWriter().frame(blankGrid()), text).cells.slice(0, 2).join(), "32,7");
});

test("changes on several rows give one move per row, in row order", () => {
  const writer = createScreenWriter();
  const grid = blankGrid();
  writer.frame(grid);
  setCell(grid, 3, 20, "L", 7);
  setCell(grid, 3, 2, "F", 7);
  const text = writer.frame(grid);
  assert.ok(text.indexOf(goto(3, 4)) < text.indexOf(goto(21, 4)));
  assert.ok(text.indexOf(goto(3, 4)) >= 0 && text.indexOf(goto(21, 4)) >= 0);
});

test("a move of the cursor alone is only a cursor move", () => {
  const writer = createScreenWriter();
  const grid = blankGrid();
  writer.frame(grid);
  setCursor(grid, 2, 1);
  assert.equal(writer.frame(grid), goto(2, 3));
  assert.equal(writer.frame(grid), "");
});

test("the cursor ends at the grid's cursor cell after cells were drawn", () => {
  const writer = createScreenWriter();
  const grid = blankGrid();
  writer.frame(grid);
  setCell(grid, 0, 0, "A");
  setCursor(grid, 1, 0);
  assert.ok(writer.frame(grid).endsWith(`${goto(1, 2)}${SHOW}`));
});

test("CP437 bytes are drawn as their Unicode glyphs", () => {
  const grid = blankGrid();
  const bytes = [0xc9, 0xcd, 0xbb, 0x01, 0xb0, 0xe1, 0x80, 0x7f, 0xff];
  bytes.forEach((byte, col) => ([grid[cellOffset(col, 0)], grid[cellOffset(col, 0) + 1]] = [byte, 7]));
  const text = createScreenWriter().frame(grid);
  assert.ok(text.includes("╔═╗☺░ßÇ⌂ "), JSON.stringify(text.slice(0, 400)));
  assert.deepEqual(shown(text).grid(), grid);
});

test("frames applied in turn rebuild the grid, whatever changed in between", () => {
  const writer = createScreenWriter();
  const terminal = shown();
  const grid = blankGrid();
  let seed = 7;
  const next = () => (seed = (seed * 1103515245 + 12345) & 0x7fffffff);
  for (let frame = 0; frame < 30; frame++) {
    for (let k = 0; k < 40; k++) {
      const at = next() % (COLS * ROWS);
      const byte = 0x20 + (next() % 200);
      grid[at * 2] = byte === 0x20 + 127 ? 0x41 : byte;
      grid[at * 2 + 1] = next() & 0xff;
    }
    setCursor(grid, next() % COLS, next() % ROWS);
    terminal.feed(writer.frame(grid));
    assert.deepEqual(terminal.grid(), grid, `frame ${frame}`);
  }
});

test("a terminal smaller than 80x25 gets a notice, not the picture", () => {
  const writer = createScreenWriter();
  writer.resize(60, 20);
  const grid = blankGrid();
  setCell(grid, 0, 0, "Q");
  const text = writer.frame(grid);
  assert.match(text, /enlarge/i);
  assert.match(text, /80x25/);
  assert.ok(!text.includes("Q"), "no cells are drawn");
  assert.ok(text.length < 200, "one line");
  assert.ok(!text.includes("\n"));
  const terminal = new AnsiScreen();
  assert.doesNotThrow(() => terminal.feed(text));
});

test("the notice is shown once; frames stay silent while the window is too small", () => {
  const writer = createScreenWriter();
  writer.resize(79, 25);
  const grid = blankGrid();
  assert.notEqual(writer.frame(grid), "");
  setCell(grid, 1, 1, "A");
  assert.equal(writer.frame(grid), "");
  writer.resize(80, 24);
  assert.match(writer.frame(grid), /enlarge/i, "a new size gets a new notice");
});

test("when the window is big enough again the whole screen is redrawn", () => {
  const writer = createScreenWriter();
  const grid = blankGrid();
  "kept".split("").forEach((char, col) => setCell(grid, col, 4, char, 0x2f));
  writer.frame(grid);
  writer.resize(60, 20);
  writer.frame(grid);
  setCell(grid, 0, 0, "N");
  writer.resize(80, 25);
  const text = writer.frame(grid);
  assert.ok(text.includes("\x1b[2J"), "the notice is cleared");
  assert.deepEqual(shown(text).grid(), grid);
  assert.equal(text.split("kept").length, 2, "unchanged cells are drawn again");
  assert.equal(writer.frame(grid), "");
});

test("a bigger window also redraws; the same size does not", () => {
  const writer = createScreenWriter();
  const grid = blankGrid();
  writer.frame(grid);
  writer.resize(80, 25);
  assert.equal(writer.frame(grid), "", "the size did not change");
  writer.resize(120, 40);
  const text = writer.frame(grid);
  assert.ok(text.includes("\x1b[2J"));
  assert.deepEqual(shown(text).grid(), grid);
});
