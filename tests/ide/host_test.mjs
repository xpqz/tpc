// Tests the IDE host (host/compiler.js, host/ide.js, the embedded IdeHost unit) against the
// real compiler.wasm, at two levels: the imports called directly, the way the IdeHost unit
// calls them, and a toy IDE program (tests/lang/ide_host_toy_open_save_compile_run.pas) driven
// by keys. Expected results come from the real TP 5.0: the files in oracle/ and guests/ were
// made with tools/tp5.sh (see docs/prs/74.md for the commands).
// Usage: node tests/ide/host_test.mjs [compiler.wasm]   (default: the release build of
// `make wasm`).
import assert from "node:assert/strict";
import fs from "node:fs";
import { execFileSync } from "node:child_process";
import { Host } from "../../host/runtime.js";
import { ideImports } from "../../host/ide.js";
import { loadFiles, saveFile } from "../../host/persist.js";
import { RunnerError, run, loadCompiler, addIdeImports } from "../run.mjs";
import { dump } from "../screen_dump.mjs";
import { KeyScript, parseKeys } from "../keys.mjs";
import { cannedCompiler, cannedRun } from "./canned.mjs";
import { runTests } from "../host/browser_helpers.mjs";

const ORACLE = new URL("./oracle/", import.meta.url);
const GUESTS = new URL("./guests/", import.meta.url);
const TOY = new URL("../lang/ide_host_toy_open_save_compile_run", import.meta.url);
const root = new URL("../../", import.meta.url).pathname;
const target = execFileSync("sh", ["scripts/cargo-target.sh"], { cwd: root, encoding: "utf8" }).trim();
const wasmPath = process.argv[2] ?? `${target}/wasm32-unknown-unknown/release/tp5c.wasm`;
const compiler = await loadCompiler(wasmPath);

const read = (url) => fs.readFileSync(url);
const source = (name) => read(new URL(`${name}.pas`, ORACLE));
const expected = (name) => read(new URL(`${name}.out`, ORACLE)).toString("latin1");
const bytes = (text) => Uint8Array.from(text, (c) => c.charCodeAt(0));
const guestOut = (name) => read(new URL(name, GUESTS)).toString("latin1");

// An IDE host: memory the "IDE" owns, a file system, scripted keys.
function newIde({ files = new Map(), keys = "" } = {}) {
  const out = [];
  const host = new Host({
    write: (_fd, data) => out.push(Buffer.from(data)),
    input: null, files, keys: new KeyScript(parseKeys(keys)),
    clock: { nowMs: () => 0, advance() {} },
  });
  const memory = new WebAssembly.Memory({ initial: 1 });
  host.attach({ exports: { memory } });
  addIdeImports(host, compiler);
  return { host, memory, out };
}

// What the IDE's unit does: name as a Pascal string at 0, result record at 32, text at 64.
function compile(ide, name, text) {
  const heap = new Uint8Array(ide.memory.buffer);
  heap.set([name.length, ...bytes(name)], 0);
  heap.set(text, 64);
  ide.host.imports.ide_compile(0, 64, text.length, 32);
  const view = new DataView(ide.memory.buffer, 32, 23);
  const nameLength = heap[32 + 10];
  return {
    code: view.getInt16(0, true), line: view.getInt32(2, true), col: view.getInt32(6, true),
    file: String.fromCharCode(...heap.slice(32 + 11, 32 + 11 + nameLength)),
  };
}

const tests = [];
const test = (name, body) => tests.push([name, body]);

// ---- the imports, called the way the IdeHost unit calls them ----

test("a program with an error in the buffer: code, line and the main file's name", () => {
  const ide = newIde();
  const r = compile(ide, "P.PAS", source("error_in_main"));
  assert.equal(expected("error_in_main").trim(), "P.PAS(3): Error 3: Unknown identifier.");
  assert.deepEqual([r.code, r.file, r.line], [3, "P.PAS", 3]);
  assert.ok(r.col > 0, "the column is reported");
});

