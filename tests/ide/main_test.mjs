// Tests the whole IDE (ide/ide.pas, built here by the real compiler.wasm) against the real
// compiler.wasm that it calls through the host: compile errors in the edited file and in a
// unit, a real run on the user screen, and the exit test of the plan (open Snake, edit a line,
// compile, run, introduce a syntax error, see the cursor jump, fix it, run again) without a
// browser. The IDE's own behaviour with the canned compiler is tested by the corpus programs
// `ide_main_*` against screens from the real TP 5.0; this file adds what only the real
// compiler can answer. Expected error numbers and lines come from tools/tp5.sh:
//   MISSEMI.PAS(5): Error 85: ";" expected.     (the fixture)
//   P.PAS(41): Error 85: ";" expected.          (Snake with the `;` of line 40 removed)
//   U.PAS(7): Error 3: Unknown identifier.      (tests/ide/oracle/error_in_unit)
// TP reports a line only; the columns below are this compiler's (the token that is wrong).
// Usage: node tests/ide/main_test.mjs [compiler.wasm]   (default: the release build of
// `make wasm`).
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { execFileSync } from "node:child_process";
import { run, loadCompiler } from "../run.mjs";
import { runTests } from "../host/browser_helpers.mjs";

const root = new URL("../../", import.meta.url).pathname;
const target = execFileSync("sh", ["scripts/cargo-target.sh"], { cwd: root, encoding: "utf8" }).trim();
const wasmPath = process.argv[2] ?? `${target}/wasm32-unknown-unknown/release/tp5c.wasm`;
const read = (...parts) => fs.readFileSync(path.join(root, ...parts));

// The IDE: ide/ide.pas with the units of ide/. A second compiler instance builds it, so that
// nothing it registers is seen by the one the IDE calls through the host.
const sources = new Map(
  fs.readdirSync(path.join(root, "ide")).map((name) => [name, read("ide", name)]));
assert.ok(sources.has("ide.pas"), "ide/ide.pas, the IDE program, does not exist");
const build = (await loadCompiler(wasmPath)).compile("IDE.PAS", sources.get("ide.pas"), sources);
assert.ok(build.module, `the IDE does not compile: ${JSON.stringify(build)}`);
const compiler = await loadCompiler(wasmPath);

const fixtures = (...names) =>
  new Map(names.map((n) => [n, read("tests/ide/main_fixtures", n)]));
const guests = (...names) => new Map(names.map((n) => [n, read("tests/ide/guests", n)]));
const snake = () => new Map([["SNAKE.PAS", read("tests/programs/snake.pas")]]);

async function ide(keys, files) {
  const { stdout, code } = await run(build.module, {
    files, keys: Buffer.from(keys), compiler });
  const dump = stdout.toString("latin1");
  const rows = dump.split("\n");
  return { dump, rows, code, status: rows[24], cursor: rows[25] };
}

const type = (text) => [...text].map((c) => (c === " " ? "Space" : c)).join(" ");
const open = (name) => `F3 ${type(name.toLowerCase())} Enter`;
const times = (count, key) => Array(count).fill(key).join(" ");
const tests = [];
const test = (name, body) => tests.push([name, body]);

test("an error in a unit is named with its file, line and column; the cursor stays", async () => {
  const files = fixtures("C.PAS", "U.PAS");
  const got = await ide(`${open("c.pas")} F9`, files);
  assert.equal(got.code, 255, "the IDE waits for the next key");
  assert.match(got.status, /^ 1:1 +Insert +U\.PAS 7:11 Error 3: Unknown identifier$/);
  assert.equal(got.cursor, "cursor 1 2", "nothing of U.PAS is in the buffer to jump to");
});

test("a syntax error in the edited file: the message and the cursor on the error", async () => {
  const got = await ide(`${open("missemi.pas")} F9`, fixtures("MISSEMI.PAS"));
  assert.match(got.status, /^ 5:3 +Insert +Error 85: ";" expected$/);
  assert.equal(got.cursor, "cursor 3 6", "line 5 of the file is row 6 of the screen");
});

test("a good program compiles: the message, the cursor where it was", async () => {
  const got = await ide(`${open("hello.pas")} Right F9`, guests("HELLO.PAS"));
  assert.match(got.status, /^ 1:2 +Insert +Compiled$/);
});

test("Ctrl-F9 runs the program on the user screen and waits for a key", async () => {
  const got = await ide(`${open("hello.pas")} Ctrl-F9`, guests("HELLO.PAS"));
  assert.equal(got.code, 255);
  assert.equal(got.rows[0], "", "the IDE's own screen was wiped for the program");
  assert.equal(got.rows[2], "    guest was here");
  assert.equal(got.status, "Exit code 0 - press a key");
});

