// Checks host/terminal.js against a recording canvas context: glyph table, palette, attribute
// decoding, blinking, cursor and redraw-only-on-change. Exit 0 on success.
import assert from "node:assert/strict";
import { Terminal, CP437, COLS, ROWS, CURSOR, clearScreen } from "../../host/terminal.js";
import { newGrid, runTests } from "./browser_helpers.mjs";

const tests = [];
const test = (name, body) => tests.push([name, body]);

// A canvas whose context logs fillRect/fillText with the fill style in force.
function fakeCanvas() {
  const log = [];
  const ctx = {
    fillStyle: "",
    fillRect(x, y, w, h) { log.push({ op: "rect", style: this.fillStyle, x, y, w, h }); },
    fillText(text, x, y) { log.push({ op: "text", style: this.fillStyle, text, x, y }); },
  };
  return { canvas: { width: 0, height: 0, getContext: () => ctx }, log };
}

function setup() {
  const { canvas, log } = fakeCanvas();
  const grid = newGrid();
  clearScreen(grid);
  return { grid, log, canvas, term: new Terminal(canvas, grid) };
}

const put = (grid, col, row, char, attr) => {
  grid[(row * COLS + col) * 2] = char.charCodeAt(0);
  grid[(row * COLS + col) * 2 + 1] = attr;
};
const texts = (log) => log.filter((e) => e.op === "text" && e.text !== " ");

test("the glyph table has 256 entries: ASCII as is, CP437 pictures and box drawing above", () => {
  assert.equal(CP437.length, 256);
  for (let c = 32; c < 127; c++) assert.equal(CP437[c], String.fromCharCode(c));
  const known = { 1: "☺", 3: "♥", 24: "↑", 127: "⌂", 128: "Ç", 176: "░", 178: "▓", 179: "│",
    186: "║", 196: "─", 201: "╔", 205: "═", 219: "█", 225: "ß", 227: "π", 248: "°", 254: "■" };
  for (const [code, glyph] of Object.entries(known)) assert.equal(CP437[code], glyph, code);
  assert.equal(new Set(CP437).size, 255, "every glyph but 0 and 255 (blanks) is distinct");
});

test("the canvas is 80x25 cells and clearScreen gives spaces, light grey on black, cursor home", () => {
  const { canvas, grid } = setup();
  assert.ok(canvas.width > 0 && canvas.width % COLS === 0);
  assert.ok(canvas.height > 0 && canvas.height % ROWS === 0);
  assert.equal(grid[0], 32);
  assert.equal(grid[1], 7);
  assert.deepEqual([grid[CURSOR], grid[CURSOR + 1]], [0, 0]);
});

test("a cell is drawn as its background rectangle and its glyph in the foreground colour", () => {
  const { grid, log, term } = setup();
  put(grid, 2, 1, "A", 0x1e); // yellow on blue
  term.draw(0);
  const cell = texts(log);
  assert.equal(cell.length, 1);
  assert.equal(cell[0].style, "#ffff55");
  const [bg] = log.filter((e) => e.op === "rect" && e.x === cell[0].x && e.y === cell[0].y);
  assert.equal(bg.style, "#0000aa");
  assert.equal(log.filter((e) => e.op === "rect" && e.w === bg.w && e.h === bg.h).length, COLS * ROWS);
});

test("all 16 colours are distinct, with the VGA values for brown and light grey", () => {
  const { grid, log, term } = setup();
  for (let colour = 0; colour < 16; colour++) put(grid, colour, 0, "x", colour);
  term.draw(0);
  const styles = texts(log).map((e) => e.style);
  assert.equal(new Set(styles).size, 16);
  assert.ok(styles.includes("#aa5500") && styles.includes("#aaaaaa") && styles.includes("#ffffff"));
});

test("background colours 0..7 come from bits 4..6", () => {
  const { canvas, grid, log, term } = setup();
  put(grid, 0, 0, "x", 0x4f);
  term.draw(0);
  const bg = log.find((e) => e.op === "rect" && e.x === 0 && e.y === 0 && e.h === canvas.height / ROWS);
  assert.equal(bg.style, "#aa0000");
});

test("blink: bit 7 hides the glyph in the off phase only", () => {
  const { grid, log, term } = setup();
  put(grid, 0, 0, "B", 0x8f);
  term.draw(0);
  assert.equal(texts(log).length, 1, "on phase");
  log.length = 0;
  term.draw(500);
  assert.equal(texts(log).length, 0, "off phase");
  term.draw(1000);
  assert.equal(texts(log).length, 1, "on again");
});

test("the cursor is an underline in the cell's colour, shown in the on phase, moving with the grid", () => {
  const { canvas, grid, log, term } = setup();
  const cellH = canvas.height / ROWS;
  grid[CURSOR] = 5;
  grid[CURSOR + 1] = 3;
  term.draw(0);
  const bars = log.filter((e) => e.op === "rect" && e.h < cellH);
  assert.equal(bars.length, 1, "one cursor bar");
  assert.equal(bars[0].style, "#aaaaaa");
  assert.ok(bars[0].x > 0 && bars[0].y > 0);
  log.length = 0;
  term.draw(500);
  assert.equal(log.filter((e) => e.op === "rect" && e.h < cellH).length, 0, "off phase: no bar");
});

test("an unchanged screen in the same blink phase is not redrawn; a changed cell is", () => {
  const { grid, log, term } = setup();
  term.draw(0);
  assert.ok(log.length > COLS * ROWS, "first draw paints everything");
  log.length = 0;
  term.draw(100);
  assert.equal(log.length, 0, "nothing changed");
  put(grid, 0, 0, "Z", 7);
  term.draw(200);
  assert.equal(texts(log).length, 1);
});

await runTests(tests);