test("an error in a unit from the file system names the unit", () => {
  const unit = read(new URL("error_in_unit/U.PAS", ORACLE));
  const ide = newIde({ files: new Map([["U.PAS", unit]]) });
  const r = compile(ide, "P.PAS", source("error_in_unit"));
  assert.equal(expected("error_in_unit").trim(), "U.PAS(7): Error 3: Unknown identifier.");
  assert.deepEqual([r.code, r.file, r.line], [3, "U.PAS", 7]);
});

test("the unsaved buffer wins over a saved file of the same name, in any case", () => {
  const stale = bytes("program P;\nbegin\n  nonsense;\nend.\n");
  const ide = newIde({ files: new Map([["P.PAS", stale]]) });
  const fresh = source("runtime_error_after_writing_a_file");
  assert.equal(compile(ide, "P.PAS", fresh).code, 0);
  assert.equal(compile(ide, "p.pas", fresh).code, 0);
});

test("a unit saved again is seen by the next compile (registered files are replaced)", () => {
  const files = new Map([["U.PAS", bytes("unit U;\ninterface\nprocedure Hello;\nimplementation\n" +
    "procedure Hello; begin nonsense end;\nend.\n")]]);
  const ide = newIde({ files });
  const main = bytes("program C;\nuses U;\nbegin\n  Hello;\nend.\n");
  const bad = compile(ide, "C.PAS", main);
  assert.deepEqual([bad.code, bad.file], [3, "U.PAS"]);
  ide.host.vfs.files.set("U.PAS", { data: bytes("unit U;\ninterface\nprocedure Hello;\n" +
    "implementation\nprocedure Hello; begin end;\nend.\n") });
  assert.equal(compile(ide, "C.PAS", main).code, 0);
});

test("an include file of the file system is found by {$I}", () => {
  const files = new Map([["BODY.INC", bytes("procedure Say;\nbegin\n  WriteLn('from include');\n" +
    "end;\n")]]);
  const ide = newIde({ files });
  const r = compile(ide, "P.PAS", bytes("program P;\n{$I BODY.INC}\nbegin\n  Say;\nend.\n"));
  assert.equal(r.code, 0);
  const code = ide.host.imports.ide_run();
  assert.equal(code, 0);
  assert.equal(dump(ide.host.screen.cells, code).split("\n")[0], "from include");
});

test("files that are not Pascal sources are not registered", () => {
  const files = new Map([["DATA.TXT", bytes("this is not Pascal")], ["NOTES", bytes("x")]]);
  const ide = newIde({ files });
  assert.equal(compile(ide, "P.PAS", bytes("program P;\nbegin\nend.\n")).code, 0);
  assert.equal(compiler.registered.includes("DATA.TXT"), false);
  assert.equal(compiler.registered.includes("NOTES"), false);
});

test("nothing to run is -2: before any compile and after a failed one", () => {
  const ide = newIde();
  assert.equal(ide.host.imports.ide_run(), -2);
  compile(ide, "P.PAS", source("error_in_main"));
  assert.equal(ide.host.imports.ide_run(), -2, "a failed compile leaves nothing to run");
  const r = compile(ide, "P.PAS", source("runtime_error_after_writing_a_file"));
  assert.deepEqual([r.code, r.line, r.col, r.file], [0, 0, 0, ""]);
});

test("a failed compile after a good one forgets the good module", () => {
  const ide = newIde();
  compile(ide, "P.PAS", bytes("program P;\nbegin\nend.\n"));
  compile(ide, "P.PAS", source("error_in_main"));
  assert.equal(ide.host.imports.ide_run(), -2);
});

