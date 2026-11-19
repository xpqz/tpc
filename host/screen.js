// The 80x25 text screen as a grid of cells, driven by the cell operations of the `screen`
// import (docs/abi.md section 8). Cursor, window and attribute state live in the Crt unit;
// this model only stores what Crt draws and prints it as the screen dump that
// tools/crt/SCRDUMP.PAS writes on the real Turbo Pascal.
import { HostError } from "./error.js";

export const COLUMNS = 80;
export const ROWS = 25;
const BLANK = 0x20;
const LIGHT_GREY_ON_BLACK = 0x07;

const OP_PUT = 0;
const OP_SCROLL_UP = 1;
const OP_SCROLL_DOWN = 2;
const OP_CURSOR = 3;

const low = (word) => word & 0xff;
const high = (word) => (word >> 8) & 0xff;

export class Screen {
  constructor() {
    this.chars = new Uint8Array(COLUMNS * ROWS).fill(BLANK);
    this.attrs = new Uint8Array(COLUMNS * ROWS).fill(LIGHT_GREY_ON_BLACK);
    this.cursorCol = 0;
    this.cursorRow = 0;
  }

  // Index of a cell in `chars` and `attrs`, row by row.
  cell(col, row) {
    if (col < 0 || col >= COLUMNS || row < 0 || row >= ROWS) {
      throw new HostError(`host.screen: cell (${col}, ${row}) is off the screen`);
    }
    return row * COLUMNS + col;
  }

  put(col, row, char, attr) {
    const at = this.cell(col, row);
    this.chars[at] = char;
    this.attrs[at] = attr;
  }

  // Moves the cells of the rectangle (inclusive, 0-based) up by `lines` rows, or down when
  // `lines` is negative. Each destination row reads from `lines` rows further away, so the
  // copy runs in the direction the cells move away from; rows with no source become blank.
  scroll(left, top, right, bottom, lines, attr) {
    this.cell(left, top);
    this.cell(right, bottom);
    for (let step = 0; step <= bottom - top; step++) {
      const row = lines > 0 ? top + step : bottom - step;
      const from = row + lines;
      for (let col = left; col <= right; col++) {
        if (from >= top && from <= bottom) {
          this.put(col, row, this.chars[this.cell(col, from)], this.attrs[this.cell(col, from)]);
        } else {
          this.put(col, row, BLANK, attr);
        }
      }
    }
  }

  setCursor(col, row) {
    this.cell(col, row);
    this.cursorCol = col;
    this.cursorRow = row;
  }

  // The `screen(op, a, b, c)` import. Operands are packed in 16-bit words:
  //   put          a = column, b = row, c = char | attr << 8
  //   scroll up    a = left | top << 8, b = right | bottom << 8, c = lines | attr << 8
  //   scroll down  as scroll up
  //   cursor       a = column, b = row
  apply(op, a, b, c) {
    if (op === OP_PUT) this.put(a, b, low(c), high(c));
    else if (op === OP_SCROLL_UP || op === OP_SCROLL_DOWN) {
      const lines = op === OP_SCROLL_UP ? low(c) : -low(c);
      this.scroll(low(a), high(a), low(b), high(b), lines, high(c));
    } else if (op === OP_CURSOR) this.setCursor(a, b);
    else throw new HostError(`host.screen: bad operation ${op}`);
  }

  // The 25 rows without trailing blanks.
  textRows() {
    const rows = [];
    for (let row = 0; row < ROWS; row++) {
      const cells = this.chars.subarray(row * COLUMNS, (row + 1) * COLUMNS);
      rows.push(String.fromCharCode(...cells).replace(/ +$/, ""));
    }
    return rows;
  }

  // One `attr ROW COL LEN HEX` line (1-based) per run of cells whose attribute is not 07.
  attributeRuns() {
    const runs = [];
    for (let row = 0; row < ROWS; row++) {
      let col = 0;
      while (col < COLUMNS) {
        const attr = this.attrs[this.cell(col, row)];
        const start = col;
        while (col < COLUMNS && this.attrs[this.cell(col, row)] === attr) col++;
        if (attr !== LIGHT_GREY_ON_BLACK) {
          const hex = attr.toString(16).toUpperCase().padStart(2, "0");
          runs.push(`attr ${row + 1} ${start + 1} ${col - start} ${hex}`);
        }
      }
    }
    return runs;
  }

  // The screen dump: the text rows, the cursor (1-based column and row), the exit code and
  // the attribute runs, one per line.
  dump(exitCode) {
    const lines = [...this.textRows()];
    lines.push(`cursor ${this.cursorCol + 1} ${this.cursorRow + 1}`);
    lines.push(`exit ${exitCode}`);
    lines.push(...this.attributeRuns());
    return lines.map((line) => `${line}\n`).join("");
  }
}
