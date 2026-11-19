// The key queue between the page and the program Worker: a ring of bytes in a
// SharedArrayBuffer. The page is the only producer (it owns TAIL), the program the only
// consumer (it owns HEAD); both only ever grow, a key lives in slot `count % SLOTS`.
import { CP437 } from "./terminal.js";

const HEAD = 0;
const TAIL = 1;
const FIRST_SLOT = 2;
const SLOTS = 64;

export const newKeyQueue = () => new Int32Array(new SharedArrayBuffer((FIRST_SLOT + SLOTS) * 4));

// Queues all of `bytes`, or none and false if they do not fit (a dropped scan code would
// leave the program with a lone 0).
export function pushKeys(queue, bytes) {
  const tail = Atomics.load(queue, TAIL);
  if (tail - Atomics.load(queue, HEAD) + bytes.length > SLOTS) return false;
  bytes.forEach((byte, i) => (queue[FIRST_SLOT + ((tail + i) % SLOTS)] = byte));
  Atomics.store(queue, TAIL, tail + bytes.length);
  Atomics.notify(queue, TAIL);
  return true;
}

// Crt's KeyPressed: 1 if a key waits, without taking it.
export const keyPressed = (queue) =>
  Atomics.load(queue, HEAD) !== Atomics.load(queue, TAIL) ? 1 : 0;

// Crt's ReadKey: sleeps until a key is queued, then takes it. Atomics.wait returns at once
// when TAIL no longer equals `head`, so a key pushed between the check and the wait is not
// missed.
export function readKey(queue) {
  const head = Atomics.load(queue, HEAD);
  while (Atomics.load(queue, TAIL) === head) Atomics.wait(queue, TAIL, head);
  const byte = queue[FIRST_SLOT + (head % SLOTS)];
  Atomics.store(queue, HEAD, head + 1);
  return byte;
}

const CONTROL = { Enter: 13, Backspace: 8, Tab: 9, Escape: 27 };
// Scan codes of the keys ReadKey reports as 0 followed by the code.
const EXTENDED = { ArrowUp: 72, ArrowLeft: 75, ArrowRight: 77, ArrowDown: 80, Home: 71, End: 79,
  PageUp: 73, PageDown: 81, Insert: 82, Delete: 83 };
const FUNCTION_KEY = /^F([1-9]|10)$/;
const F1_SCAN = 59;
const FIRST_HIGH_CODE = 128; // the first byte above ASCII

// The bytes ReadKey returns for a keydown event; none for keys a DOS program never sees.
export function keyBytes({ key, ctrlKey }) {
  if (Object.hasOwn(CONTROL, key)) return [CONTROL[key]];
  if (Object.hasOwn(EXTENDED, key)) return [0, EXTENDED[key]];
  const fn = FUNCTION_KEY.exec(key);
  if (fn) return [0, F1_SCAN + Number(fn[1]) - 1];
  if (key.length !== 1) return [];
  const code = key.charCodeAt(0);
  if (ctrlKey && /[a-z]/i.test(key)) return [(code | 0x20) - 96];
  if (code < FIRST_HIGH_CODE) return [code];
  const cp437 = CP437.indexOf(key, FIRST_HIGH_CODE);
  return cp437 < 0 ? [] : [cp437];
}
