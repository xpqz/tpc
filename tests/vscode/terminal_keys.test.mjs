// Terminal input to ReadKey bytes, checked against the bytes the browser path produces
// (host/keys.js keyBytes) for the same key. Each row is the xterm sequence and the key event.
import test from "node:test";
import assert from "node:assert/strict";
import { keyBytes } from "../../host/keys.js";
import { inputBytes } from "../../vscode/src/terminal_text.mjs";

const ESC = "\x1b";
const key = (name, modifiers = {}) => ({
  key: name, code: name, ctrlKey: false, altKey: false, shiftKey: false, ...modifiers,
});
const SHIFT = { shiftKey: true };
const ALT = { altKey: true };
const CTRL = { ctrlKey: true };

// xterm's modifier parameter: 2 shift, 3 alt, 4 shift+alt, 5 ctrl, 6 shift+ctrl, 7 alt+ctrl.
const MODIFIERS = [
  [2, SHIFT], [3, ALT], [4, { ...SHIFT, ...ALT }], [5, CTRL], [6, { ...SHIFT, ...CTRL }],
  [7, { ...ALT, ...CTRL }],
];

const rows = [];
const add = (sequence, event) => rows.push([sequence, event]);

// Arrows, Home and End: ESC [ X, ESC O X (application mode) and the modified ESC [ 1 ; m X.
const letterKeys = { A: "ArrowUp", B: "ArrowDown", C: "ArrowRight", D: "ArrowLeft", H: "Home", F: "End" };
for (const [letter, name] of Object.entries(letterKeys)) {
  add(`${ESC}[${letter}`, key(name));
  add(`${ESC}O${letter}`, key(name));
  for (const [parameter, modifiers] of MODIFIERS) add(`${ESC}[1;${parameter}${letter}`, key(name, modifiers));
}
// Insert, Delete, Home, End, PageUp, PageDown in the ESC [ n ~ form, plain and modified.
const tildeKeys = { 1: "Home", 2: "Insert", 3: "Delete", 4: "End", 5: "PageUp", 6: "PageDown" };
for (const [number, name] of Object.entries(tildeKeys)) {
  add(`${ESC}[${number}~`, key(name));
  for (const [parameter, modifiers] of MODIFIERS) add(`${ESC}[${number};${parameter}~`, key(name, modifiers));
}
// Function keys: F1-F4 are ESC O P..S (or ESC [ 1 ; m P..S modified), F5-F12 ESC [ n ~.
"PQRS".split("").forEach((letter, i) => {
  add(`${ESC}O${letter}`, key(`F${i + 1}`));
  add(`${ESC}[1${i + 1}~`, key(`F${i + 1}`));
  for (const [parameter, modifiers] of MODIFIERS) add(`${ESC}[1;${parameter}${letter}`, key(`F${i + 1}`, modifiers));
});
const functionNumbers = [15, 17, 18, 19, 20, 21, 23, 24];
functionNumbers.forEach((number, i) => {
  add(`${ESC}[${number}~`, key(`F${i + 5}`));
  for (const [parameter, modifiers] of MODIFIERS) add(`${ESC}[${number};${parameter}~`, key(`F${i + 5}`, modifiers));
});
add(`${ESC}[Z`, key("Tab", SHIFT));
// Control keys.
add("\r", key("Enter"));
add("\x7f", key("Backspace"));
add("\t", key("Tab"));
add(ESC, key("Escape"));
add(" ", key(" "));
// Ctrl+letter is the control code, Alt+letter is ESC and the letter (either case).
for (const letter of "abcdefghijklmnopqrstuvwxyz") {
  const code = `Key${letter.toUpperCase()}`;
  add(String.fromCharCode(letter.charCodeAt(0) - 96), { key: letter, code, ctrlKey: true, altKey: false, shiftKey: false });
  add(`${ESC}${letter}`, { key: letter, code, ctrlKey: false, altKey: true, shiftKey: false });
  add(`${ESC}${letter.toUpperCase()}`, { key: letter.toUpperCase(), code, ctrlKey: false, altKey: true, shiftKey: true });
}
// Printable characters of ASCII and of CP437.
for (const char of "aZ09!~{|}[]") add(char, key(char, { code: "x" }));
for (const char of "üÆ░┌ß") add(char, key(char, { code: "x" }));

test(`${rows.length} terminal sequences give the bytes the browser gives for the same key`, () => {
  assert.ok(rows.length > 250);
  for (const [sequence, event] of rows) {
    const expected = keyBytes(event);
    assert.ok(expected.length > 0, `keys.js knows ${JSON.stringify(event)}`);
    assert.deepEqual(inputBytes(sequence), expected, `${JSON.stringify(sequence)} is ${JSON.stringify(event)}`);
  }
});

test("function keys are the extended codes of Table C.2", () => {
  assert.deepEqual(inputBytes(`${ESC}OP`), [0, 59]);
  assert.deepEqual(inputBytes(`${ESC}[21~`), [0, 68]); // F10
  assert.deepEqual(inputBytes(`${ESC}[23~`), [0, 133]); // F11
  assert.deepEqual(inputBytes(`${ESC}[24~`), [0, 134]); // F12
  assert.deepEqual(inputBytes(`${ESC}[1;2P`), [0, 84]); // Shift+F1
  assert.deepEqual(inputBytes(`${ESC}[1;5P`), [0, 94]); // Ctrl+F1
  assert.deepEqual(inputBytes(`${ESC}[1;3P`), [0, 104]); // Alt+F1
  assert.deepEqual(inputBytes(`${ESC}[1;5C`), [0, 116]); // Ctrl+Right
  assert.deepEqual(inputBytes(`${ESC}[Z`), [0, 15]); // Shift+Tab
  assert.deepEqual(inputBytes(`${ESC}x`), [0, 45]); // Alt+X
  assert.deepEqual(inputBytes("\x01"), [1]); // Ctrl+A
});

test("sequences a DOS program never sees give no bytes and do not eat what follows", () => {
  for (const unknown of [`${ESC}[99~`, `${ESC}[1;9A`, `${ESC}[I`, `${ESC}[O`, `${ESC}[<0;5;5M`, `${ESC}[200~`, `${ESC}[?1;2c`]) {
    assert.deepEqual(inputBytes(`${unknown}k`), [107], JSON.stringify(unknown));
  }
  assert.deepEqual(inputBytes(`${ESC}[`), []);
  assert.deepEqual(inputBytes(`${ESC}[1;`), []);
});

test("Escape followed by something that is not a letter or a sequence is two keys", () => {
  assert.deepEqual(inputBytes(`${ESC}1`), [27, 49]);
  assert.deepEqual(inputBytes(`${ESC}${ESC}`), [27, 27]);
  assert.deepEqual(inputBytes(`${ESC}[A${ESC}[B`), [0, 72, 0, 80]);
  assert.deepEqual(inputBytes(`a${ESC}b\r`), [97, 0, 48, 13]);
});

test("a pasted mix keeps its order", () => {
  assert.deepEqual(inputBytes(`x${ESC}[A${ESC}OPy\r`), [120, 0, 72, 0, 59, 121, 13]);
});
