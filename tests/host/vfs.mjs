// Checks tests/vfs.mjs against the contract in docs/abi.md section 8. Exit 0 on success.
import assert from "node:assert/strict";
import fs from "node:fs";
import { Vfs, Input, RESET, REWRITE, APPEND } from "../vfs.mjs";

const aux = new URL("fixtures/vfs/", import.meta.url).pathname;
const text = (b) => b.toString("latin1");
const NOT_FOUND = -2;
const PATH_NOT_FOUND = -3;
const TOO_MANY = -4;
const DENIED = -5;
const BAD_HANDLE = -6;

const tests = [];
const test = (name, body) => tests.push([name, body]);

// Reads the whole file `name` through the VFS; returns null if it cannot be opened.
function slurp(vfs, name) {
  const h = vfs.open(name, RESET);
  if (h < 0) return null;
  const parts = [];
  for (;;) {
    const chunk = vfs.read(h, 7);
    assert.ok(Buffer.isBuffer(chunk), `read returned ${chunk}`);
    if (chunk.length === 0) break;
    parts.push(chunk);
  }
  assert.equal(vfs.close(h), 0);
  return Buffer.concat(parts);
}

test("modes are 0 Reset, 1 Rewrite, 2 Append", () => {
  assert.deepEqual([RESET, REWRITE, APPEND], [0, 1, 2]);
});

test("Reset reads an aux file byte for byte, in chunks, then 0 at end of file", () => {
  const vfs = new Vfs(aux);
  assert.equal(text(slurp(vfs, "DATA.TXT")), "alpha\r\nbeta\r\n");
  const h = vfs.open("DATA.TXT", RESET);
  assert.ok(h > 0, `handle ${h}`);
  assert.equal(vfs.read(h, 1000).length, 13);
  assert.equal(vfs.read(h, 1000).length, 0);
  assert.equal(vfs.read(h, 1000).length, 0);
});

test("an empty file reads as end of file at once", () => {
  const vfs = new Vfs(aux);
  const h = vfs.open("EMPTY.TXT", RESET);
  assert.ok(h > 0);
  assert.equal(vfs.read(h, 10).length, 0);
});

test("a zero-length read returns 0 bytes and does not move the position", () => {
  const vfs = new Vfs(aux);
  const h = vfs.open("ONE.TXT", RESET);
  assert.equal(vfs.read(h, 0).length, 0);
  assert.equal(text(vfs.read(h, 5)), "x");
});

test("file names are case-insensitive", () => {
  const vfs = new Vfs(aux);
  for (const name of ["data.txt", "Data.Txt", "DATA.TXT"]) {
    assert.equal(text(slurp(vfs, name)), "alpha\r\nbeta\r\n", name);
  }
});

test("Reset of a missing file returns -2", () => {
  const vfs = new Vfs(aux);
  assert.equal(vfs.open("NOPE.TXT", RESET), NOT_FOUND);
});

test("no aux directory: an empty file system", () => {
  const vfs = new Vfs(null);
  assert.equal(vfs.open("DATA.TXT", RESET), NOT_FOUND);
  const h = vfs.open("NEW.TXT", REWRITE);
  assert.ok(h > 0);
  assert.equal(vfs.write(h, Buffer.from("hi")), 2);
  assert.equal(vfs.close(h), 0);
  assert.equal(text(slurp(vfs, "NEW.TXT")), "hi");
});

test("Rewrite creates a file; what was written reads back, case-insensitively", () => {
  const vfs = new Vfs(aux);
  const h = vfs.open("Out.Txt", REWRITE);
  assert.ok(h > 0);
  assert.equal(vfs.write(h, Buffer.from("one\r\n")), 5);
  assert.equal(vfs.write(h, Buffer.from("two")), 3);
  assert.equal(vfs.write(h, Buffer.alloc(0)), 0);
  assert.equal(vfs.close(h), 0);
  assert.equal(text(slurp(vfs, "OUT.TXT")), "one\r\ntwo");
});

test("Rewrite truncates an existing file at open", () => {
  const vfs = new Vfs(aux);
  const h = vfs.open("DATA.TXT", REWRITE);
  assert.equal(vfs.close(h), 0);
  assert.equal(slurp(vfs, "DATA.TXT").length, 0);
});

