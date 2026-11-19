// Console line ends in the browser host without Crt: WriteLn ends with LF only
// (the Node runner's output stays LF), so the teletype must treat LF as a new line starting in
// column 1, as a terminal does. The program (tests/programs/console_newline.pas) writes
// `first` with WriteLn, `second` with Write and WriteLn, then `third`; tools/tp5.sh prints
// the rows `first`, `second`, `third`. Needs the compiler: $TP5C is the path of the tp5c
// binary. Exit 0 on success.
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import { Worker } from "node:worker_threads";
import { clearScreen } from "../../host/terminal.js";
import { newKeyQueue } from "../../host/keys.js";
import { newGrid, rowText, sleepMs } from "./browser_helpers.mjs";

const PROGRAM = new URL("../programs/console_newline.pas", import.meta.url).pathname;
const SCRATCH = new URL("../../tmp/console_newline.wasm", import.meta.url).pathname;
fs.mkdirSync(new URL("../../tmp", import.meta.url).pathname, { recursive: true });
execFileSync(process.env.TP5C, [PROGRAM, "-o", SCRATCH]);

const grid = newGrid();
clearScreen(grid);
const worker = new Worker(new URL("node_worker.mjs", import.meta.url), {
  workerData: { bytes: fs.readFileSync(SCRATCH), files: new Map(), keys: newKeyQueue(), grid },
});
const message = new Promise((resolve) => worker.on("message", resolve));
const result = await Promise.race([message, sleepMs(5000).then(() => ({ timeout: true }))]);
await worker.terminate();

assert.deepEqual(result, { done: { code: 0, halted: false } });
const rows = [0, 1, 2, 3].map((row) => rowText(grid, row));
assert.ok(rows[0] !== "", "the program wrote something");
assert.deepEqual(rows, ["first", "second", "third", ""]);
console.log("console_newline: ok");
