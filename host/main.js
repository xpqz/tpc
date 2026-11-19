// The page: owns the shared screen and key queue, starts and stops the program Worker.
// `?wasm=URL` names the program to run; `&compiler=URL` makes it the IDE and names compiler.wasm.
//
// How the browser host fits together. Two threads: this page and a Worker (worker.js) that
// runs the compiled program. They share two SharedArrayBuffers and send no messages while
// the program runs:
//   screen grid  the Worker's Screen writes cells (a Crt program draws them itself); the page
//                repaints the grid every animation frame (terminal.js)
//   key ring     the page pushes the bytes of each keydown (keys.js); the program's `readkey`
//                import sleeps in Atomics.wait while the ring is empty and the page wakes it
//                with Atomics.notify
// A synchronous program cannot be interrupted, so Stop is Worker.terminate(), which ends even
// a program blocked in `readkey` or spinning. Only the end of the run comes back as a message.
//
// The IDE is the program Worker with the compiler (worker.js). When it runs a program it posts
// { run } and sleeps on `cell`; the page starts a second program Worker on the same screen and
// keys, and answers through the cell: the exit code plus 2 when the program ends, STOPPED when
// the Stop button ends it (Stop then ends only that program; with none running it ends the IDE).
// The IDE's files are the page's localStorage (persist.js): read at load, and written as the
// IDE posts { saved }. A running program gets the IDE's files, and its own writes are dropped.
import { Terminal, GRID_BYTES, clearScreen } from "./terminal.js";
import { newKeyQueue, pushKeys, keyBytes } from "./keys.js";
import { loadFiles, saveFile } from "./persist.js";

const STOPPED = 1;

const grid = new Uint8Array(new SharedArrayBuffer(GRID_BYTES));
const status = document.getElementById("status");
let keys = newKeyQueue();
let worker = null;
let program = null;
let compiler = null;
let cell = null;
let guest = null; // the program started by the IDE, while it runs

const newWorker = () => new Worker(new URL("./worker.js", import.meta.url), { type: "module" });

function stop(message) {
  if (guest) endGuest(STOPPED);
  worker?.terminate();
  worker = null;
  status.textContent = message;
}

// Answers the IDE's ide_run and wakes it.
function endGuest(answer) {
  guest?.terminate();
  guest = null;
  Atomics.store(cell, 0, answer);
  Atomics.notify(cell, 0);
}

function runGuest({ module, files }) {
  clearScreen(grid);
  guest = newWorker();
  guest.onmessage = ({ data }) => {
    if (data.error) status.textContent = data.error;
    endGuest(data.error ? STOPPED : data.done.code + 2);
  };
  guest.postMessage({ bytes: module, files, keys, grid });
}

function onWorkerMessage({ data }) {
  if (data.saved) saveFile(localStorage, data.saved.name, data.saved.bytes);
  else if (data.run) runGuest(data.run);
  else stop(data.error ?? `finished, exit code ${data.done.code}`);
}

function run(bytes) {
  stop("running");
  clearScreen(grid);
  keys = newKeyQueue();
  cell = new Int32Array(new SharedArrayBuffer(4));
  worker = newWorker();
  worker.onmessage = onWorkerMessage;
  worker.postMessage({ bytes, files: loadFiles(localStorage), keys, grid, compiler, cell });
}

document.getElementById("run").onclick = () => program && run(program);
document.getElementById("stop").onclick = () => (guest ? endGuest(STOPPED) : stop("stopped"));
document.addEventListener("keydown", (event) => {
  const bytes = worker && !event.metaKey ? keyBytes(event) : [];
  if (bytes.length > 0 && pushKeys(keys, bytes)) event.preventDefault();
});

clearScreen(grid);
new Terminal(document.getElementById("screen"), grid).start();
const fetchBytes = async (url) => new Uint8Array(await (await fetch(url)).arrayBuffer());
const params = new URLSearchParams(location.search);
if (params.get("compiler")) compiler = await fetchBytes(params.get("compiler"));
if (params.get("wasm")) {
  program = await fetchBytes(params.get("wasm"));
  run(program);
}
