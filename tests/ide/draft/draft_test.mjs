// Tests the draft IDE host imports against the real compiler.wasm, without an IDE program:
// the test plays the IDE, writing the buffer into memory and calling the two imports.
// Usage: node tests/ide/draft/draft_test.mjs [compiler.wasm]   (default: the release build
// from `make wasm`). Expected results in oracle/ were made with tools/tp5.sh.
import assert from "node:assert/strict";
import fs from "node:fs";
import { execFileSync } from "node:child_process";
import { Host } from "../../../host/runtime.js";
import { dump } from "../../screen_dump.mjs";
import { KeyScript, parseKeys } from "../../keys.mjs";
import { addIdeImports, loadCompiler } from "./node_ide.mjs";
import { cannedCompiler, cannedRun } from "./canned.mjs";
import { ideImports } from "./ide.js";

const ORACLE = new URL("./oracle/", import.meta.url);
const root = new URL("../../../", import.meta.url).pathname;
const target = execFileSync("sh", ["scripts/cargo-target.sh"], { cwd: root, encoding: "utf8" }).trim();
const wasmPath = process.argv[2] ?? `${target}/wasm32-unknown-unknown/release/tp5c.wasm`;
const compiler = await loadCompiler(wasmPath);

const source = (name) => fs.readFileSync(new URL(`${name}.pas`, ORACLE));
const expected = (name) => fs.readFileSync(new URL(`${name}.out`, ORACLE), "latin1");
const bytes = (text) => Uint8Array.from(text, (c) => c.charCodeAt(0));

// An IDE host: memory the "IDE" owns, a file system, scripted keys.
function newIde({ files = new Map(), keys = "", keepFiles = false } = {}) {
  const out = [];
  const host = new Host({
    write: (_fd, data) => out.push(Buffer.from(data)),
    input: null, files, keys: new KeyScript(parseKeys(keys)),
    clock: { nowMs: () => 0, advance() {} },
  });
  const memory = new WebAssembly.Memory({ initial: 1 });
  host.attach({ exports: { memory } });
  addIdeImports(host, compiler, { keepFiles });
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

test("a program with an error in the buffer: code, line and the main file's name", () => {
  const ide = newIde();
  const r = compile(ide, "P.PAS", source("error_in_main"));
  assert.equal(expected("error_in_main").trim(), "P.PAS(3): Error 3: Unknown identifier.");
  assert.deepEqual([r.code, r.file, r.line], [3, "P.PAS", 3]);
  assert.ok(r.col > 0, "the column is reported");
});

test("an error in a unit from the file system names the unit", () => {
  const unit = fs.readFileSync(new URL("error_in_unit/U.PAS", ORACLE));
  const ide = newIde({ files: new Map([["U.PAS", unit]]) });
  const r = compile(ide, "P.PAS", source("error_in_unit"));
  assert.equal(expected("error_in_unit").trim(), "U.PAS(7): Error 3: Unknown identifier.");
  assert.deepEqual([r.code, r.file, r.line], [3, "U.PAS", 7]);
});

test("the unsaved buffer wins over a saved file of the same name", () => {
  const stale = bytes("program P;\nbegin\n  nonsense;\nend.\n");
  const ide = newIde({ files: new Map([["P.PAS", stale]]) });
  assert.equal(compile(ide, "P.PAS", source("runtime_error_after_writing_a_file")).code, 0);
});

test("a successful compile clears the result record; ide_run before one is -2", () => {
  const ide = newIde();
  assert.equal(ide.host.imports.ide_run(), -2);
  compile(ide, "P.PAS", source("error_in_main"));
  assert.equal(ide.host.imports.ide_run(), -2, "a failed compile leaves nothing to run");
  const r = compile(ide, "P.PAS", source("runtime_error_after_writing_a_file"));
  assert.deepEqual([r.code, r.line, r.col, r.file], [0, 0, 0, ""]);
});

test("run: a runtime error ends the run with its code; with keepFiles (Node only) the file is kept", () => {
  const ide = newIde({ keepFiles: true });
  compile(ide, "P.PAS", source("runtime_error_after_writing_a_file"));
  assert.equal(ide.host.imports.ide_run(), 200);
  const written = ide.host.vfs.files.get("OUT.TXT");
  assert.equal(Buffer.from(written.data).toString("latin1"), "written by the program\r\n");
});

test("run: by default the program's files are gone, in Node as in the browser", () => {
  const ide = newIde({ keepFiles: false });
  compile(ide, "P.PAS", source("runtime_error_after_writing_a_file"));
  ide.host.imports.ide_run();
  assert.equal(ide.host.vfs.files.has("OUT.TXT"), false);
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

test("a bad pointer from the IDE is a host error, not a crash", () => {
  const ide = newIde();
  assert.throws(() => ide.host.imports.ide_compile(0, 1 << 20, 10, 32), /out of bounds/);
});

let failed = 0;
for (const [name, body] of tests) {
  try {
    body();
  } catch (e) {
    failed++;
    console.error(`FAIL ${name}\n${e.stack}`);
  }
}
if (failed) process.exit(1);
console.log(`${tests.length} tests passed`);
