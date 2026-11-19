// Canvas renderer for the 80x25 text screen. The screen is a byte array any thread can write
// (a SharedArrayBuffer view in the browser): a char and an attribute byte per cell, row by
// row, then the cursor column and row. An attribute is the foreground colour in bits 0-3,
// the background in bits 4-6 and the blink flag in bit 7, as in the PC text mode.
export const COLS = 80;
export const ROWS = 25;
export const CURSOR = COLS * ROWS * 2;
export const GRID_BYTES = CURSOR + 2;
export const DEFAULT_ATTR = 7;

// The glyph of every byte in code page 437: pictures for the control codes, ASCII, then the
// accented letters, box drawing and maths symbols of the high half.
export const CP437 =
  " ☺☻♥♦♣♠•◘○◙♂♀♪♫☼" + "►◄↕‼¶§▬↨↑↓→←∟↔▲▼" +
  String.fromCharCode(...Array.from({ length: 95 }, (_, i) => 32 + i)) + "⌂" +
  "ÇüéâäàåçêëèïîìÄÅ" + "ÉæÆôöòûùÿÖÜ¢£¥₧ƒ" + "áíóúñÑªº¿⌐¬½¼¡«»" + "░▒▓│┤╡╢╖╕╣║╗╝╜╛┐" +
  "└┴┬├─┼╞╟╚╔╩╦╠═╬╧" + "╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀" + "αßΓπΣσµτΦΘΩδ∞φε∩" + "≡±≥≤⌠⌡÷≈°∙·√ⁿ²■ ";

const PALETTE = [
  "#000000", "#0000aa", "#00aa00", "#00aaaa", "#aa0000", "#aa00aa", "#aa5500", "#aaaaaa",
  "#555555", "#5555ff", "#55ff55", "#55ffff", "#ff5555", "#ff55ff", "#ffff55", "#ffffff",
];
const CELL_W = 9;
const CELL_H = 16;
const CURSOR_H = 2;
const BLINK_MS = 500;

// Blanks the screen (light grey on black) and homes the cursor.
export function clearScreen(cells) {
  for (let i = 0; i < CURSOR; i += 2) {
    cells[i] = 0x20;
    cells[i + 1] = DEFAULT_ATTR;
  }
  cells[CURSOR] = cells[CURSOR + 1] = 0;
}

export class Terminal {
  constructor(canvas, cells) {
    canvas.width = COLS * CELL_W;
    canvas.height = ROWS * CELL_H;
    this.ctx = canvas.getContext("2d");
    this.ctx.font = `${CELL_H - 2}px monospace`;
    this.ctx.textBaseline = "top";
    this.cells = cells;
    this.shown = null; // the frame and blink phase last painted
    this.phase = false;
  }

  // Paints the screen if it differs from the last paint; call it every animation frame.
  draw(nowMs) {
    const phase = Math.floor(nowMs / BLINK_MS) % 2 === 0;
    const frame = this.cells.slice(); // a private copy: the program may write meanwhile
    if (this.shown && phase === this.phase && frame.every((b, i) => b === this.shown[i])) return;
    for (let row = 0; row < ROWS; row++) {
      for (let col = 0; col < COLS; col++) this.drawCell(frame, row, col, phase);
    }
    if (phase) this.drawCursor(frame);
    this.shown = frame;
    this.phase = phase;
  }

  drawCell(frame, row, col, phase) {
    const at = (row * COLS + col) * 2;
    const attr = frame[at + 1];
    const [x, y] = [col * CELL_W, row * CELL_H];
    this.ctx.fillStyle = PALETTE[(attr >> 4) & 7];
    this.ctx.fillRect(x, y, CELL_W, CELL_H);
    if (attr & 0x80 && !phase) return; // blinking text is hidden in the off phase
    this.ctx.fillStyle = PALETTE[attr & 15];
    this.ctx.fillText(CP437[frame[at]], x, y);
  }

  // The cursor is a bar along the bottom of its cell, in the cell's foreground colour.
  drawCursor(frame) {
    const [col, row] = [frame[CURSOR], frame[CURSOR + 1]];
    this.ctx.fillStyle = PALETTE[frame[(row * COLS + col) * 2 + 1] & 15];
    this.ctx.fillRect(col * CELL_W, (row + 1) * CELL_H - CURSOR_H, CELL_W, CURSOR_H);
  }

  // Repaints on every animation frame until the page goes away.
  start() {
    const tick = (now) => {
      this.draw(now);
      requestAnimationFrame(tick);
    };
    requestAnimationFrame(tick);
  }
}
