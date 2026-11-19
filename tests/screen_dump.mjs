// The screen dump of a Screen's cells, in the format tools/crt/SCRDUMP.PAS writes on the real
// Turbo Pascal 5.0 (docs/abi.md section 8): the 25 rows without trailing blanks, the cursor
// (1-based column and row), the exit code, and one `attr ROW COL LEN HEX` line per run of
// cells whose attribute is not 07.
import { COLS, ROWS, CURSOR, DEFAULT_ATTR } from "../host/terminal.js";

const charAt = (cells, col, row) => cells[(row * COLS + col) * 2];
const attrAt = (cells, col, row) => cells[(row * COLS + col) * 2 + 1];

function textRows(cells) {
  const rows = [];
  for (let row = 0; row < ROWS; row++) {
    const chars = Array.from({ length: COLS }, (_, col) => charAt(cells, col, row));
    rows.push(String.fromCharCode(...chars).replace(/ +$/, ""));
  }
  return rows;
}

function attributeRuns(cells) {
  const runs = [];
  for (let row = 0; row < ROWS; row++) {
    let col = 0;
    while (col < COLS) {
      const attr = attrAt(cells, col, row);
      const start = col;
      while (col < COLS && attrAt(cells, col, row) === attr) col++;
      if (attr !== DEFAULT_ATTR) {
        const hex = attr.toString(16).toUpperCase().padStart(2, "0");
        runs.push(`attr ${row + 1} ${start + 1} ${col - start} ${hex}`);
      }
    }
  }
  return runs;
}

export function dump(cells, exitCode) {
  const cursor = `cursor ${cells[CURSOR] + 1} ${cells[CURSOR + 1] + 1}`;
  const lines = [...textRows(cells), cursor, `exit ${exitCode}`, ...attributeRuns(cells)];
  return lines.map((line) => `${line}\n`).join("");
}
