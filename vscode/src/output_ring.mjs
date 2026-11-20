// The channel a program's output takes from the worker to the extension: a ring of bytes in a
// SharedArrayBuffer with one writer (the worker) and one reader (the extension, on a timer).
// The worker runs the program in one synchronous call, so it has no timer and could not flush a
// batch of writes by itself; a program that prints a line and then computes for a minute would
// keep its line to itself. With a ring the extension reads what is there whenever its timer
// fires, so output arrives in a bounded number of chunks, and a program that writes faster than
// the extension reads waits (the ring is full) instead of filling memory.
//
// HEAD and TAIL count the bytes ever read and written, and wrap as 32-bit integers; the
// capacity is a power of two, so a count's position in the ring is `count & (capacity - 1)`.
const HEAD = 0;
const TAIL = 1;
const DEFAULT_CAPACITY = 1 << 20;

export function newOutputRing(capacity = DEFAULT_CAPACITY) {
  if ((capacity & (capacity - 1)) !== 0) throw new Error("the ring's capacity is a power of two");
  return {
    counts: new Int32Array(new SharedArrayBuffer(8)),
    data: new Uint8Array(new SharedArrayBuffer(capacity)),
  };
}

// Worker side: appends all of `bytes`, waiting for the reader whenever the ring is full.
export function writeOutput({ counts, data }, bytes) {
  const mask = data.length - 1;
  let written = 0;
  while (written < bytes.length) {
    const head = Atomics.load(counts, HEAD);
    const tail = Atomics.load(counts, TAIL);
    const free = data.length - ((tail - head) | 0);
    if (free === 0) {
      Atomics.wait(counts, HEAD, head); // returns at once if the reader has moved on
      continue;
    }
    const count = Math.min(free, bytes.length - written);
    for (let i = 0; i < count; i++) data[(tail + i) & mask] = bytes[written + i];
    written += count;
    Atomics.store(counts, TAIL, (tail + count) | 0);
  }
}

// Extension side: takes everything written so far (an empty array when there is nothing).
export function readOutput({ counts, data }) {
  const mask = data.length - 1;
  const head = Atomics.load(counts, HEAD);
  const available = (Atomics.load(counts, TAIL) - head) | 0;
  const taken = new Uint8Array(available);
  for (let i = 0; i < available; i++) taken[i] = data[(head + i) & mask];
  Atomics.store(counts, HEAD, (head + available) | 0);
  Atomics.notify(counts, HEAD);
  return taken;
}