test("a key ends the user screen: the IDE is back with its file name and no message", async () => {
  const got = await ide(`${open("hello.pas")} Ctrl-F9 x Alt-X`, guests("HELLO.PAS"));
  assert.equal(got.code, 0);
  assert.match(got.rows[0], /^ File  Run  Compile  Options +HELLO\.PAS$/);
  assert.equal(got.rows[1], "program Hello;");
  assert.equal(got.status, " 1:1         Insert");
});

test("a runtime error ends the run with its number on the prompt", async () => {
  const got = await ide(`${open("divzero.pas")} Ctrl-F9`, guests("DIVZERO.PAS"));
  assert.equal(got.status, "Exit code 200 - press a key");
});

test("Ctrl-F9 with a compile error shows the error and runs nothing", async () => {
  const got = await ide(`${open("missemi.pas")} Ctrl-F9`, fixtures("MISSEMI.PAS"));
  assert.match(got.status, /Error 85: ";" expected$/);
  assert.match(got.rows[0], /File/, "the IDE screen was not replaced");
});

// ---- the exit test of the plan ----
// Snake reads every key that is waiting, so a key script cannot continue after Ctrl-F9 in
// Snake: the first two tests stop at the prompt after its run. The whole sequence is run with
// HELLO.PAS, which reads no keys; tests/host/browser/idemain.mjs does it with Snake and a real
// keyboard.

// Line 42 of Snake is `  Write('Score: ', Score, '   ');`: Home and nine Rights put the cursor
// on the S.
const EDIT_SNAKE = `${open("snake.pas")} ${times(41, "Down")} Home ${times(9, "Right")} ` +
  `${times(5, "Del")} ${type("Points")}`;

test("Snake: open, edit a line, compile", async () => {
  const got = await ide(`${EDIT_SNAKE} F9`, snake());
  assert.match(got.status, /^ 42:16 +Insert +Compiled$/);
  assert.ok(got.rows.some((row) => row.startsWith("  Write('Points: ', Score")), "the edit");
  assert.match(got.rows[0], /SNAKE\.PAS$/);
});

test("Snake: run the edited program; the edit shows in its first row", async () => {
  // Snake ends when the snake runs into the wall; the run uses a virtual clock.
  const got = await ide(`${EDIT_SNAKE} F9 Ctrl-F9`, snake());
  assert.match(got.rows[0], /^Points: \d+ *$/);
  assert.equal(got.status, "Exit code 0 - press a key");
});

// Line 5 of HELLO.PAS is `  Write('guest was here');`; line 4 is `  GotoXY(5, 3);`.
const EDIT = `${open("hello.pas")} ${times(4, "Down")} End ${times(3, "Left")} ` +
  `${times(4, "Backspace")} ${type("over there")}`;
const BREAK = "Up End Backspace";           // the `;` of line 4
const FIX = "Up End ;";                     // the cursor is on line 5: back to line 4
const RUN = "Ctrl-F9";

test("exit test: edit a line, compile, run", async () => {
  const got = await ide(`${EDIT} F9`, guests("HELLO.PAS"));
  assert.match(got.status, /^ 5:30 +Insert +Compiled$/);
  const ran = await ide(`${EDIT} ${RUN}`, guests("HELLO.PAS"));
  assert.equal(ran.rows[2], "    guest was over there");
  assert.equal(ran.status, "Exit code 0 - press a key");
});

test("exit test: a missing semicolon, the message and the cursor on the next line", async () => {
  const got = await ide(`${EDIT} ${RUN} x ${BREAK} F9`, guests("HELLO.PAS"));
  assert.match(got.status, /^ 5:3 +Insert +Error 85: ";" expected$/);
  assert.equal(got.cursor, "cursor 3 6", "line 5 of the file is row 6 of the screen");
});

test("exit test: fix the semicolon, run again", async () => {
  const got = await ide(`${EDIT} ${RUN} x ${BREAK} F9 ${FIX} ${RUN}`, guests("HELLO.PAS"));
  assert.equal(got.rows[2], "    guest was over there");
  assert.equal(got.status, "Exit code 0 - press a key");
});

test("exit test: back in the IDE, Quit offers to save the edits and Y saves them", async () => {
  const keys = `${EDIT} ${RUN} x ${BREAK} F9 ${FIX} ${RUN} x Alt-X`;
  const asked = await ide(keys, guests("HELLO.PAS"));
  assert.equal(asked.code, 255);
  assert.match(asked.status, /Save changes to HELLO\.PAS\? \(Y\/N\)$/);
  const saved = [];
  const finished = await run(build.module, {
    files: guests("HELLO.PAS"), keys: Buffer.from(`${keys} y`), compiler,
    onSaved: (name, bytes) => saved.push([name, Buffer.from(bytes).toString("latin1")]),
  });
  assert.equal(finished.code, 0);
  assert.equal(saved.length, 1);
  assert.equal(saved[0][0], "HELLO.PAS");
  assert.ok(saved[0][1].includes("  Write('guest was over there');"), "the edited line");
  assert.ok(saved[0][1].includes("  GotoXY(5, 3);\r\n"), "the fixed line");
});

await runTests(tests);
