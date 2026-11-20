import test from "node:test";
import assert from "node:assert/strict";

const load = () => import("../../vscode/src/latin1.mjs");

test("every byte value survives decode then encode", async () => {
  const { encodeLatin1, decodeLatin1 } = await load();
  const all = Uint8Array.from({ length: 256 }, (_, i) => i);
  const text = decodeLatin1(all);
  assert.equal(text.length, 256);
  assert.equal(text.charCodeAt(0xe9), 0xe9);
  assert.deepEqual(encodeLatin1(text), all);
});

test("a character above 255 becomes one question mark, so columns stay one per unit", async () => {
  const { encodeLatin1, decodeLatin1 } = await load();
  assert.deepEqual([...encodeLatin1("a€b")], [0x61, 0x3f, 0x62]);
  assert.deepEqual([...encodeLatin1("\u{1F600}")], [0x3f, 0x3f], "a surrogate pair is two units");
});

test("empty text is an empty array", async () => {
  const { encodeLatin1, decodeLatin1 } = await load();
  assert.equal(encodeLatin1("").length, 0);
  assert.equal(decodeLatin1(new Uint8Array(0)), "");
});
