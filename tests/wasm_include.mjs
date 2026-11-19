// node tests/wasm_include.mjs compiler.wasm ok.pas include_err.pas statement_err.pas
// Include files are registered with add_file like units (decision 27); an error inside an
// include reports the registration index of the include (decision 28). Each program has its
// include files in a directory next to it (decision 26) and an `.out` or `.err`.
import fs from "node:fs";
import path from "node:path";
import assert from "node:assert/strict";
import { run } from "./run.mjs";

const [wasmPath, okPas, includeErrPas, statementErrPas] = process.argv.slice(2);
const { instance } = await WebAssembly.instantiate(fs.readFileSync(wasmPath), {});
const x = instance.exports;

function put(...parts) {
  const total = parts.reduce((n, p) => n + p.length, 0);
  const base = x.alloc(total);
  let at = base;
  return parts.map((p) => {
    new Uint8Array(x.memory.buffer, at, p.length).set(p);
    at += p.length;
    return at - p.length;
  });
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
  return fs.readdirSync(pasPath.replace(/\.pas$/, "")).filter((f) => /\.INC$/.test(f)).sort();
}

function register(pasPath) {
  const dir = pasPath.replace(/\.pas$/, "");
  for (const f of auxFiles(pasPath)) assert.equal(addFile(f, fs.readFileSync(path.join(dir, f))), 0);
}

function expectedError(pasPath) {
  const [code, line, file] = fs.readFileSync(pasPath.replace(/\.pas$/, ".err"), "utf8").trim().split(/\s+/);
  return { code: Number(code), line: Number(line), file };
}

// Fourteen nested includes compile and print what TP5 printed.
register(okPas);
assert.equal(compile(okPas), 0, "program with nested includes");
const wasm = new Uint8Array(x.memory.buffer, x.out_ptr(), x.out_len()).slice();
const want = fs.readFileSync(okPas.replace(/\.pas$/, ".out"));
assert.ok(want.length > 0);
assert.deepEqual((await run(wasm)).stdout, want);

// An error in an include: err_file is the registration index, counted over all files added.
const before = auxFiles(okPas).length;
register(includeErrPas);
assert.equal(compile(includeErrPas), 1);
let e = expectedError(includeErrPas);
assert.equal(x.err_code(), e.code);
assert.equal(x.err_line(), e.line);
assert.equal(x.err_file(), before + auxFiles(includeErrPas).indexOf(e.file) + 1);

// An error in the main program after that is file 0.
register(statementErrPas);
assert.equal(compile(statementErrPas), 1);
e = expectedError(statementErrPas);
assert.deepEqual([x.err_code(), x.err_line(), x.err_file()], [e.code, e.line, 0]);