test("run: a runtime error ends the run with its code and the program's files are gone", () => {
  const ide = newIde();
  compile(ide, "P.PAS", source("runtime_error_after_writing_a_file"));
  assert.equal(ide.host.imports.ide_run(), 200);
  assert.equal(ide.host.vfs.files.has("OUT.TXT"), false);
  assert.equal(compiler.registered.includes("OUT.TXT"), false);
});

test("run twice runs the same module twice", () => {
  const ide = newIde();
  compile(ide, "P.PAS", source("runtime_error_after_writing_a_file"));
  assert.deepEqual([ide.host.imports.ide_run(), ide.host.imports.ide_run()], [200, 200]);
});

test("run: a Crt program starts on a cleared screen and reads the IDE's keyboard", () => {
  const ide = newIde({ keys: "x" });
  ide.host.screen.cells.fill(88); // the IDE's picture is wiped, as the user screen would be
  compile(ide, "P.PAS", source("run_crt_program"));
  const code = ide.host.imports.ide_run();
  assert.equal(dump(ide.host.screen.cells, code), expected("run_crt_program"));
  assert.equal(ide.host.keys.available(), false, "the program consumed the key");
});

// The texts and results are those of the toy editor's run on the real TP 5.0, whose log row
// reads: Error 3 in P.PAS 2:1; Run -2; Compiled; Run 2; (tests/lang/crt_ide_oracle_pattern_*).
test("canned compiler gives the results the Pascal stand-in gave under TP 5.0", () => {
  const ide = newIde();
  Object.assign(ide.host.imports, ideImports(ide.host, {
    compiler: cannedCompiler(), runModule: cannedRun }));
  const bad = compile(ide, "P.PAS", bytes("begin\r\nERROR\r\nend.\r\n"));
  assert.deepEqual([bad.code, bad.line, bad.col, bad.file], [3, 2, 1, "P.PAS"]);
  assert.equal(ide.host.imports.ide_run(), -2);
  const good = compile(ide, "P.PAS", bytes("begin\r\nend.\r\n"));
  assert.deepEqual([good.code, good.line, good.col, good.file], [0, 0, 0, ""]);
  assert.equal(ide.host.imports.ide_run(), 2, "two lines compiled: the toy's `Run 2`");
});

test("an injected run step gets the module and a Map of the IDE's files", () => {
  const files = new Map([["A.TXT", bytes("a")]]);
  const ide = newIde({ files });
  let seen = null;
  Object.assign(ide.host.imports, ideImports(ide.host, {
    compiler, runModule: (module, programFiles) => ((seen = { module, programFiles }), 7) }));
  compile(ide, "P.PAS", bytes("program P;\nbegin\nend.\n"));
  assert.equal(ide.host.imports.ide_run(), 7);
  assert.ok(seen.module instanceof Uint8Array && seen.module.length > 8);
  assert.deepEqual([...seen.programFiles], [["A.TXT", bytes("a")]]);
});

test("a bad pointer from the IDE is a host error, not a crash", () => {
  const ide = newIde();
  assert.throws(() => ide.host.imports.ide_compile(0, 1 << 20, 10, 32), /out of bounds/);
  assert.throws(() => ide.host.imports.ide_compile(0, 64, 10, (1 << 16) - 4), /out of bounds/);
});

// ---- the toy IDE, driven by keys ----

// A second compiler instance builds the toy, so that nothing it registers is seen by the one
// the toy itself calls through the host.
const toyBuild = (await loadCompiler(wasmPath)).compile(
  "TOY.PAS", read(new URL(`${TOY.pathname}.pas`, import.meta.url)), new Map());
assert.ok(toyBuild.module, `the toy does not compile: ${JSON.stringify(toyBuild)}`);
const toyModule = toyBuild.module;

// Files for the toy's file system: the fixture's own (BAD.PAS, GOOD.PAS) or a guest program.
const FIXTURE = new URL(`${TOY.pathname}/`, import.meta.url);
const toyFiles = (...names) =>
  new Map(names.map((n) => [n, read(new URL(n, fs.existsSync(new URL(n, FIXTURE)) ? FIXTURE : GUESTS))]));

