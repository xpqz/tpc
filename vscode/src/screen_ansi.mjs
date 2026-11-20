// The Crt screen as terminal text. The program draws on a grid (the layout of host/terminal.js:
// a character and an attribute byte per cell, then the cursor column and row). A frame is the
// ANSI text that makes the terminal show what changed since the previous frame.
import { COLS, ROWS, CURSOR, CP437, cellOffset } from "./host.mjs";

const ESC = "\x1b";
const HIDE_CURSOR = `${ESC}[?25l`;
const SHOW_CURSOR = `${ESC}[?25h`;
const CLEAR_SCREEN = `${ESC}[2J`;

// The alternate screen keeps the user's terminal history untouched; leaving it also resets the
// colours and shows the cursor, whatever state the last frame left.
export const ENTER_SCREEN = `${ESC}[?1049h`;
export const LEAVE_SCREEN = `${ESC}[?1049l${ESC}[0m${SHOW_CURSOR}`;

// DOS numbers colours blue = 1, green = 2, red = 4; ANSI numbers red = 1, green = 2, blue = 4.
// Swapping bits 0 and 2 converts either way.
function swapRedAndBlue(colour) {
  return ((colour & 1) << 2) | (colour & 2) | ((colour & 4) >> 2);
}

// The text attribute of a cell: foreground in bits 0-3, background in bits 4-6, blink in bit 7.
// Bit 3 (high intensity) uses the bright foreground codes 90-97. Blink is shown as a bright
// background (100-107) instead of the terminal's blink, which is rarely wanted. Each code
// starts with a reset, so a frame never depends on the colours an earlier one left.
export function attributeCode(attr) {
  const foreground = (attr & 8 ? 90 : 30) + swapRedAndBlue(attr & 7);
  const background = (attr & 0x80 ? 100 : 40) + swapRedAndBlue((attr >> 4) & 7);
  return `${ESC}[0;${foreground};${background}m`;
}

const moveTo = (col, row) => `${ESC}[${row + 1};${col + 1}H`;

const sameCell = (a, b, offset) => a[offset] === b[offset] && a[offset + 1] === b[offset + 1];

// A writer remembers the grid it last drew. `frame(grid)` returns the text for the change
// since then ("" for none). `resize(columns, rows)` tells it the terminal's size: below 80x25
// it shows a notice instead of the picture, and after any size change the next frame is a full
// redraw, because the terminal may have rearranged its contents.
export function createScreenWriter() {
  let drawn = null; // copy of the grid on the terminal; null when everything must be drawn
  let columns = COLS;
  let rows = ROWS;
  let noticeShown = false;

  const tooSmall = () => columns < COLS || rows < ROWS;

  function notice() {
    noticeShown = true;
    drawn = null;
    const text = `Please enlarge the terminal to at least ${COLS}x${ROWS} (now ${columns}x${rows}).`;
    return `${CLEAR_SCREEN}${moveTo(0, 0)}${text}`;
  }

  // The cells that differ from `drawn`, as moves, colour changes and glyphs. A move is needed
  // only where the cell does not follow the previous one; after the last column the terminal
  // would wrap, so the next cell always gets a move.
  function changedCells(grid) {
    let text = "";
    let attributeOnTerminal = -1;
    let cursorAt = -1; // offset the terminal cursor is at, -1 when unknown
    for (let row = 0; row < ROWS; row++) {
      for (let col = 0; col < COLS; col++) {
        const offset = cellOffset(col, row);
        if (drawn !== null && sameCell(drawn, grid, offset)) continue;
        if (cursorAt !== offset) text += moveTo(col, row);
        if (grid[offset + 1] !== attributeOnTerminal) {
          attributeOnTerminal = grid[offset + 1];
          text += attributeCode(attributeOnTerminal);
        }
        text += CP437[grid[offset]];
        cursorAt = col === COLS - 1 ? -1 : offset + 2;
      }
    }
    return text;
  }

  const cursorMove = (grid) => moveTo(grid[CURSOR], grid[CURSOR + 1]);

  return {
    resize(newColumns, newRows) {
      if (newColumns === columns && newRows === rows) return;
      [columns, rows] = [newColumns, newRows];
      noticeShown = false;
      drawn = null;
    },

    // The program keeps writing the grid while a frame is made, so the frame works on a copy:
    // what is drawn and what is remembered as drawn are then the same cells.
    frame(sharedGrid) {
      if (tooSmall()) return noticeShown ? "" : notice();
      const grid = sharedGrid.slice();
      const redraw = drawn === null;
      const cells = changedCells(grid);
      const cursorMoved = drawn === null || !sameCell(drawn, grid, CURSOR);
      drawn = grid;
      if (cells === "") return cursorMoved ? cursorMove(grid) : "";
      const clear = redraw ? CLEAR_SCREEN : "";
      return `${HIDE_CURSOR}${clear}${cells}${cursorMove(grid)}${SHOW_CURSOR}`;
    },
  };
}
