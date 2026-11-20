// The worker thread a program runs in: the host runtime of host/runtime.js with a real clock,
// the shared key queue as the keyboard, and output, file changes and the end of the run posted
// to the extension. It is the Node counterpart of host/worker.js, which draws on a screen grid
// for the browser; a terminal wants the bytes instead.
//
// workerData is { bytes, files, keys }: the compiled module, the file snapshot (Map name ->
// bytes) and the key queue. Messages posted back: { output: bytes }, { written: { name, bytes } },
// { erased: name }, then one of { done: { code, halted } } or { error: message }.
// The thread ends only when the extension terminates it, which also ends a program that is
// blocked in ReadKey or spinning.
import { parentPort, workerData } from "node:worker_threads";
import { Host } from "../../host/runtime.js";
import { keyPressed, readKey } from "../../host/keys.js";

const { bytes, files, keys } = workerData;
const post = (message) => parentPort.postMessage(message);

// A word nobody notifies: Atomics.wait on it is a sleep that times out.
const pause = new Int32Array(new SharedArrayBuffer(4));
const localNowMs = () => Date.now() - new Date().getTimezoneOffset() * 60000;

const host = new Host({
  write: (_fd, output) => post({ output }),
  input: null, // there is no stdin: the line is typed through the key queue
  files,
  clock: { nowMs: localNowMs, advance: (ms) => Atomics.wait(pause, 0, 0, ms) },
  keys: { available: () => keyPressed(keys), next: () => readKey(keys) },
  onWritten: (name, contents) => post({ written: { name, bytes: contents } }),
  onErased: (name) => post({ erased: name }),
});
// -2 tells System to edit the console line from `readkey` and echo it itself (docs/abi.md).
host.imports.readline = () => -2;

try {
  const module = new WebAssembly.Module(bytes);
  const instance = new WebAssembly.Instance(module, { host: host.imports });
  host.attach(instance);
  post({ done: host.runMain(instance) });
} catch (failure) {
  post({ error: failure.message });
}