test("Append writes after the existing contents", () => {
  const vfs = new Vfs(aux);
  const h = vfs.open("DATA.TXT", APPEND);
  assert.ok(h > 0);
  assert.equal(vfs.write(h, Buffer.from("gamma\r\n")), 7);
  assert.equal(vfs.close(h), 0);
  assert.equal(text(slurp(vfs, "DATA.TXT")), "alpha\r\nbeta\r\ngamma\r\n");
});

test("Append to a missing file creates it", () => {
  const vfs = new Vfs(aux);
  const h = vfs.open("LOG.TXT", APPEND);
  assert.ok(h > 0);
  vfs.write(h, Buffer.from("a"));
  vfs.close(h);
  assert.equal(text(slurp(vfs, "LOG.TXT")), "a");
});

test("a program's files never reach the aux directory or the next run", () => {
  const before = fs.readdirSync(aux).sort();
  const dataBefore = fs.readFileSync(aux + "DATA.TXT");
  const vfs = new Vfs(aux);
  vfs.close(vfs.open("NEW.TXT", REWRITE));
  const h = vfs.open("DATA.TXT", APPEND);
  vfs.write(h, Buffer.from("zzz"));
  vfs.close(h);
  assert.deepEqual(fs.readdirSync(aux).sort(), before);
  assert.deepEqual(fs.readFileSync(aux + "DATA.TXT"), dataBefore);
  const next = new Vfs(aux);
  assert.equal(next.open("NEW.TXT", RESET), NOT_FOUND);
  assert.equal(text(slurp(next, "DATA.TXT")), "alpha\r\nbeta\r\n");
});

test("two handles on one file have independent positions", () => {
  const vfs = new Vfs(aux);
  const a = vfs.open("DATA.TXT", RESET);
  const b = vfs.open("DATA.TXT", RESET);
  assert.notEqual(a, b);
  assert.equal(text(vfs.read(a, 5)), "alpha");
  assert.equal(text(vfs.read(b, 3)), "alp");
  assert.equal(text(vfs.read(a, 2)), "\r\n");
});

test("a file written through one handle is visible to a later Reset before close", () => {
  const vfs = new Vfs(null);
  const w = vfs.open("F.TXT", REWRITE);
  vfs.write(w, Buffer.from("abc"));
  assert.equal(text(slurp(vfs, "F.TXT")), "abc");
});

test("bytes are raw: every value 0..255 round-trips", () => {
  const vfs = new Vfs(null);
  const all = Buffer.from(Array.from({ length: 256 }, (_, i) => i));
  const h = vfs.open("BIN.DAT", REWRITE);
  assert.equal(vfs.write(h, all), 256);
  vfs.close(h);
  assert.deepEqual(slurp(vfs, "BIN.DAT"), all);
});

test("close, read, write on an unknown or closed handle return -6", () => {
  const vfs = new Vfs(aux);
  assert.equal(vfs.close(0), BAD_HANDLE);
  assert.equal(vfs.close(99), BAD_HANDLE);
  assert.equal(vfs.read(99, 1), BAD_HANDLE);
  assert.equal(vfs.write(99, Buffer.from("a")), BAD_HANDLE);
  const h = vfs.open("DATA.TXT", RESET);
  assert.equal(vfs.close(h), 0);
  assert.equal(vfs.close(h), BAD_HANDLE);
  assert.equal(vfs.read(h, 1), BAD_HANDLE);
});

test("reading a write handle and writing a read handle return -5", () => {
  const vfs = new Vfs(aux);
  const r = vfs.open("DATA.TXT", RESET);
  assert.equal(vfs.write(r, Buffer.from("x")), DENIED);
  for (const mode of [REWRITE, APPEND]) {
    const w = vfs.open("W.TXT", mode);
    assert.equal(vfs.read(w, 1), DENIED);
  }
  assert.equal(text(slurp(vfs, "DATA.TXT")), "alpha\r\nbeta\r\n");
});

