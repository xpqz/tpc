// The 80x25 text screen, driven by the cell operations of the `screen` import (docs/abi.md
// section 8) and by console output. `cells` is the byte layout host/terminal.js draws: a
// character and an attribute byte per cell, row by row, then the cursor column and row. The
// browser shares one such array with the page; the test runner makes its own. Cursor, window
// and attribute state live in the Crt unit; this model only stores what Crt draws. The test
// runner prints it with tests/screen_dump.mjs.
import { HostError } from "./error.js";
import {
  COLS, ROWS, CURSOR, GRID_BYTES, DEFAULT_ATTR, BEL, BS, LF, CR, cellOffset, clearScreen,
} from "./terminal.js";

const BLANK = 0x20;
const OP_PUT = 0;
const OP_SCROLL_UP = 1;
const OP_SCROLL_DOWN = 2;
const OP_CURSOR = 3;

// Operands of `screen` are 16-bit words that pack two bytes.
const low = (word) => word & 0xff;
const high = (word) => (word >> 8) & 0xff;

// A blank screen: light grey blanks, cursor at the top left.
function blankCells() {
  const cells = new Uint8Array(GRID_BYTES);
  clearScreen(cells);
  return cells;
}

export class Screen {
  constructor(cells = blankCells()) {
    this.cells = cells;
  }

  // Offset of a cell's character byte; its attribute follows.
  cell(col, row) {
    if (col < 0 || col >= COLS || row < 0 || row >= ROWS) {
      throw new HostError(`host.screen: cell (${col}, ${row}) is off the screen`);
    }
    return cellOffset(col, row);
  }

  put(col, row, char, attr) {
    const at = this.cell(col, row);
    [this.cells[at], this.cells[at + 1]] = [char, attr];
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
          const source = this.cell(col, from);
          this.put(col, row, this.cells[source], this.cells[source + 1]);
        } else {
          this.put(col, row, BLANK, attr);
        }
      }
    }
  }

  setCursor(col, row) {
    this.cell(col, row);
    [this.cells[CURSOR], this.cells[CURSOR + 1]] = [col, row];
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

  // Console output of a program without Crt: a teletype. Characters go to the cursor cell
  // (keeping its attribute) and the cursor wraps at column 80; the screen scrolls up after
  // the last row. CR, LF and BS move the cursor; BEL is ignored.
  print(bytes) {
    for (const byte of bytes) {
      let [col, row] = [this.cells[CURSOR], this.cells[CURSOR + 1]];
      if (byte === CR) col = 0;
      else if (byte === LF) row++;
      else if (byte === BS) col = Math.max(col - 1, 0);
      else if (byte !== BEL) {
        const at = this.cell(col, row);
        this.cells[at] = byte;
        if (++col === COLS) [col, row] = [0, row + 1];
      }
      if (row === ROWS) {
        this.scroll(0, 0, COLS - 1, ROWS - 1, 1, DEFAULT_ATTR);
        row = ROWS - 1;
      }
      this.setCursor(col, row);
    }
  }
}
