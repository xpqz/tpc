// node tests/wasm_api.mjs compiler.wasm ok.pas ok.out err.pas code line col
// Drives the compiler's wasm exports; exit 0 on success, 1 on mismatch.
import fs from "node:fs";
import assert from "node:assert/strict";
import { run } from "./run.mjs";

const [wasmPath, okSrc, okOut, errSrc, code, line, col] = process.argv.slice(2);
const { instance } = await WebAssembly.instantiate(fs.readFileSync(wasmPath), {});
const x = instance.exports;
for (const n of ["memory", "alloc", "compile", "out_ptr", "out_len", "err_line", "err_col", "err_code"]) {
  assert.ok(n in x, `missing export ${n}`);
}

function compile(path) {
  const src = fs.readFileSync(path);
  const ptr = x.alloc(src.length);
  new Uint8Array(x.memory.buffer, ptr, src.length).set(src);
  return x.compile(ptr, src.length, 0);
}

assert.equal(compile(okSrc), 0);
const len = x.out_len();
assert.ok(len > 8);
const wasm = new Uint8Array(x.memory.buffer, x.out_ptr(), len).slice();
const res = await run(wasm);
const want = fs.readFileSync(okOut);
assert.ok(want.length > 0);
assert.deepEqual(res.stdout, want);

assert.equal(compile(errSrc), 1);
assert.deepEqual([x.err_code(), x.err_line(), x.err_col()], [code, line, col].map(Number));

// A failed compile does not poison the next one.
assert.equal(compile(okSrc), 0);
console.log("ok");
