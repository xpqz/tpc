// Terminal text and key bytes. A program writes CP437 bytes with LF line ends (or CR LF) and
// reads ReadKey bytes; a terminal shows Unicode with CR LF line ends and types characters and
// escape sequences.
import { CP437 } from "../../host/terminal.js";

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

// The extended ReadKey code (after a 0 byte) of the final part of a terminal escape sequence:
// ESC [ A is the up arrow, ESC [ 3 ~ is Delete.
const EXTENDED_CODES = {
  A: 72, B: 80, C: 77, D: 75, H: 71, F: 79,
  "1~": 71, "2~": 82, "3~": 83, "4~": 79, "5~": 73, "6~": 81,
};

const ESCAPE = "\x1b";
const BACKSPACE = 8;
const DELETE_KEY = 127; // what a terminal sends for the Backspace key

// The index just past the escape sequence that starts at `start` (ESC then [ or O, parameter
// characters, and one final character from @ to ~), and its text without the ESC and the [.
function readEscape(data, start) {
  let end = start + 2;
  while (end < data.length && !/[@-~]/.test(data[end])) end++;
  return { end: end + 1, body: data.slice(start + 2, end + 1) };
}

// The ReadKey bytes for text typed into the terminal. Keys a DOS program never sees give none.
export function inputBytes(data) {
  const bytes = [];
  let i = 0;
  while (i < data.length) {
    const char = data[i];
    const next = data[i + 1];
    if (char === ESCAPE && (next === "[" || next === "O")) {
      const { end, body } = readEscape(data, i);
      if (Object.hasOwn(EXTENDED_CODES, body)) bytes.push(0, EXTENDED_CODES[body]);
      i = end;
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
