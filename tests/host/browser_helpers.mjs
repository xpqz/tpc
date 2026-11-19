// Shared by the browser-side host tests: tiny assertion runner, grid reading, and hand-built
// wasm programs that import the interactive host functions (no Crt unit exists to compile).
import { COLS, CURSOR, GRID_BYTES, CP437 } from "../../host/terminal.js";

export const newGrid = () => new Uint8Array(new SharedArrayBuffer(GRID_BYTES));
export const rowText = (grid, row) =>
  Array.from({ length: COLS }, (_, c) => CP437[grid[(row * COLS + c) * 2]])
    .join("")
    .trimEnd();
export const cursorOf = (grid) => [grid[CURSOR], grid[CURSOR + 1]];
export const sleepMs = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

export async function runTests(tests) {
  let failed = 0;
  for (const [name, body] of tests) {
    try {
      await body();
    } catch (e) {
      failed++;
      console.error(`FAIL ${name}\n${e.stack}`);
    }
  }
  if (failed) process.exit(1);
  console.log(`${tests.length} tests passed`);
}

const uleb = (n) => (n < 128 ? [n] : [(n & 127) | 128, ...uleb(n >> 7)]);
const section = (id, bytes) => [id, ...uleb(bytes.length), ...bytes];
const name = (s) => [s.length, ...Array.from(s, (c) => c.charCodeAt(0))];
const i32 = (n) => [0x41, n];
const call = (f) => [0x10, f];
const STORE8 = [0x3a, 0, 0];
const ADD = [0x6a];
// Function indexes of the imports, in the order the module declares them.
const WRITE = 0, READKEY = 1, KEYPRESSED = 2, SLEEP = 3, MAIN = 4;
// write(1, 0, 1): the byte at address 0 goes to the console.
const EMIT = [...i32(1), ...i32(0), ...i32(1), ...call(WRITE)];

// A module importing host.write/readkey/keypressed/sleep, with `main` running `body`.
function program(body) {
  const imports = [
    [WRITE, "write", 2], [READKEY, "readkey", 1], [KEYPRESSED, "keypressed", 1], [SLEEP, "sleep", 0],
  ].flatMap(([, n, type]) => [...name("host"), ...name(n), 0, type]);
  return new Uint8Array([
    0, 0x61, 0x73, 0x6d, 1, 0, 0, 0,
    ...section(1, [4, 0x60, 1, 0x7f, 0, 0x60, 0, 1, 0x7f, 0x60, 3, 0x7f, 0x7f, 0x7f, 0, 0x60, 0, 0]),
    ...section(2, [4, ...imports]),
    ...section(3, [1, 3]),
    ...section(5, [1, 0, 1]),
    ...section(7, [2, ...name("main"), 0, MAIN, ...name("memory"), 2, 0]),
    ...section(10, [1, ...uleb(body.length + 2), 0, ...body, 0x0b]),
  ]);
}

// Reads a key, echoes it, sleeps 50 ms, then echoes '1' if another key is waiting, else '0'.
export const echoProgram = () =>
  program([
    ...i32(0), ...call(READKEY), ...STORE8, ...EMIT,
    ...i32(50), ...call(SLEEP),
    ...i32(0), ...call(KEYPRESSED), ...i32(48), ...ADD, ...STORE8, ...EMIT,
  ]);
// loop br 0 end: never returns and never calls the host.
export const spinProgram = () => program([0x03, 0x40, 0x0c, 0, 0x0b]);
export const emptyProgram = () => program([]);
