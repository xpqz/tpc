// The wasm export err_message_ptr/err_message_len gives the same message text as the native
// tp5c prints, for several error numbers: the native text is the reference.
import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync, spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { buildNative, buildWasm, root } from "./build.mjs";

const scratch = fs.mkdtempSync(path.join(root, "tmp/vscode-ffi-"));
test.after(() => fs.rmSync(scratch, { recursive: true, force: true }));

const cases = [
  { code: 3, source: "program a;\nbegin\n  x := 1;\nend.\n" },
  { code: 42, source: "program a;\nbegin\n  if then;\nend.\n" },
  { code: 10, source: "program a;\nbegin\n" },
  { code: 15, source: "program a;\nuses nothere;\nbegin\nend.\n" },
  { code: 26, source: "program a;\nvar i: integer;\nbegin\n  i := 's';\nend.\n" },
  { code: 85, source: "program a;\nbegin\n  writeln(1)\n  writeln(2);\nend.\n" },
];

const nativeMessage = (tp5c, source) => {
  const file = path.join(scratch, "prog.pas");
  fs.writeFileSync(file, source);
  const run = spawnSync(tp5c, [file, "-o", path.join(scratch, "prog.wasm")], { encoding: "utf8" });
  assert.equal(run.status, 1, run.stderr);
  const match = run.stderr.match(/\): Error (\d+): (.*)\n$/);
  assert.ok(match, run.stderr);
  return { code: Number(match[1]), message: match[2] };
};

const wasmMessage = (exports, source) => {
  const bytes = Buffer.from(source, "latin1");
  const at = exports.alloc(bytes.length);
  new Uint8Array(exports.memory.buffer, at, bytes.length).set(bytes);
  assert.equal(exports.compile(at, bytes.length, 0), 1);
  const text = new Uint8Array(exports.memory.buffer, exports.err_message_ptr(), exports.err_message_len());
  return { code: exports.err_code(), message: Buffer.from(text).toString("latin1") };
};

test("the wasm error message equals the native tp5c message", async () => {
  const tp5c = buildNative();
  const module = await WebAssembly.compile(fs.readFileSync(buildWasm()));
  for (const { code, source } of cases) {
    const native = nativeMessage(tp5c, source);
    assert.equal(native.code, code);
    assert.notEqual(native.message, "");
    const { exports } = await WebAssembly.instantiate(module);
    assert.deepEqual(wasmMessage(exports, source), native, `error ${code}`);
  }
});

test("an error number without a table entry has the message Unknown error", async () => {
  // Before any compile the stored error is the default Diag with code 0, which has no entry.
  const { exports } = await WebAssembly.instantiate(
    await WebAssembly.compile(fs.readFileSync(buildWasm())),
  );
  const text = new Uint8Array(exports.memory.buffer, exports.err_message_ptr(), exports.err_message_len());
  assert.equal(Buffer.from(text).toString("latin1"), "Unknown error");
});
