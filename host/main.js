// The page: owns the shared screen and key queue, starts and stops the program Worker.
// `?wasm=URL` names the program to run.
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
