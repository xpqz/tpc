// The batch that collects a program's output writes before the worker posts them.
import test from "node:test";
import assert from "node:assert/strict";
import { createOutputBatch } from "../../vscode/src/output_batch.mjs";

function setup(options = {}) {
  const posted = [];
  const clock = { time: 0 };
  const batch = createOutputBatch({
    post: (bytes) => posted.push(Buffer.from(bytes).toString("latin1")),
    now: () => clock.time,
    intervalMs: 20,
    limitBytes: 100,
    ...options,
  });
  return { batch, posted, clock };
}
const bytes = (text) => Uint8Array.from(text, (c) => c.charCodeAt(0));

test("small writes wait; flush posts them as one message in order", () => {
  const { batch, posted } = setup();
  batch.write(bytes("ab"));
  batch.write(bytes("cd"));
  assert.deepEqual(posted, []);
  batch.flush();
  assert.deepEqual(posted, ["abcd"]);
});

test("flush with nothing collected posts nothing", () => {
  const { batch, posted } = setup();
  batch.flush();
  batch.write(bytes("x"));
  batch.flush();
  batch.flush();
  assert.deepEqual(posted, ["x"]);
});

test("reaching the byte limit posts the collected bytes", () => {
  const { batch, posted } = setup();
  batch.write(bytes("a".repeat(60)));
  assert.deepEqual(posted, []);
  batch.write(bytes("b".repeat(40)));
  assert.deepEqual(posted, ["a".repeat(60) + "b".repeat(40)]);
});

test("a write bigger than the limit goes out after what was collected, in order", () => {
  const { batch, posted } = setup();
  batch.write(bytes("first"));
  batch.write(bytes("z".repeat(250)));
  assert.deepEqual(posted, ["first", "z".repeat(250)]);
  batch.flush();
  assert.equal(posted.length, 2);
});

test("a write after the interval has passed posts everything collected", () => {
  const { batch, posted, clock } = setup();
  batch.write(bytes("a"));
  clock.time = 19;
  batch.write(bytes("b"));
  assert.deepEqual(posted, []);
  clock.time = 20;
  batch.write(bytes("c"));
  assert.deepEqual(posted, ["abc"]);
  clock.time = 25;
  batch.write(bytes("d"));
  assert.equal(posted.length, 1, "the interval counts from the last post");
});

test("flushIfDue posts only once the interval has passed", () => {
  const { batch, posted, clock } = setup();
  batch.write(bytes("a"));
  clock.time = 10;
  batch.flushIfDue();
  assert.deepEqual(posted, []);
  clock.time = 20;
  batch.flushIfDue();
  assert.deepEqual(posted, ["a"]);
  batch.flushIfDue();
  assert.equal(posted.length, 1, "nothing left to post");
});

test("the bytes are copied: changing the source afterwards changes nothing", () => {
  const { batch, posted } = setup();
  const source = bytes("hello");
  batch.write(source);
  source.fill(0x58);
  batch.flush();
  assert.deepEqual(posted, ["hello"]);
});

test("empty writes are ignored", () => {
  const { batch, posted } = setup();
  batch.write(new Uint8Array(0));
  batch.flush();
  assert.deepEqual(posted, []);
});

test("many writes of every size come out complete and in order", () => {
  const { batch, posted, clock } = setup();
  let seed = 3;
  const next = () => (seed = (seed * 1103515245 + 12345) & 0x7fffffff);
  let sent = "";
  for (let i = 0; i < 2000; i++) {
    const text = String.fromCharCode(...Array.from({ length: next() % 130 }, () => 32 + (next() % 90)));
    sent += text;
    clock.time += next() % 7;
    batch.write(bytes(text));
    if (next() % 50 === 0) batch.flush();
  }
  batch.flush();
  assert.equal(posted.join(""), sent);
  assert.ok(posted.every((chunk) => chunk.length > 0));
});
