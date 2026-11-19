// Checks host/keys.js: the SharedArrayBuffer key ring (blocking read, peek, overflow) and the
// mapping from browser key events to the bytes Crt's ReadKey returns. Exit 0 on success.
import assert from "node:assert/strict";
import { Worker } from "node:worker_threads";
import { newKeyQueue, pushKeys, readKey, keyPressed, keyBytes } from "../../host/keys.js";
import { runTests, sleepMs } from "./browser_helpers.mjs";

const tests = [];
const test = (name, body) => tests.push([name, body]);

test("keys come out in the order they went in", () => {
  const q = newKeyQueue();
  assert.equal(pushKeys(q, [65, 66]), true);
  assert.equal(pushKeys(q, [67]), true);
  assert.deepEqual([readKey(q), readKey(q), readKey(q)], [65, 66, 67]);
});

test("keypressed peeks: 0 when empty, 1 while a key waits, and it does not consume", () => {
  const q = newKeyQueue();
  assert.equal(keyPressed(q), 0);
  pushKeys(q, [7]);
  assert.equal(keyPressed(q), 1);
  assert.equal(keyPressed(q), 1);
  assert.equal(readKey(q), 7);
  assert.equal(keyPressed(q), 0);
});

test("the ring wraps around: 500 keys through a queue that holds fewer", () => {
  const q = newKeyQueue();
  for (let i = 0; i < 500; i++) {
    assert.equal(pushKeys(q, [i % 256]), true);
    assert.equal(readKey(q), i % 256);
  }
});

test("a full queue refuses a key and keeps what it holds; an extended key is all or nothing", () => {
  const q = newKeyQueue();
  let n = 0;
  while (pushKeys(q, [n % 256])) n++;
  assert.ok(n >= 16 && n < 1000, `capacity ${n}`);
  assert.equal(pushKeys(q, [99]), false);
  assert.equal(readKey(q), 0);
  assert.equal(pushKeys(q, [0, 72]), false, "one free slot cannot hold two bytes");
  assert.equal(readKey(q), 1);
  assert.equal(pushKeys(q, [0, 72]), true);
  for (let i = 2; i < n; i++) assert.equal(readKey(q), i % 256);
  assert.deepEqual([readKey(q), readKey(q)], [0, 72]);
});

test("readkey blocks in another thread until a key is pushed", async () => {
  const q = newKeyQueue();
  const worker = new Worker(
    "import {parentPort,workerData as q} from 'node:worker_threads';" +
      "import {readKey} from '" + new URL("../../host/keys.js", import.meta.url).href + "';" +
      "parentPort.postMessage(readKey(q));",
    { eval: true, workerData: q },
  );
  let got = null;
  worker.on("message", (m) => (got = m));
  await sleepMs(150);
  assert.equal(got, null, "readKey returned with nothing queued");
  pushKeys(q, [42]);
  for (let i = 0; i < 100 && got === null; i++) await sleepMs(10);
  assert.equal(got, 42);
  await worker.terminate();
});

const ev = (key, extra = {}) => keyBytes({ key, ctrlKey: false, ...extra });

test("keyBytes: printable, control and CP437 characters", () => {
  assert.deepEqual(ev("a"), [97]);
  assert.deepEqual(ev(" "), [32]);
  assert.deepEqual(ev("Enter"), [13]);
  assert.deepEqual(ev("Backspace"), [8]);
  assert.deepEqual(ev("Tab"), [9]);
  assert.deepEqual(ev("Escape"), [27]);
  assert.deepEqual(ev("c", { ctrlKey: true }), [3]);
  assert.deepEqual(ev("é"), [130]);
  assert.deepEqual(ev("ß"), [225]);
});

test("keyBytes: extended keys are 0 then the scan code", () => {
  assert.deepEqual(ev("ArrowUp"), [0, 72]);
  assert.deepEqual(ev("ArrowDown"), [0, 80]);
  assert.deepEqual(ev("ArrowLeft"), [0, 75]);
  assert.deepEqual(ev("ArrowRight"), [0, 77]);
  assert.deepEqual(ev("Home"), [0, 71]);
  assert.deepEqual(ev("Delete"), [0, 83]);
  assert.deepEqual(ev("F1"), [0, 59]);
  assert.deepEqual(ev("F10"), [0, 68]);
});

test("keyBytes: modifiers alone, unmapped keys and characters outside CP437 give nothing", () => {
  for (const key of ["Shift", "Control", "Alt", "CapsLock", "F13", "€", "Dead", "ContextMenu"]) {
    assert.deepEqual(ev(key), [], key);
  }
});

await runTests(tests);
