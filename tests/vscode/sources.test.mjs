import test from "node:test";
import assert from "node:assert/strict";

const load = () => import("../../vscode/src/sources.mjs");
const bytes = (text) => Uint8Array.from(Buffer.from(text, "latin1"));
const disk = (files) => ({
  diskNames: Object.keys(files),
  readDisk: (name) => bytes(files[name]),
});

test("only .pas and .inc files are sources, in any case, and the main file is left out", async () => {
  const { selectSources } = await load();
  const result = selectSources({
    mainName: "Main.pas",
    ...disk({ "Main.pas": "m", "U.PAS": "u", "defs.inc": "i", "notes.txt": "n", "a.pas.bak": "b" }),
    buffers: new Map(),
  });
  assert.deepEqual([...result.keys()].sort(), ["U.PAS", "defs.inc"]);
  assert.deepEqual(result.get("U.PAS"), bytes("u"));
});

test("main file matching ignores case", async () => {
  const { selectSources } = await load();
  const result = selectSources({ mainName: "main.PAS", ...disk({ "MAIN.pas": "m" }), buffers: new Map() });
  assert.equal(result.size, 0);
});

test("an unsaved buffer wins over the file on disk, whatever the case of its name", async () => {
  const { selectSources } = await load();
  const reads = [];
  const result = selectSources({
    mainName: "main.pas",
    diskNames: ["Util.pas", "other.pas"],
    readDisk: (name) => (reads.push(name), bytes("disk")),
    buffers: new Map([["UTIL.PAS", "buffer"]]),
  });
  assert.deepEqual(result.get("UTIL.PAS"), bytes("buffer"));
  assert.equal(result.has("Util.pas"), false, "one entry per file");
  assert.deepEqual(result.get("other.pas"), bytes("disk"));
  assert.deepEqual(reads, ["other.pas"], "the shadowed disk file is not read");
});

test("a buffer that is not on disk yet is a source; the main buffer is not", async () => {
  const { selectSources } = await load();
  const result = selectSources({
    mainName: "main.pas",
    diskNames: [],
    readDisk: () => assert.fail("nothing to read"),
    buffers: new Map([["new.pas", "unit New;"], ["main.pas", "program m;"], ["readme.md", "x"]]),
  });
  assert.deepEqual([...result.keys()], ["new.pas"]);
});

test("a buffer's text is encoded one byte per character", async () => {
  const { selectSources } = await load();
  const result = selectSources({
    mainName: "m.pas",
    diskNames: [],
    readDisk: () => assert.fail("nothing to read"),
    buffers: new Map([["u.pas", "café€"]]),
  });
  assert.deepEqual([...result.get("u.pas")], [0x63, 0x61, 0x66, 0xe9, 0x3f]);
});
