import test from "node:test";
import assert from "node:assert/strict";
import { createOutputDecoder, inputBytes } from "../../vscode/src/terminal_text.mjs";

const bytes = (...values) => Uint8Array.from(values);
const ascii = (text) => Uint8Array.from(text, (c) => c.charCodeAt(0));

test("a lone LF becomes CR LF", () => {
  const decode = createOutputDecoder();
  assert.equal(decode(ascii("a\nb\n")), "a\r\nb\r\n");
});

test("CR LF stays CR LF, also when the two bytes arrive in different chunks", () => {
  const decode = createOutputDecoder();
  assert.equal(decode(ascii("a\r\nb")), "a\r\nb");
  assert.equal(decode(ascii("c\r")), "c\r");
  assert.equal(decode(ascii("\nd")), "\nd");
  assert.equal(decode(ascii("\n")), "\r\n", "the state is the byte before, not the chunk start");
});

test("an empty chunk gives an empty string and keeps the state", () => {
  const decode = createOutputDecoder();
  decode(ascii("x\r"));
  assert.equal(decode(new Uint8Array(0)), "");
  assert.equal(decode(ascii("\n")), "\n");
});

test("ASCII and the controls a terminal acts on pass through", () => {
  const decode = createOutputDecoder();
  assert.equal(decode(ascii("ab \b\x07\t~")), "ab \b\x07\t~");
});

test("bytes above 127 are CP437 characters", () => {
  const decode = createOutputDecoder();
  assert.equal(decode(bytes(129, 176, 218, 0xe1, 0xff)), "ü░┌ß ");
});

test("typing: Enter is 13, both Backspace codes are 8, printable ASCII is itself", () => {
  assert.deepEqual(inputBytes("\r"), [13]);
  assert.deepEqual(inputBytes("\x7f"), [8]);
  assert.deepEqual(inputBytes("\b"), [8]);
  assert.deepEqual(inputBytes("Ab 9~"), [65, 98, 32, 57, 126]);
});

test("typing: a character that is in CP437 gives its byte, others none", () => {
  assert.deepEqual(inputBytes("ü"), [129]);
  assert.deepEqual(inputBytes("中"), []);
});

test("typing: arrows and Delete are 0 and the extended code, other escape sequences vanish", () => {
  assert.deepEqual(inputBytes("\x1b[A"), [0, 72]);
  assert.deepEqual(inputBytes("\x1b[B"), [0, 80]);
  assert.deepEqual(inputBytes("\x1b[C"), [0, 77]);
  assert.deepEqual(inputBytes("\x1b[D"), [0, 75]);
  assert.deepEqual(inputBytes("\x1b[3~"), [0, 83]);
  assert.deepEqual(inputBytes("\x1b[99~x"), [120]);
});

test("typing: a pasted run keeps its order and a lone Escape is 27", () => {
  assert.deepEqual(inputBytes("ab\rc"), [97, 98, 13, 99]);
  assert.deepEqual(inputBytes("\x1b"), [27]);
  assert.deepEqual(inputBytes(""), []);
});
