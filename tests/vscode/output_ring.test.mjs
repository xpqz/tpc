// The ring a program's output travels in from the worker to the extension.
import test from "node:test";
import assert from "node:assert/strict";
import { newOutputRing, readOutput, writeOutput } from "../../vscode/src/output_ring.mjs";

const bytes = (text) => Uint8Array.from(text, (c) => c.charCodeAt(0));
const text = (taken) => Buffer.from(taken).toString("latin1");

test("what was written is read once, in order", () => {
  const ring = newOutputRing(64);
  writeOutput(ring, bytes("ab"));
  writeOutput(ring, bytes("cd"));
  assert.equal(text(readOutput(ring)), "abcd");
  assert.equal(readOutput(ring).length, 0, "nothing is read twice");
});

test("an empty ring and an empty write give nothing", () => {
  const ring = newOutputRing(16);
  assert.equal(readOutput(ring).length, 0);
  writeOutput(ring, new Uint8Array(0));
  assert.equal(readOutput(ring).length, 0);
});

test("the ring wraps around its end", () => {
  const ring = newOutputRing(16);
  for (let round = 0; round < 20; round++) {
    const chunk = `${round}`.padStart(2, "0").repeat(5); // 10 bytes
    writeOutput(ring, bytes(chunk));
    assert.equal(text(readOutput(ring)), chunk, `round ${round}`);
  }
});

test("a write that exactly fills the ring is read whole", () => {
  const ring = newOutputRing(8);
  writeOutput(ring, bytes("12345678"));
  assert.equal(text(readOutput(ring)), "12345678");
});

test("the capacity must be a power of two", () => {
  assert.throws(() => newOutputRing(100));
});

test("counts that wrap as 32-bit integers still work", () => {
  const ring = newOutputRing(16);
  Atomics.store(ring.counts, 0, 0x7ffffffc);
  Atomics.store(ring.counts, 1, 0x7ffffffc);
  writeOutput(ring, bytes("abcdefgh")); // the tail passes 2^31
  assert.equal(text(readOutput(ring)), "abcdefgh");
  writeOutput(ring, bytes("ij"));
  assert.equal(text(readOutput(ring)), "ij");
});

test("random writes and reads keep every byte in order", () => {
  const ring = newOutputRing(32);
  let seed = 5;
  const next = () => (seed = (seed * 1103515245 + 12345) & 0x7fffffff);
  let sent = "";
  let received = "";
  let unread = 0;
  for (let i = 0; i < 3000; i++) {
    const chunk = String.fromCharCode(...Array.from({ length: next() % 12 }, () => 33 + (next() % 90)));
    sent += chunk;
    writeOutput(ring, bytes(chunk));
    unread += chunk.length;
    // A single thread cannot wait for itself: read before the ring could fill.
    if (unread >= 16 || next() % 3 === 0) {
      received += text(readOutput(ring));
      unread = 0;
    }
  }
  received += text(readOutput(ring));
  assert.equal(received, sent);
});
