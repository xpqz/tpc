import test from "node:test";
import assert from "node:assert/strict";
import { buildWasm } from "./build.mjs";
import { fakeCompilerModule } from "./fake_compiler.mjs";

const load = () => import("../../vscode/src/compiler_service.mjs");
const bytes = (text) => Uint8Array.from(Buffer.from(text, "latin1"));
const HELLO = bytes("program hello;\nbegin\n  writeln('hi');\nend.\n");
const USES_UNIT = bytes("program p;\nuses Helper;\nbegin\n  Greet;\nend.\n");
const HELPER = bytes("unit Helper;\ninterface\nprocedure Greet;\nimplementation\nprocedure Greet;\nbegin\n  writeln('hi');\nend;\nend.\n");

const realService = async () => {
  const { CompilerService } = await load();
  return CompilerService.fromFile(buildWasm());
};

test("a valid program gives a wasm binary", async () => {
  const result = (await realService()).compile("hello.pas", HELLO, new Map());
  assert.equal(result.ok, true);
  assert.deepEqual([...result.wasm.slice(0, 4)], [0, 0x61, 0x73, 0x6d]);
});

test("an error carries code, position and the compiler's message", async () => {
  const result = (await realService()).compile("a.pas", bytes("program a;\nbegin\n  x := 1;\nend.\n"), new Map());
  assert.equal(result.ok, false);
  assert.deepEqual(result.error, { code: 3, line: 3, col: 3, file: "a.pas", message: "Unknown identifier" });
});

test("a unit from the file map is found and its errors name the file as given", async () => {
  const service = await realService();
  assert.equal(service.compile("p.pas", USES_UNIT, new Map([["Helper.pas", HELPER]])).ok, true);
  const broken = bytes(Buffer.from(HELPER).toString("latin1").replace("writeln('hi')", "nosuch := 1"));
  const result = service.compile("p.pas", USES_UNIT, new Map([["Helper.pas", broken]]));
  assert.equal(result.ok, false);
  assert.equal(result.error.file, "Helper.pas");
  assert.equal(result.error.code, 3);
  assert.equal(result.error.line, 7);
});

test("a fresh instance per compile: a file that is gone is no longer found", async () => {
  const service = await realService();
  assert.equal(service.compile("p.pas", USES_UNIT, new Map([["Helper.pas", HELPER]])).ok, true);
  const result = service.compile("p.pas", USES_UNIT, new Map());
  assert.equal(result.ok, false);
  assert.equal(result.error.code, 15);
  assert.equal(result.error.file, "p.pas");
});

test("a changed file replaces the earlier text, and the error of an earlier compile does not stick", async () => {
  const service = await realService();
  const bad = service.compile("a.pas", bytes("program a;\nbegin\n  x := 1;\nend.\n"), new Map());
  assert.equal(bad.ok, false);
  assert.equal(service.compile("a.pas", HELLO, new Map()).ok, true);
});

test("compiles are independent of order", async () => {
  const service = await realService();
  const first = service.compile("p.pas", USES_UNIT, new Map());
  const second = service.compile("p.pas", USES_UNIT, new Map());
  assert.deepEqual(first, second);
});

test("a trap in the compiler is an internal error, and the next compile still works", async () => {
  const { CompilerService } = await load();
  const service = new CompilerService(fakeCompilerModule());
  const trapped = service.compile("a.pas", new Uint8Array(0), new Map());
  assert.equal(trapped.ok, false);
  assert.equal(trapped.error, undefined);
  assert.match(trapped.internal, /unreachable/i);
  const next = service.compile("a.pas", bytes("program a;"), new Map());
  assert.deepEqual(next, {
    ok: false,
    error: { code: 5, line: 3, col: 7, file: "a.pas", message: "boom" },
  });
});
