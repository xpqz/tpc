// One run of a compiled program: a worker thread (program_worker.mjs), the keys sent to it, its
// output as terminal text, and the program's directory as its file system. A program that uses
// Crt draws on a grid shared with the worker; the session turns it into frames (screen_ansi.mjs)
// on a timer.
import { Worker } from "node:worker_threads";
import { newKeyQueue, pushKeys, GRID_BYTES, clearScreen } from "./host.mjs";
import { ENTER_SCREEN, LEAVE_SCREEN, createScreenWriter } from "./screen_ansi.mjs";
import { newOutputRing, readOutput } from "./output_ring.mjs";
import { diskWriter, readSnapshot } from "./program_files.mjs";
import { createOutputDecoder } from "./terminal_text.mjs";

const FRAME_INTERVAL_MS = 33; // about 30 frames a second
const OUTPUT_INTERVAL_MS = 20; // how often the output of a plain program is taken

const IMPORT_SECTION = 2;
const IMPORT_FUNCTION = 0;
const MODULE_HEADER_BYTES = 8;

// The module's imports as { module, name, kind }, read from the import section directly.
// Compiling the module here would block this thread on V8's background compiler, which can
// stall; the worker compiles it anyway. A module that is malformed or has other kinds of imports
// than functions gives what could be read.
function importedNames(wasm) {
  let at = MODULE_HEADER_BYTES;
  const byte = () => wasm[at++];
  const number = () => {
    let value = 0;
    for (let shift = 0, next = 0x80; next & 0x80; shift += 7) {
      next = byte();
      value |= (next & 0x7f) << shift;
    }
    return value;
  };
  const text = () => {
    const length = number();
    at += length;
    return String.fromCharCode(...wasm.subarray(at - length, at));
  };
  while (at < wasm.length) {
    const id = byte();
    const size = number();
    if (id !== IMPORT_SECTION) {
      at += size;
      continue;
    }
    const names = [];
    for (let count = number(); count > 0; count--) {
      const imported = { module: text(), name: text(), kind: byte() };
      if (imported.kind !== IMPORT_FUNCTION) return names;
      number(); // the function's type index
      names.push(imported);
    }
    return names;
  }
  return [];
}

// Crt programs draw with the host's `screen` import.
export function usesCrt(wasm) {
  return importedNames(wasm).some(({ name }) => name === "screen");
}

// Starts `wasm` in `directory`. `onOutput(text)` receives terminal text. Returns
//   sendKeys(bytes)  queues ReadKey bytes (what does not fit in the queue is dropped),
//   stop()           ends the program wherever it is,
//   running          false once the run has ended,
//   grid             the screen of a Crt program (undefined for other programs),
//   resize(c, r)     the terminal's size in columns and rows (Crt programs),
//   screenHeld       true while a Crt program's last picture is still shown,
//   showEnd()        leaves the picture and writes the closing text; a Crt program's picture
//                    stays until then, so a program that draws and ends can be seen,
//   ended            a promise of { kind: "exit", code }, { kind: "stopped" } or
//                    { kind: "crashed", message }, resolved after the closing text went to
//                    `onOutput` (for a Crt program: after its last picture did).
// With `writeFiles` the program's file changes are written to `directory` as they happen.
export function startRun({ wasm, directory, writeFiles, onOutput }) {
  const keys = newKeyQueue();
  const decode = createOutputDecoder();
  const disk = writeFiles ? diskWriter(directory) : undefined;
  const ring = newOutputRing();
  const grid = usesCrt(wasm) ? newGrid() : undefined;
  const writer = grid && createScreenWriter();
  const worker = new Worker(new URL("./program_worker.mjs", import.meta.url), {
    workerData: { bytes: wasm, files: readSnapshot(directory), keys, output: ring, grid },
  });
  let running = true;
  let screenHeld = false;
  let closingText = "";
  let resolveEnded;
  const ended = new Promise((resolve) => (resolveEnded = resolve));

  // The program's output is taken from the ring on a timer, and once more when it ends, so
  // a run's output is a few chunks however much it prints.
  function takeOutput() {
    const bytes = readOutput(ring);
    if (bytes.length > 0) onOutput(decode(bytes));
  }
  const outputTimer = setInterval(takeOutput, OUTPUT_INTERVAL_MS);

  function drawFrame() {
    const text = writer.frame(grid);
    if (text !== "") onOutput(text);
  }

  // The Crt screen is drawn only when the grid changed, by the timer and once more at the end.
  let frameTimer;
  if (grid) {
    onOutput(ENTER_SCREEN);
    screenHeld = true;
    frameTimer = setInterval(drawFrame, FRAME_INTERVAL_MS);
  }

  function showEnd() {
    if (!screenHeld) return;
    screenHeld = false;
    onOutput(`${LEAVE_SCREEN}\r\n${closingText}\r\n`);
  }

  function finish(outcome, text) {
    if (!running) return;
    running = false;
    closingText = text;
    clearInterval(frameTimer);
    clearInterval(outputTimer);
    takeOutput();
    if (grid) drawFrame();
    else onOutput(`\r\n${text}\r\n`);
    resolveEnded(outcome);
    worker.terminate();
  }

  function report(problem) {
    if (problem !== undefined) onOutput(`(${problem})\r\n`);
  }

  const crash = (text) => finish({ kind: "crashed", message: text }, `error: ${text}`);

  worker.on("message", (message) => {
    if (!running) return;
    if (message.written && disk) report(disk.write(message.written.name, message.written.bytes));
    else if (message.erased && disk) report(disk.erase(message.erased));
    else if (message.done) finish({ kind: "exit", code: message.done.code }, `exit code ${message.done.code}`);
    else if (message.error !== undefined) crash(message.error);
  });
  worker.on("error", (failure) => crash(failure.message));
  worker.on("exit", () => crash("the program's thread ended unexpectedly"));

  return {
    sendKeys: (bytes) => pushKeys(keys, bytes),
    stop: () => finish({ kind: "stopped" }, "stopped"),
    grid,
    resize(columns, rows) {
      if (!writer) return;
      writer.resize(columns, rows);
      if (screenHeld) drawFrame();
    },
    showEnd,
    get screenHeld() {
      return screenHeld;
    },
    get running() {
      return running;
    },
    ended,
  };
}

// The screen a Crt program draws on: blank, shared with the worker.
function newGrid() {
  const grid = new Uint8Array(new SharedArrayBuffer(GRID_BYTES));
  clearScreen(grid);
  return grid;
}
