// One run of a compiled program: a worker thread (program_worker.mjs), the keys sent to it, its
// output as terminal text, and the program's directory as its file system.
import { Worker } from "node:worker_threads";
import { newKeyQueue, pushKeys } from "../../host/keys.js";
import { diskWriter, readSnapshot } from "./program_files.mjs";
import { createOutputDecoder } from "./terminal_text.mjs";

export const CRT_MESSAGE = "Programs that use Crt are not supported in the terminal yet.";

// Crt programs draw with the host's `screen` import, which a terminal cannot show.
export function usesCrt(wasm) {
  const imports = WebAssembly.Module.imports(new WebAssembly.Module(wasm));
  return imports.some(({ name }) => name === "screen");
}

// Starts `wasm` in `directory`. `onOutput(text)` receives terminal text. Returns
//   sendKeys(bytes)  queues ReadKey bytes (what does not fit in the queue is dropped),
//   stop()           ends the program wherever it is,
//   running          false once the run has ended,
//   ended            a promise of { kind: "exit", code }, { kind: "stopped" } or
//                    { kind: "crashed", message }, resolved after the closing text went to
//                    `onOutput`.
// With `writeFiles` the program's file changes are written to `directory` as they happen.
export function startRun({ wasm, directory, writeFiles, onOutput }) {
  const keys = newKeyQueue();
  const decode = createOutputDecoder();
  const disk = writeFiles ? diskWriter(directory) : undefined;
  const worker = new Worker(new URL("./program_worker.mjs", import.meta.url), {
    workerData: { bytes: wasm, files: readSnapshot(directory), keys },
  });
  let running = true;
  let resolveEnded;
  const ended = new Promise((resolve) => (resolveEnded = resolve));

  function finish(outcome, closingText) {
    if (!running) return;
    running = false;
    onOutput(`\r\n${closingText}\r\n`);
    resolveEnded(outcome);
    worker.terminate();
  }

  function report(problem) {
    if (problem !== undefined) onOutput(`(${problem})\r\n`);
  }

  const crash = (text) => finish({ kind: "crashed", message: text }, `error: ${text}`);

  worker.on("message", (message) => {
    if (!running) return;
    if (message.output) onOutput(decode(message.output));
    else if (message.written && disk) report(disk.write(message.written.name, message.written.bytes));
    else if (message.erased && disk) report(disk.erase(message.erased));
    else if (message.done) finish({ kind: "exit", code: message.done.code }, `exit code ${message.done.code}`);
    else if (message.error !== undefined) crash(message.error);
  });
  worker.on("error", (failure) => crash(failure.message));
  worker.on("exit", () => crash("the program's thread ended unexpectedly"));

  return {
    sendKeys: (bytes) => pushKeys(keys, bytes),
    stop: () => finish({ kind: "stopped" }, "stopped"),
    get running() {
      return running;
    },
    ended,
  };
}