async function toy(keys, { files = toyFiles("BAD.PAS", "GOOD.PAS"), onSaved } = {}) {
  const { stdout, code } = await run(toyModule, {
    files, keys: Buffer.from(keys), compiler, onSaved });
  const dump = stdout.toString("latin1");
  return { dump, rows: dump.split("\n"), code };
}

// Key script tokens for typed text (a space is the key `Space`).
const type = (text) => [...text].map((c) => (c === " " ? "Space" : c)).join(" ");
const open = (name) => `F3 ${type(name.toLowerCase())} Enter`;

test("toy: the oracle's key script gives the oracle's screen, except for the run result", async () => {
  // The canned stand-in runs nothing and returns the line count (2); the real compiler runs
  // `begin end.`, which ends with exit code 0. Every other line is the oracle's, from TP 5.0.
  const keys = read(new URL(`${TOY.pathname}.keys`, import.meta.url)).toString();
  const oracle = read(new URL(`${TOY.pathname}.out`, import.meta.url)).toString("latin1");
  const got = await toy(keys);
  assert.equal(got.code, 0);
  assert.equal(got.dump, oracle.replace("Exit code 2", "Exit code 0"));
});

test("toy: File Open of a missing name says so and keeps the buffer", async () => {
  const got = await toy(`${open("nope.pas")} Alt-X`);
  assert.equal(got.rows[2], "Cannot open NOPE.PAS");
  assert.equal(got.rows[0], "File: P.PAS");
});

test("toy: a compile error reports the code, file, line and column", async () => {
  const got = await toy(`${open("bad.pas")} F9 Alt-X`);
  assert.equal(got.rows[2], "Error 3 in BAD.PAS 2:1");
});

test("toy: an error in a unit names the unit's file", async () => {
  const files = new Map([
    ["C.PAS", read(new URL("error_in_unit.pas", ORACLE))],
    ["U.PAS", read(new URL("error_in_unit/U.PAS", ORACLE))]]);
  const got = await toy(`${open("c.pas")} F9 Alt-X`, { files });
  assert.match(got.rows[2], /^Error 3 in U\.PAS 7:\d+$/);
});

test("toy: F10 with nothing compiled shows -2 on the status line of the user screen", async () => {
  const got = await toy("F10 x Alt-X");
  assert.ok(got.rows.includes("Exit code -2"), "-2 reaches the log");
});

test("toy: a run starts on a cleared screen and leaves the program's picture until a key", async () => {
  const got = await toy(`${open("hello.pas")} Ctrl-F9`, { files: toyFiles("HELLO.PAS") });
  assert.equal(got.code, 255, "the toy waits for the key that is not there");
  // The oracle's picture of the program alone: 'guest was here' at column 5 of row 3.
  const picture = guestOut("hello.out").split("\n");
  assert.equal(got.rows[2], picture[2]);
  assert.equal(got.rows[0], "", "the toy's own header was wiped before the run");
  assert.equal(got.rows[24], "Exit code 0 - press a key");
});

test("toy: the program reads the keys of the same keyboard; the toy gets the next one", async () => {
  const keys = `${open("echokey.pas")} Ctrl-F9 z`;
  const files = toyFiles("ECHOKEY.PAS");
  const waiting = await toy(keys, { files });
  const picture = guestOut("echokey.out").split("\n");
  assert.equal(guestOut("echokey.keys").trim(), "z");
  assert.equal(waiting.rows[0], picture[0], "the program took the z");
  const after = await toy(`${keys} x Alt-X`, { files });
  assert.equal(after.code, 0, "the toy took the x and the Alt-X");
  assert.equal(after.rows[0], "File: ECHOKEY.PAS", "the toy repainted after the key");
  assert.ok(after.rows.includes("Exit code 0"));
});