test("valid 8.3 names are accepted, invalid ones return -2 in every mode", () => {
  const vfs = new Vfs(null);
  for (const name of ["A", "ABCDEFGH", "A.B", "ABCDEFGH.XYZ", "A_1$~!#.%&-", "{A}(B)@'.^T", "NAME."]) {
    for (const mode of [REWRITE, APPEND]) {
      const h = vfs.open(name, mode);
      assert.ok(h > 0, `${name} mode ${mode}: ${h}`);
      vfs.close(h);
    }
  }
  for (const name of ["", "ABCDEFGHI", "ABCDEFGHI.TXT", "A.TXTX", "A.B.C", ".TXT", "A B", "A*.TXT", "A?", "A,B", "A+B", "Aé"]) {
    for (const mode of [RESET, REWRITE, APPEND]) {
      assert.equal(vfs.open(name, mode), NOT_FOUND, `${JSON.stringify(name)} mode ${mode}`);
    }
  }
});

test("a name with a directory part returns -3 and never touches the host file system", () => {
  const vfs = new Vfs(aux);
  for (const name of ["SUB\\A.TXT", "SUB/A.TXT", "C:A.TXT", "..\\X", "\\A.TXT", "/etc/passwd"]) {
    for (const mode of [RESET, REWRITE, APPEND]) {
      assert.equal(vfs.open(name, mode), PATH_NOT_FOUND, `${name} mode ${mode}`);
    }
  }
});

test("an unknown mode returns -5", () => {
  const vfs = new Vfs(aux);
  assert.equal(vfs.open("DATA.TXT", 3), DENIED);
  assert.equal(vfs.open("DATA.TXT", -1), DENIED);
});

test("at most 20 files are open at once; a closed handle can be reused", () => {
  const vfs = new Vfs(aux);
  const handles = [];
  for (let i = 0; i < 20; i++) {
    const h = vfs.open("DATA.TXT", RESET);
    assert.ok(h > 0, `open #${i}: ${h}`);
    handles.push(h);
  }
  assert.equal(new Set(handles).size, 20);
  assert.equal(vfs.open("DATA.TXT", RESET), TOO_MANY);
  assert.equal(vfs.close(handles[5]), 0);
  assert.ok(vfs.open("DATA.TXT", RESET) > 0);
});

test("subdirectories of the aux directory are not files", () => {
  const vfs = new Vfs(new URL("fixtures/", import.meta.url).pathname);
  assert.equal(vfs.open("VFS", RESET), NOT_FOUND);
});

test("a missing aux directory is a runner error", () => {
  assert.throws(() => new Vfs(aux + "NO_SUCH_DIR"), /NO_SUCH_DIR/);
});

// Console input.
const lines = (buf, max = 255) => {
  const input = new Input(buf);
  const got = [];
  for (let l = input.readLine(max); l !== null; l = input.readLine(max)) got.push(text(l));
  return got;
};

test("readLine splits at LF and CRLF and strips the terminator", () => {
  assert.deepEqual(lines(Buffer.from("a\nb\r\nc\n")), ["a", "b", "c"]);
});

test("readLine keeps empty lines and a last line without terminator", () => {
  assert.deepEqual(lines(Buffer.from("\n\nx\n\ny")), ["", "", "x", "", "y"]);
});

test("empty input has no lines; a lone newline is one empty line", () => {
  assert.deepEqual(lines(Buffer.alloc(0)), []);
  assert.deepEqual(lines(Buffer.from("\n")), [""]);
});

test("a CR that is not before LF stays in the line", () => {
  assert.deepEqual(lines(Buffer.from("a\rb\n")), ["a\rb"]);
});

test("a line longer than max is cut to max bytes and the rest is discarded", () => {
  assert.deepEqual(lines(Buffer.from("abcdefgh\nxy\n"), 3), ["abc", "xy"]);
  assert.deepEqual(lines(Buffer.from("abc\n"), 3), ["abc"]);
  assert.deepEqual(lines(Buffer.from("abc\nd\n"), 0), ["", ""]);
});

test("high-bit bytes pass through unchanged", () => {
  const got = new Input(Buffer.from([0xe9, 0xff, 0x0a])).readLine(10);
  assert.deepEqual([...got], [0xe9, 0xff]);
});

test("readLine keeps returning null at end of input", () => {
  const input = new Input(Buffer.from("a\n"));
  assert.ok(input.readLine(9) !== null);
  assert.equal(input.readLine(9), null);
  assert.equal(input.readLine(9), null);
});

let failed = 0;
for (const [name, body] of tests) {
  try {
    body();
    console.log(`ok   ${name}`);
  } catch (e) {
    failed++;
    console.log(`FAIL ${name}: ${e.message.split("\n")[0]}`);
  }
}
process.exitCode = failed === 0 ? 0 : 1;
