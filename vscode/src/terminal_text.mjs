// Terminal text and key bytes. A program writes CP437 bytes with LF line ends (or CR LF) and
// reads ReadKey bytes; a terminal shows Unicode with CR LF line ends and types characters and
// escape sequences.
import { CP437 } from "../../host/terminal.js";
import { keyBytes } from "../../host/keys.js";

const FIRST_NON_ASCII = 128;
const CR = 13;
const LF = 10;

// Returns decode(bytes) -> string for one program's output. A terminal's LF only moves down,
// so a LF not already preceded by CR gets one; the previous byte is kept between chunks.
export function createOutputDecoder() {
  let previous = 0;
  return (bytes) => {
    let text = "";
    for (const byte of bytes) {
      if (byte === LF && previous !== CR) text += "\r";
      text += byte < FIRST_NON_ASCII ? String.fromCharCode(byte) : CP437[byte];
      previous = byte;
    }
    return text;
  };
}

const ESCAPE = "\x1b";
const BACKSPACE = 8;
const DELETE_KEY = 127; // what a terminal sends for the Backspace key

// xterm's final characters and numbers, as the key a browser would report. The bytes then come
// from keyBytes (host/keys.js), so the terminal and the browser cannot disagree.
const LETTER_KEYS = {
  A: "ArrowUp", B: "ArrowDown", C: "ArrowRight", D: "ArrowLeft", H: "Home", F: "End",
  P: "F1", Q: "F2", R: "F3", S: "F4", Z: "Tab",
};
const TILDE_KEYS = {
  1: "Home", 2: "Insert", 3: "Delete", 4: "End", 5: "PageUp", 6: "PageDown",
  11: "F1", 12: "F2", 13: "F3", 14: "F4", 15: "F5", 17: "F6", 18: "F7", 19: "F8",
  20: "F9", 21: "F10", 23: "F11", 24: "F12",
};
// A sequence body is optional digits, an optional `;modifier` and the final character:
// `A`, `15~`, `1;5C`.
const SEQUENCE_BODY = /^(\d+)?(?:;(\d+))?([A-Za-z~])$/;

// xterm's modifier parameter is 1 plus a bit set: 1 shift, 2 alt, 4 ctrl. Anything else
// (meta, an unknown value) is not a key DOS knows.
function modifiersOf(parameter) {
  const bits = parameter === undefined ? 0 : Number(parameter) - 1;
  if (bits < 0 || bits > 7) return undefined;
  return { shiftKey: (bits & 1) !== 0, altKey: (bits & 2) !== 0, ctrlKey: (bits & 4) !== 0 };
}

// The key event of a sequence body, or undefined when it is not a key DOS knows.
function keyOfSequence(body) {
  const match = SEQUENCE_BODY.exec(body);
  const modifiers = match && modifiersOf(match[2]);
  if (!modifiers) return undefined;
  const [, number, , final] = match;
  const key = final === "~" ? TILDE_KEYS[number] : LETTER_KEYS[final];
  const numberFits = final === "~" ? key !== undefined : number === undefined || number === "1";
  if (key === undefined || !numberFits) return undefined;
  return { key, code: key, ...modifiers, ...(final === "Z" ? { shiftKey: true } : {}) };
}

// The index just past the escape sequence that starts at `start` (ESC then [ or O, parameter
// characters, and one final character from @ to ~), and its text without the ESC and the [.
function readEscape(data, start) {
  let end = start + 2;
  while (end < data.length && !/[@-~]/.test(data[end])) end++;
  return { end: end + 1, body: data.slice(start + 2, end + 1) };
}

// Alt+letter arrives as ESC and the letter.
function altLetterBytes(letter) {
  return keyBytes({ key: letter, code: `Key${letter.toUpperCase()}`, altKey: true });
}

// The ReadKey bytes for text typed into the terminal. Keys a DOS program never sees give none.
export function inputBytes(data) {
  const bytes = [];
  let i = 0;
  while (i < data.length) {
    const char = data[i];
    const next = data[i + 1];
    // ESC O ends the data only when the user typed Alt+Shift+O; otherwise it starts F1-F4.
    const startsSequence = next === "[" || (next === "O" && i + 2 < data.length);
    if (char === ESCAPE && startsSequence) {
      const { end, body } = readEscape(data, i);
      const event = keyOfSequence(body);
      if (event) bytes.push(...keyBytes(event));
      i = end;
      continue;
    }
    if (char === ESCAPE && next !== undefined && /[a-zA-Z]/.test(next)) {
      bytes.push(...altLetterBytes(next));
      i += 2;
      continue;
    }
    i++;
    const code = char.charCodeAt(0);
    if (code === DELETE_KEY) bytes.push(BACKSPACE);
    else if (code < FIRST_NON_ASCII) bytes.push(code);
    else if (CP437.indexOf(char, FIRST_NON_ASCII) >= 0) bytes.push(CP437.indexOf(char, FIRST_NON_ASCII));
  }
  return bytes;
}
