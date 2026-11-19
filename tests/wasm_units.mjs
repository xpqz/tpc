// node tests/wasm_units.mjs compiler.wasm ok.pas unit_err.pas cycle.pas missing.pas
// Drives add_file and err_file of the compiler's wasm exports (decisions 27 and 28).
// Each program has its units in a directory next to it (decision 26) and an `.out` or `.err`.
// Protocol under test: one `alloc` holds the name and then the source; add_file takes both
// ranges inside it. Registered files stay registered across compile calls; a second
// add_file of the same name replaces the first.
import fs from "node:fs";
import path from "node:path";
import assert from "node:assert/strict";
import { run } from "./run.mjs";

const [wasmPath, okPas, unitErrPas, cyclePas, missingPas] = process.argv.slice(2);
const { instance } = await WebAssembly.instantiate(fs.readFileSync(wasmPath), {});
const x = instance.exports;
for (const n of ["add_file", "err_file"]) {
  assert.ok(n in x, `missing export ${n}`);
}

function put(...parts) {
  const total = parts.reduce((n, p) => n + p.length, 0);
  const base = x.alloc(total);
  let at = base;
  const starts = parts.map((p) => {
    new Uint8Array(x.memory.buffer, at, p.length).set(p);
    at += p.length;
    return at - p.length;
  });
  return starts;
}

function addFile(name, text) {
  const n = Buffer.from(name);
  const [namePtr, srcPtr] = put(n, text);
  return x.add_file(namePtr, n.length, srcPtr, text.length);
}

function compile(pasPath) {
  const src = fs.readFileSync(pasPath);
  const [ptr] = put(src);
  return x.compile(ptr, src.length, 0);
}

function auxFiles(pasPath) {
  const dir = pasPath.replace(/\.pas$/, "");
  return fs.readdirSync(dir).filter((f) => /\.PAS$/.test(f)).sort();
}

function register(pasPath) {
  const dir = pasPath.replace(/\.pas$/, "");
  for (const f of auxFiles(pasPath)) assert.equal(addFile(f, fs.readFileSync(path.join(dir, f))), 0);
}

function expectedError(pasPath) {
  const [code, line, file] = fs.readFileSync(pasPath.replace(/\.pas$/, ".err"), "utf8").trim().split(/\s+/);
  return { code: Number(code), line: Number(line), file };
}

// A program with its unit: compiles and prints what TP5 printed.
register(okPas);
assert.equal(compile(okPas), 0);
const wasm = new Uint8Array(x.memory.buffer, x.out_ptr(), x.out_len()).slice();
const want = fs.readFileSync(okPas.replace(/\.pas$/, ".out"));
assert.ok(want.length > 0);
assert.deepEqual((await run(wasm)).stdout, want);

// An error in a unit: err_file is the registration index, 1 for the first file added.
// Files registered so far stay registered; the second program's files follow them.
const before = auxFiles(okPas).length;
register(unitErrPas);
assert.equal(compile(unitErrPas), 1);
let e = expectedError(unitErrPas);
assert.equal(x.err_code(), e.code);
assert.equal(x.err_line(), e.line);
assert.equal(x.err_file(), before + auxFiles(unitErrPas).indexOf(e.file) + 1);

// An error in the main program is file 0, also after a failure in a unit.
assert.equal(compile(missingPas), 1);
e = expectedError(missingPas);
assert.deepEqual([x.err_code(), x.err_line(), x.err_file()], [e.code, e.line, 0]);

// The file that closes a cycle is the one reported.
const names = [...auxFiles(okPas), ...auxFiles(unitErrPas)];
register(cyclePas);
assert.equal(compile(cyclePas), 1);
e = expectedError(cyclePas);
assert.equal(x.err_code(), e.code);
assert.equal(x.err_file(), names.length + auxFiles(cyclePas).indexOf(e.file) + 1);

// Adding a file under an existing name replaces it and keeps its index.
const fixed = Buffer.from("unit Broken;\ninterface\nimplementation\nend.\n");
assert.equal(addFile("BROKEN.PAS", fixed), 0);
assert.equal(compile(unitErrPas), 0);

// A range outside the last alloc is refused.
x.alloc(4);
assert.equal(x.add_file(1 << 20, 4, 1 << 20, 4), 1);
console.log("ok");
