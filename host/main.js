// The page: owns the shared screen and key queue, starts and stops the program Worker.
// `?wasm=URL` names the program to run.
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
import { Terminal, GRID_BYTES, clearScreen } from "./terminal.js";
import { newKeyQueue, pushKeys, keyBytes } from "./keys.js";

const grid = new Uint8Array(new SharedArrayBuffer(GRID_BYTES));
const status = document.getElementById("status");
let keys = newKeyQueue();
let worker = null;
let program = null;

function stop(message) {
  worker?.terminate();
  worker = null;
  status.textContent = message;
}

function run(bytes) {
  stop("running");
  clearScreen(grid);
  keys = newKeyQueue();
  worker = new Worker(new URL("./worker.js", import.meta.url), { type: "module" });
  worker.onmessage = ({ data }) => stop(data.error ?? `finished, exit code ${data.done.code}`);
  worker.postMessage({ bytes, files: new Map(), keys, grid });
}

document.getElementById("run").onclick = () => program && run(program);
document.getElementById("stop").onclick = () => stop("stopped");
document.addEventListener("keydown", (event) => {
  const bytes = worker && !event.metaKey ? keyBytes(event) : [];
  if (bytes.length > 0 && pushKeys(keys, bytes)) event.preventDefault();
});

clearScreen(grid);
new Terminal(document.getElementById("screen"), grid).start();
const url = new URLSearchParams(location.search).get("wasm");
if (url) {
  program = new Uint8Array(await (await fetch(url)).arrayBuffer());
  run(program);
}
