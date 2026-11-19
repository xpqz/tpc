// Console input in the browser host without Crt (decision 46): host/worker.js answers
// `readline` with -2, so System reads the line from the key queue with its own line editor and
// echoes it on the teletype. The program (tests/programs/readln_browser.pas) reads a String
// and two Integers with ReadLn and writes `hello NAME SUM`; its output for the typed lines
// `Bob` and `12 30` is the one tools/tp5.sh prints: `name? a b? hello Bob 42`.
// Needs the compiler: $TP5C is the path of the tp5c binary. Exit 0 on success.
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import { Worker } from "node:worker_threads";
import { clearScreen } from "../../host/terminal.js";
import { newKeyQueue, pushKeys } from "../../host/keys.js";
import { newGrid, rowText, cursorOf, sleepMs, runTests } from "./browser_helpers.mjs";

const PROGRAM = new URL("../programs/readln_browser.pas", import.meta.url).pathname;
const SCRATCH = new URL("../../tmp/console_readln.wasm", import.meta.url).pathname;
fs.mkdirSync(new URL("../../tmp", import.meta.url).pathname, { recursive: true });
execFileSync(process.env.TP5C, [PROGRAM, "-o", SCRATCH]);
const wasm = fs.readFileSync(SCRATCH);

const ENTER = 13, BACKSPACE = 8, ESC = 27;
const typed = (text) => [...text].map((c) => c.charCodeAt(0));

// Runs the program in a worker with `keys` queued; resolves with the grid and the posted
// result once the program ended.
async function run(keyBytes) {
  const grid = newGrid();
  clearScreen(grid);
  const keys = newKeyQueue();
  assert.ok(pushKeys(keys, keyBytes));
  const worker = new Worker(new URL("node_worker.mjs", import.meta.url), {
    workerData: { bytes: wasm, files: new Map(), keys, grid },
  });
  const message = new Promise((resolve) => worker.on("message", resolve));
  const result = await Promise.race([message, sleepMs(5000).then(() => ({ timeout: true }))]);
  await worker.terminate();
  return { grid, result };
}

const rows = (grid) => [0, 1, 2].map((row) => rowText(grid, row));

const tests = [
  ["a string and two numbers are read from typed lines and echoed", async () => {
    const { grid, result } = await run([...typed("Bob"), ENTER, ...typed("12 30"), ENTER]);
    assert.deepEqual(result, { done: { code: 0, halted: false } });
    assert.deepEqual(rows(grid), ["name? Bob", "a b? 12 30", "hello Bob 42"]);
    assert.deepEqual(cursorOf(grid), [0, 3]);
  }],
  ["Backspace removes the last character from the line and the screen", async () => {
    const keys = [...typed("Bxo"), BACKSPACE, BACKSPACE, ...typed("ob"), ENTER,
                  BACKSPACE, ...typed("1 2"), ENTER];
    const { grid, result } = await run(keys);
    assert.equal(result.done?.code, 0);
    assert.deepEqual(rows(grid), ["name? Bob", "a b? 1 2", "hello Bob 3"]);
  }],
  ["Esc clears the line; an extended key and a control key are ignored", async () => {
    const keys = [...typed("Zed"), ESC, ...typed("Al"), 0, 75, 9, ...typed("ice"), ENTER,
                  ...typed("7 8"), ENTER];
    const { grid, result } = await run(keys);
    assert.equal(result.done?.code, 0);
    assert.deepEqual(rows(grid), ["name? Alice", "a b? 7 8", "hello Alice 15"]);
  }],
  ["an empty line gives an empty string", async () => {
    const { grid, result } = await run([ENTER, ...typed("5 5"), ENTER]);
    assert.equal(result.done?.code, 0);
    assert.deepEqual(rows(grid), ["name?", "a b? 5 5", "hello  10"]);
  }],
  ["negative: Backspace on an empty line does nothing, not even on the prompt", async () => {
    const { grid, result } = await run([BACKSPACE, ...typed("A"), ENTER, ...typed("1 1"), ENTER]);
    assert.equal(result.done?.code, 0);
    assert.deepEqual(rows(grid), ["name? A", "a b? 1 1", "hello A 2"]);
  }],
  ["negative: a bad number is runtime error 106, as with the Node runner", async () => {
    const { grid, result } = await run([...typed("Bob"), ENTER, ...typed("x"), ENTER]);
    assert.deepEqual(result, { done: { code: 106, halted: true } });
    assert.equal(rowText(grid, 1), "a b? x");
  }],
];

await runTests(tests);
