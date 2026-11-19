// Draft of the chord-aware replacement for keyBytes in host/keys.js (decision 41). The bytes
// are what ReadKey returns for a key: one ASCII byte, or 0 and the extended code of the
// Reference Guide's Table C.2 (Appendix C, page 424). Letters and digits are identified by
// `event.code`, the physical key, so Alt+X is the same chord on every layout (on macOS the
// Option key makes `event.key` a symbol).
import { CP437 } from "../../../host/terminal.js";

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
  const typedSymbol = key.length === 1 && key.charCodeAt(0) < DEL && key.toLowerCase() !== code[3].toLowerCase();
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
