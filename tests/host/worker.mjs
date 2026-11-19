// Checks host/worker.js: a program runs in a real worker thread with keys from the shared
// queue, sleeps with Atomics.wait, draws to the shared grid, and stops on terminate even while
// blocked or spinning. Also Screen.print, the teletype that puts console output on the grid. Exit 0 on success.
import assert from "node:assert/strict";
import { Worker } from "node:worker_threads";
import { COLS, ROWS, CURSOR, clearScreen } from "../../host/terminal.js";
import { newKeyQueue, pushKeys } from "../../host/keys.js";
import { Screen } from "../../host/screen.js";
import { echoProgram, spinProgram, emptyProgram, newGrid, rowText, cursorOf, sleepMs, runTests }
  from "./browser_helpers.mjs";

const tests = [];
const test = (name, body) => tests.push([name, body]);

function launch(bytes, files = new Map()) {
  const grid = newGrid();
  clearScreen(grid);
  const keys = newKeyQueue();
  const worker = new Worker(new URL("node_worker.mjs", import.meta.url), {
    workerData: { bytes, files, keys, grid },
  });
  const messages = [];
  worker.on("message", (m) => messages.push(m));
  worker.on("error", (e) => messages.push({ crash: e.message }));
  return { worker, grid, keys, messages };
}

async function until(what, condition, ms = 3000) {
  const start = Date.now();
  while (!condition()) {
    assert.ok(Date.now() - start < ms, `timed out waiting for ${what}`);
    await sleepMs(5);
  }
  return Date.now();
}

test("a program reads queued keys, sleeps at least its delay, and peeks without consuming", async () => {
  const { worker, grid, keys } = launch(echoProgram());
  await sleepMs(200); // the worker is blocked in readkey, so its start-up is not in the measure
  const pushed = Date.now();
  pushKeys(keys, [97, 98]);
  await until("echo", () => rowText(grid, 0).startsWith("a"));
  const peeked = await until("peek result", () => rowText(grid, 0) === "a1");
  // The program starts its sleep(50) only after reading the pushed key, so the sleep begins after
  // `pushed` and the peek result appears after it ends. A late poll only makes the gap longer, so
  // the bound holds however loaded the machine is, and a sleep that returns early breaks it.
  assert.ok(peeked - pushed >= 50, `sleep(50) and the key read took ${peeked - pushed} ms`);
  await worker.terminate();
});

test("readkey blocks with nothing queued, then returns the key that arrives", async () => {
  const { worker, grid, keys, messages } = launch(echoProgram());
  await sleepMs(200);
  assert.equal(rowText(grid, 0), "", "no output without a key");
  assert.deepEqual(messages, []);
  pushKeys(keys, [120]);
  await until("x0", () => rowText(grid, 0) === "x0");
  await worker.terminate();
});

test("a finished program posts its exit status", async () => {
  const { messages } = launch(emptyProgram());
  await until("done", () => messages.length > 0);
  assert.deepEqual(messages, [{ done: { code: 0, halted: false } }]);
});

test("a module that does not instantiate posts an error and the worker survives it", async () => {
  const { messages } = launch(new Uint8Array([1, 2, 3]));
  await until("error", () => messages.length > 0);
  assert.equal(typeof messages[0].error, "string");
  assert.ok(messages[0].error.length > 0);
});

test("terminate stops a program blocked in readkey", async () => {
  const { worker } = launch(echoProgram());
  await sleepMs(100);
  const code = await Promise.race([worker.terminate(), sleepMs(2000).then(() => "hung")]);
  assert.equal(code, 1);
});

test("terminate stops a program spinning in an infinite loop; this thread stays responsive", async () => {
  const { worker } = launch(spinProgram());
  await sleepMs(100);
  let ticks = 0;
  const timer = setInterval(() => ticks++, 10);
  const code = await Promise.race([worker.terminate(), sleepMs(2000).then(() => "hung")]);
  await sleepMs(50);
  clearInterval(timer);
  assert.equal(code, 1);
  assert.ok(ticks >= 3, `only ${ticks} ticks`);
});

const bytes = (s) => Uint8Array.from(s, (c) => c.charCodeAt(0));
function screen() {
  const grid = newGrid();
  clearScreen(grid);
  const model = new Screen(grid);
  return { grid, out: (_fd, text) => model.print(text) };
}

test("print: text, CR, LF and the cursor", () => {
  const { grid, out } = screen();
  out(1, bytes("Hi\r\nX"));
  assert.equal(rowText(grid, 0), "Hi");
  assert.equal(rowText(grid, 1), "X");
  assert.deepEqual(cursorOf(grid), [1, 1]);
  assert.equal(grid[1], 7, "attribute stays light grey on black");
});

test("print: LF alone starts the next row in column 0, fd 2 draws too", () => {
  const { grid, out } = screen();
  out(1, bytes("ab\ncd"));
  out(2, bytes("!"));
  assert.equal(rowText(grid, 1), "cd!");
});

test("print: backspace moves left, stops at column 0, bell is silent", () => {
  const { grid, out } = screen();
  out(1, bytes("ab\b\bc\b\b\b\x07"));
  assert.equal(rowText(grid, 0), "cb");
  assert.deepEqual(cursorOf(grid), [0, 0]);
});

test("print: the 81st character wraps to the next row", () => {
  const { grid, out } = screen();
  out(1, bytes("x".repeat(COLS) + "y"));
  assert.equal(rowText(grid, 0), "x".repeat(COLS));
  assert.equal(rowText(grid, 1), "y");
  assert.deepEqual(cursorOf(grid), [1, 1]);
});

test("print: past the last row the screen scrolls up and the new row is blank", () => {
  const { grid, out } = screen();
  for (let i = 0; i < ROWS + 3; i++) out(1, bytes(`line${i}\r\n`));
  assert.equal(rowText(grid, 0), "line4");
  assert.equal(rowText(grid, ROWS - 2), `line${ROWS + 2}`);
  assert.equal(rowText(grid, ROWS - 1), "");
  assert.deepEqual(cursorOf(grid), [0, ROWS - 1]);
  assert.equal(grid[CURSOR - 1], 7, "scrolled-in row has the default attribute");
});

test("print: filling the last cell scrolls at once", () => {
  const { grid, out } = screen();
  out(1, bytes("top\r\n"));
  out(1, bytes("\n".repeat(ROWS - 2)));
  out(1, bytes("z".repeat(COLS)));
  assert.equal(rowText(grid, ROWS - 2), "z".repeat(COLS));
  assert.equal(rowText(grid, 0), "");
  assert.deepEqual(cursorOf(grid), [0, ROWS - 1]);
});

await runTests(tests);
