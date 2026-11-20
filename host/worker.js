// The program Worker: runs one compiled module with the host runtime. The page sends
// { bytes, files, keys, grid }: the module, the file snapshot of this run, and the two
// SharedArrayBuffer views it shares with the program (key queue, screen bytes).
// Output needs no messages: the program writes the shared screen itself and the page repaints
// it every frame. Only the end of the run is posted back: { done } or { error }.
// Stopping is the page calling Worker.terminate(), which ends even a blocked or spinning
// program.
//
// The IDE is such a program with two more fields in the message: `compiler`, the bytes of
// compiler.wasm, and `cell`, a one-word SharedArrayBuffer view. It then also has the imports
// of IdeHost (ide.js), posts { saved: { name, bytes } } for every file it writes, and runs the
// programs it compiles through the page: ide_run posts { run: { module, files } } and sleeps in
// Atomics.wait on `cell` until the page stores the outcome there (see main.js).
import { Host } from "./runtime.js";
import { readKey, keyPressed } from "./keys.js";
import { Screen } from "./screen.js";
import { Compiler } from "./compiler.js";
import { ideImports } from "./ide.js";

// The browser clock is real: `sleep` really waits. (The test runner's clock is virtual, see
// runtime.js.)
const localNowMs = () => Date.now() - new Date().getTimezoneOffset() * 60000;

// ide_run: the page runs the program. The cell holds 0 until the page answers with the exit code
// plus 2, or 1 when the program was stopped (-1).
function runOnPage(post, cell) {
  return (module, files) => {
    post({ run: { module, files } });
    Atomics.wait(cell, 0, 0);
    const answer = Atomics.load(cell, 0);
    Atomics.store(cell, 0, 0);
    return answer - 2;
  };
}

export async function runProgram({ bytes, files, keys, grid, post, compiler, cell }) {
  // A word nobody notifies: Atomics.wait on it is a sleep that times out.
  const pause = new Int32Array(new SharedArrayBuffer(4));
  const screen = new Screen(grid);
  const host = new Host({
    write: (_fd, bytes) => screen.print(bytes),
    input: null, // the browser has no stdin
    files,
    clock: { nowMs: localNowMs, advance: (ms) => Atomics.wait(pause, 0, 0, ms) },
    keys: { available: () => keyPressed(keys), next: () => readKey(keys) },
    screen,
    onWritten: compiler ? (name, bytes) => post({ saved: { name, bytes } }) : undefined,
    onErased: compiler ? (name) => post({ erased: name }) : undefined,
  });
  // No stdin: -2 tells System to edit the console line from `readkey` (docs/abi.md section 8).
  host.imports.readline = () => -2;
  try {
    if (compiler) {
      const { instance: compilerInstance } = await WebAssembly.instantiate(compiler, {});
      const ide = { compiler: new Compiler(compilerInstance), runModule: runOnPage(post, cell) };
      Object.assign(host.imports, ideImports(host, ide));
    }
    const { instance } = await WebAssembly.instantiate(bytes, { host: host.imports });
    host.attach(instance);
    post({ done: host.runMain(instance) });
  } catch (e) {
    post({ error: e.message });
  }
}

// Only inside a real Worker; the test runner imports runProgram and calls it itself.
if (typeof WorkerGlobalScope !== "undefined") {
  onmessage = ({ data }) => runProgram({ ...data, post: postMessage });
}
