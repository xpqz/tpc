// A small terminal emulator for the escape sequences the Crt renderer may emit: cursor
// position, clear screen, colours, cursor visibility and the alternate screen. It rebuilds the
// 80x25 grid (the layout of host/terminal.js) from the text a Pseudoterminal wrote, so a test
// can compare what a user would see with the grid the program drew. Anything else throws, which
// makes an unexpected sequence a test failure; so does a write past column 80, because the
// renderer must never rely on a terminal's wrapping.
import { COLS, ROWS, CURSOR, GRID_BYTES, CP437 } from "../../host/terminal.js";

const BLANK_ATTR = 7;
const NBSP_BYTE = 255;
// ANSI colour number (red = 1, blue = 4) to DOS colour number (blue = 1, red = 4).
const DOS_OF_ANSI = [0, 4, 2, 6, 1, 5, 3, 7];

function blankScreen() {
  const cells = new Uint8Array(GRID_BYTES);
  for (let i = 0; i < CURSOR; i += 2) [cells[i], cells[i + 1]] = [0x20, BLANK_ATTR];
  return cells;
}

const byteOfGlyph = (glyph) => {
  if (glyph === " ") return 0x20;
  if (glyph === " ") return NBSP_BYTE;
  const byte = CP437.indexOf(glyph);
  if (byte < 0) throw new Error(`not a CP437 glyph: ${JSON.stringify(glyph)}`);
  return byte;
};

export class AnsiScreen {
  constructor() {
    this.normal = blankScreen();
    this.alternate = blankScreen();
    this.onAlternate = false;
    this.cursorVisible = true;
    this.attr = BLANK_ATTR;
    this.sequences = []; // every escape sequence seen, in order
    this.glyphsWritten = 0;
  }

  get cells() {
    return this.onAlternate ? this.alternate : this.normal;
  }

  feed(text) {
    let i = 0;
    while (i < text.length) {
      if (text[i] === "\x1b") i = this.escape(text, i);
      else i = this.character(text, i);
    }
  }

  character(text, i) {
    const cells = this.cells;
    const char = text[i];
    if (char === "\r") cells[CURSOR] = 0;
    else if (char === "\n") cells[CURSOR + 1] = Math.min(cells[CURSOR + 1] + 1, ROWS - 1);
    else {
      if (cells[CURSOR] >= COLS) throw new Error("write past column 80");
      const at = (cells[CURSOR + 1] * COLS + cells[CURSOR]) * 2;
      [cells[at], cells[at + 1]] = [byteOfGlyph(char), this.attr];
      cells[CURSOR]++;
      this.glyphsWritten++;
    }
    return i + 1;
  }

  escape(text, start) {
    const match = /^\x1b\[([?0-9;]*)([A-Za-z])/.exec(text.slice(start, start + 40));
    if (!match) throw new Error(`unknown escape at ${JSON.stringify(text.slice(start, start + 12))}`);
    this.sequences.push(match[0]);
    const [whole, params, final] = match;
    if (params.startsWith("?")) this.privateMode(params.slice(1), final);
    else if (final === "H") this.moveTo(params);
    else if (final === "J" && params === "2") this.clear();
    else if (final === "m") this.colours(params.split(";").map(Number));
    else throw new Error(`unsupported sequence ${JSON.stringify(whole)}`);
    return start + whole.length;
  }

  privateMode(mode, final) {
    const on = final === "h";
    if (final !== "h" && final !== "l") throw new Error("bad private mode");
    if (mode === "25") this.cursorVisible = on;
    else if (mode === "1049") {
      this.onAlternate = on;
      if (on) this.alternate = blankScreen();
    } else throw new Error(`unsupported private mode ${mode}`);
  }

  moveTo(params) {
    const [row = 1, col = 1] = params === "" ? [] : params.split(";").map(Number);
    if (row < 1 || row > ROWS || col < 1 || col > COLS) throw new Error(`goto ${row};${col}`);
    [this.cells[CURSOR], this.cells[CURSOR + 1]] = [col - 1, row - 1];
  }

  clear() {
    const cursor = [this.cells[CURSOR], this.cells[CURSOR + 1]];
    this.cells.set(blankScreen());
    [this.cells[CURSOR], this.cells[CURSOR + 1]] = cursor;
  }

  // 0 resets, 30-37 and 90-97 the foreground, 40-47 and 100-107 the background; a bright
  // background is the DOS blink bit (bit 7) and a bright foreground bit 3.
  colours(codes) {
    for (const code of codes) {
      if (code === 0) this.attr = BLANK_ATTR;
      else if (code >= 30 && code <= 37) this.attr = (this.attr & 0xf8) | DOS_OF_ANSI[code - 30];
      else if (code >= 90 && code <= 97) this.attr = (this.attr & 0xf0) | 8 | DOS_OF_ANSI[code - 90];
      else if (code >= 40 && code <= 47) this.attr = (this.attr & 0x0f) | (DOS_OF_ANSI[code - 40] << 4);
      else if (code >= 100 && code <= 107)
        this.attr = (this.attr & 0x0f) | 0x80 | (DOS_OF_ANSI[code - 100] << 4);
      else throw new Error(`unsupported colour code ${code}`);
    }
  }

  // The visible screen as the grid layout, with the cursor in the grid's cursor cell.
  grid() {
    return this.cells.slice();
  }

  rowText(row) {
    let text = "";
    for (let col = 0; col < COLS; col++) text += String.fromCharCode(this.cells[(row * COLS + col) * 2]);
    return text.trimEnd();
  }
}
