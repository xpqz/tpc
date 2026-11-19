// Checks the persistence pieces of the host: host/persist.js (files <-> a localStorage-like
// store, here an in-memory one), the write hook of host/vfs.js (`onWritten`, fired when a file
// opened for writing is closed) and that the test-only paths are gone from host/. Exit 0 on
// success.
import assert from "node:assert/strict";
import * as vfsModule from "../../host/vfs.js";
import { Vfs, RESET, REWRITE, APPEND } from "../../host/vfs.js";
import { Host } from "../../host/runtime.js";
import { loadFiles, saveFile } from "../../host/persist.js";
import { runTests } from "./browser_helpers.mjs";

// The part of the Storage interface the page uses.
class MemoryStorage {
  constructor() {
    this.items = new Map();
  }
  get length() {
    return this.items.size;
  }
  key(i) {
    return [...this.items.keys()][i] ?? null;
  }
  getItem(key) {
    return this.items.get(key) ?? null;
  }
  setItem(key, value) {
    this.items.set(key, String(value));
  }
}

const bytes = (...values) => Uint8Array.from(values);
const tests = [];
const test = (name, body) => tests.push([name, body]);

test("every byte value survives a save and a load, files are keyed tp5:NAME", () => {
  const store = new MemoryStorage();
  const all = Uint8Array.from({ length: 256 }, (_, i) => i);
  saveFile(store, "ALL.BIN", all);
  assert.deepEqual([...store.items.keys()], ["tp5:ALL.BIN"]);
  assert.equal(store.getItem("tp5:ALL.BIN").length, 256, "one character per byte (Latin-1)");
  const loaded = loadFiles(store);
  assert.deepEqual([...loaded.keys()], ["ALL.BIN"]);
  assert.deepEqual(loaded.get("ALL.BIN"), all);
});

test("a 64 KB file and an empty file round-trip", () => {
  const store = new MemoryStorage();
  const big = Uint8Array.from({ length: 65521 }, (_, i) => (i * 7) & 255);
  saveFile(store, "BIG.PAS", big);
  saveFile(store, "EMPTY.TXT", new Uint8Array(0));
  const loaded = loadFiles(store);
  assert.deepEqual(loaded.get("BIG.PAS"), big);
  assert.equal(loaded.get("EMPTY.TXT").length, 0);
  assert.ok(loaded.has("EMPTY.TXT"), "an empty file still exists");
});

test("saving a name again replaces the file", () => {
  const store = new MemoryStorage();
  saveFile(store, "A.PAS", bytes(1, 2, 3));
  saveFile(store, "A.PAS", bytes(9));
  assert.deepEqual([...loadFiles(store)], [["A.PAS", bytes(9)]]);
});

test("a store with no files, or only other keys, loads as an empty Map", () => {
  const store = new MemoryStorage();
  assert.equal(loadFiles(store).size, 0);
  store.setItem("theme", "dark");
  store.setItem("tp5x", "no colon");
  store.setItem("TP5:UPPER", "wrong prefix case");
  assert.equal(loadFiles(store).size, 0);
});

test("loaded bytes are the program's to change: the store is not touched", () => {
  const store = new MemoryStorage();
  saveFile(store, "A.PAS", bytes(1, 2));
  loadFiles(store).get("A.PAS")[0] = 99;
  assert.deepEqual(loadFiles(store).get("A.PAS"), bytes(1, 2));
});

function write(vfs, name, mode, ...parts) {
  const handle = vfs.open(name, mode);
  assert.ok(handle > 0, `open ${name}: ${handle}`);
  for (const part of parts) assert.equal(vfs.write(handle, part), part.length);
  return handle;
}

test("closing a Rewrite handle reports the file name in upper case and its bytes", () => {
  const seen = [];
  const vfs = new Vfs(null, (name, data) => seen.push([name, data]));
  const handle = write(vfs, "out.txt", REWRITE, bytes(65, 66), bytes(67));
  assert.deepEqual(seen, [], "nothing is reported before Close");
  assert.equal(vfs.close(handle), 0);
  assert.deepEqual(seen, [["OUT.TXT", bytes(65, 66, 67)]]);
});

test("an Append reports the whole file, and a Rewrite without writes reports an empty one", () => {
  const seen = [];
  const vfs = new Vfs(new Map([["LOG.TXT", bytes(1, 2)]]), (name, data) => seen.push([name, data]));
  vfs.close(write(vfs, "LOG.TXT", APPEND, bytes(3)));
  vfs.close(vfs.open("NEW.TXT", REWRITE));
  assert.deepEqual(seen, [["LOG.TXT", bytes(1, 2, 3)], ["NEW.TXT", new Uint8Array(0)]]);
});

test("closing a file opened for reading reports nothing", () => {
  const seen = [];
  const vfs = new Vfs(new Map([["A.TXT", bytes(1)]]), (...args) => seen.push(args));
  vfs.close(vfs.open("A.TXT", RESET));
  assert.deepEqual(seen, []);
});

test("a failed close or a second close reports nothing", () => {
  const seen = [];
  const vfs = new Vfs(null, (...args) => seen.push(args));
  const handle = write(vfs, "A.TXT", REWRITE, bytes(1));
  vfs.close(handle);
  assert.equal(vfs.close(handle), -6);
  assert.equal(vfs.close(77), -6);
  assert.equal(seen.length, 1);
});

test("the reported bytes are a copy: changing them does not change the file", () => {
  const vfs = new Vfs(null, (_name, data) => data.fill(0));
  vfs.close(write(vfs, "A.TXT", REWRITE, bytes(5, 6)));
  const handle = vfs.open("A.TXT", RESET);
  assert.deepEqual(vfs.read(handle, 10), bytes(5, 6));
});

test("a Vfs without a hook still works", () => {
  const vfs = new Vfs(null);
  vfs.close(write(vfs, "A.TXT", REWRITE, bytes(5)));
});

test("a Host passes onWritten to its file system: fs_open, fs_write, fs_close", () => {
  const seen = [];
  const host = new Host({
    write() {}, input: null, keys: null, clock: null,
    onWritten: (name, data) => seen.push([name, data]),
  });
  const memory = new WebAssembly.Memory({ initial: 1 });
  host.attach({ exports: { memory } });
  const heap = new Uint8Array(memory.buffer);
  heap.set([..."SAVED.TXT"].map((c) => c.charCodeAt(0)), 0);
  heap.set(bytes(104, 105), 32);
  const handle = host.imports.fs_open(0, 9, REWRITE);
  host.imports.fs_write(handle, 32, 2);
  host.imports.fs_close(handle);
  assert.deepEqual(seen, [["SAVED.TXT", bytes(104, 105)]]);
});

test("the test-only paths are gone from host/: no compile stub, no LineInput", () => {
  const host = new Host({ write() {}, input: null, keys: null, clock: null });
  assert.equal("compile" in host.imports, false);
  assert.equal("LineInput" in vfsModule, false);
});

await runTests(tests);