test("toy: a Crt program that clears the screen shows only its own text", async () => {
  const got = await toy(`${open("shout.pas")} Ctrl-F9`, { files: toyFiles("SHOUT.PAS") });
  assert.equal(got.rows[0], guestOut("shout.out").split("\n")[0]);
  assert.equal(got.rows[1], "");
});

test("toy: a runtime error returns its number; the host prints it on the screen", async () => {
  // TP prints `Runtime error 200 at 0000:0016.` (guests/divzero.out); this host prints the
  // number alone (the address has no meaning here), so only the number is compared.
  assert.match(guestOut("divzero.out"), /^Runtime error 200 at/);
  const got = await toy(`${open("divzero.pas")} Ctrl-F9`, { files: toyFiles("DIVZERO.PAS") });
  assert.equal(got.rows[0], "Runtime error 200");
  assert.equal(got.rows[24], "Exit code 200 - press a key");
});

test("toy: a program is compiled from the buffer, not from the saved file", async () => {
  // GOOD.PAS compiles. Backspace over "end.\n" and typing `q;` leaves a buffer that does not,
  // while the saved file is unchanged.
  const got = await toy(`${open("good.pas")} F9 ${"Backspace ".repeat(5)}q ; F9 Alt-X`);
  assert.deepEqual(got.rows.slice(4, 8).filter((r) => r !== ""),
    ["Ready", "Opened GOOD.PAS", "Compiled", got.rows[7]]);
  assert.match(got.rows[7], /^Error \d+ in GOOD\.PAS \d+:\d+$/);
});

test("toy: Save writes the buffer through the host file system and reports it", async () => {
  const saved = [];
  const got = await toy(`${open("good.pas")} h i F2 Alt-X`, {
    onSaved: (name, data) => saved.push([name, Buffer.from(data).toString("latin1")]) });
  assert.equal(got.rows[2], "Saved GOOD.PAS");
  assert.deepEqual(saved, [["GOOD.PAS", "begin\nend.\nhi"]]);
});

test("toy: saved files survive in a store and the next session opens, compiles and runs them", async () => {
  const store = new Map();
  const storage = {
    get length() { return store.size; },
    key: (i) => [...store.keys()][i] ?? null,
    getItem: (key) => store.get(key) ?? null,
    setItem: (key, value) => store.set(key, String(value)),
  };
  // Session 1: type a whole program into a new file and save it.
  const program = "program S; begin end.";
  await toy(`${type(program)} F2 Alt-X`, {
    files: loadFiles(storage), onSaved: (name, data) => saveFile(storage, name, data) });
  assert.deepEqual([...store.keys()], ["tp5:P.PAS"]);
  assert.equal(store.get("tp5:P.PAS"), program);
  // Session 2: nothing but the store.
  const got = await toy(`${open("p.pas")} Ctrl-F9 x Alt-X`, { files: loadFiles(storage) });
  assert.deepEqual(got.rows.slice(4, 8).filter((r) => r !== ""),
    ["Ready", "Opened P.PAS", "Compiled", "Exit code 0"]);
});

test("toy: the program's own files do not reach the next run or the saved files", async () => {
  const saved = [];
  const files = new Map([["W.PAS", source("runtime_error_after_writing_a_file")]]);
  const got = await toy(`${open("w.pas")} Ctrl-F9 x ${open("out.txt")} Alt-X`, {
    files, onSaved: (...a) => saved.push(a) });
  assert.deepEqual(saved, [], "OUT.TXT written by the program is not saved");
  assert.ok(got.rows.includes("Cannot open OUT.TXT"), "the IDE cannot open the program's file");
});

test("toy: an IDE program needs a compiler: without one the runner refuses it", async () => {
  await assert.rejects(
    run(toyModule, { files: new Map(), keys: Buffer.from("Alt-X") }),
    (e) => e instanceof RunnerError && /unknown import host\.ide_compile/.test(e.message));
});

await runTests(tests);
