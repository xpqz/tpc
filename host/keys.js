// The key queue between the page and the program Worker: a ring of bytes in a
// SharedArrayBuffer. The page is the only producer (it owns TAIL), the program the only
// consumer (it owns HEAD); both only ever grow, a key lives in slot `count % SLOTS`.
import { CP437 } from "./terminal.js";

const HEAD = 0;
const TAIL = 1;
const FIRST_SLOT = 2;
const SLOTS = 64;

export const newKeyQueue = () => new Int32Array(new SharedArrayBuffer((FIRST_SLOT + SLOTS) * 4));

// Queues all of `bytes`, or none and false if they do not fit (a dropped scan code would
// leave the program with a lone 0).
export function pushKeys(queue, bytes) {
  const tail = Atomics.load(queue, TAIL);
  if (tail - Atomics.load(queue, HEAD) + bytes.length > SLOTS) return false;
  bytes.forEach((byte, i) => (queue[FIRST_SLOT + ((tail + i) % SLOTS)] = byte));
  Atomics.store(queue, TAIL, tail + bytes.length);
  Atomics.notify(queue, TAIL);
  return true;
}

// Crt's KeyPressed: 1 if a key waits, without taking it.
export const keyPressed = (queue) =>
  Atomics.load(queue, HEAD) !== Atomics.load(queue, TAIL) ? 1 : 0;

// Crt's ReadKey: sleeps until a key is queued, then takes it. Atomics.wait returns at once
// when TAIL no longer equals `head`, so a key pushed between the check and the wait is not
// missed.
export function readKey(queue) {
  const head = Atomics.load(queue, HEAD);
  while (Atomics.load(queue, TAIL) === head) Atomics.wait(queue, TAIL, head);
  const byte = queue[FIRST_SLOT + (head % SLOTS)];
  Atomics.store(queue, HEAD, head + 1);
  return byte;
}

// The bytes ReadKey returns for a key: one ASCII byte, or 0 and the extended code of the
// Reference Guide's Table C.2 (Appendix C, page 424). Letters are identified by `event.code`,
// the physical key, so Alt+X is the same chord on every layout (on macOS the Option key makes
// `event.key` a symbol).
const CONTROL = { Enter: 13, Backspace: 8, Tab: 9, Escape: 27 };
// Extended codes of the keys with no modifier; Ctrl changes some of them.
const EXTENDED = { ArrowUp: 72, ArrowLeft: 75, ArrowRight: 77, ArrowDown: 80, Home: 71, End: 79,
  PageUp: 73, PageDown: 81, Insert: 82, Delete: 83 };
const CTRL_EXTENDED = { ArrowLeft: 115, ArrowRight: 116, End: 117, PageDown: 118, Home: 119,
  PageUp: 132 };
// Alt+letter: the scan code of the key, row by row of the keyboard. Alt+digit (Table C.2 has
// them, KEYS.TBL too) is left out of the browser: with Option or AltGr the digit row types
// [ ] { } | on several layouts, and the IDE binds no Alt+digit.
const LETTER_ROWS = [["QWERTYUIOP", 16], ["ASDFGHJKL", 30], ["ZXCVBNM", 44]];
const FUNCTION_KEY = /^F([1-9]|1[0-2])$/;
const DEL = 128; // the first byte above ASCII

// Alt+letter is a chord unless the layout turned it into an ASCII symbol (Option+L is "@" on a
// German Mac): `key` then differs from the letter. On other layouts `key` is the letter, and
// where Option makes a non-ASCII symbol (Option+X is U+2248) the physical key decides.
function isAltChord({ key, code, altKey, ctrlKey }) {
  if (!altKey || ctrlKey || !/^Key[A-Z]$/.test(code)) return false;
  const isAscii = key.length === 1 && key.charCodeAt(0) < DEL;
  const typedSymbol = isAscii && key.toLowerCase() !== code[3].toLowerCase();
  return !typedSymbol;
}

// Extended code of F1..F12 with a modifier: F1-F10 are 59-68 plain, then 84, 94 and 104 for
// Shift, Ctrl and Alt; F11 and F12 are 133-134 and 135-140 in the same order.
function functionKeyCode(n, { shiftKey, ctrlKey, altKey }) {
  const level = altKey ? 3 : ctrlKey ? 2 : shiftKey ? 1 : 0;
  return n <= 10 ? [59, 84, 94, 104][level] + n - 1 : 133 + (n - 11) + 2 * level;
}

function altLetterCode(letter) {
  const [row, first] = LETTER_ROWS.find(([letters]) => letters.includes(letter));
  return first + row.indexOf(letter);
}

// The bytes ReadKey returns for a keydown event; none for keys a DOS program never sees.
export function keyBytes(event) {
  const { key, ctrlKey, altKey, shiftKey } = event;
  const fn = FUNCTION_KEY.exec(key);
  if (fn) return [0, functionKeyCode(Number(fn[1]), event)];
  if (key === "Tab" && shiftKey) return [0, 15];
  if (ctrlKey && Object.hasOwn(CTRL_EXTENDED, key)) return [0, CTRL_EXTENDED[key]];
  if (Object.hasOwn(CONTROL, key)) return [CONTROL[key]];
  if (Object.hasOwn(EXTENDED, key)) return [0, EXTENDED[key]];
  if (isAltChord(event)) return [0, altLetterCode(event.code[3])];
  if (key.length !== 1) return [];
  // Ctrl+Alt is AltGr on Windows: it types a character, so it is not a control code.
  if (ctrlKey && !altKey) return /[a-z]/i.test(key) ? [(key.charCodeAt(0) | 0x20) - 96] : [];
  const char = key.charCodeAt(0);
  if (char < DEL) return [char];
  const cp437 = CP437.indexOf(key, DEL);
  return cp437 < 0 ? [] : [cp437];
}
