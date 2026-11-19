// The program Worker: runs one compiled module with the host runtime. The page sends
// { bytes, files, keys, grid }: the module, the file snapshot of this run, and the two
// SharedArrayBuffer views it shares with the program (key queue, screen bytes).
// Output needs no messages: the program writes the shared screen itself and the page repaints
// it every frame. Only the end of the run is posted back: { done } or { error }.
// Stopping is the page calling Worker.terminate(), which ends even a blocked or spinning
// program.
import { Host } from "./runtime.js";
import { readKey, keyPressed } from "./keys.js";
import { Screen } from "./screen.js";
import { LineInput } from "./vfs.js";

// The browser clock is real: `sleep` really waits. (The test runner's clock is virtual, see
// runtime.js.)
const localNowMs = () => Date.now() - new Date().getTimezoneOffset() * 60000;

export async function runProgram({ bytes, files, keys, grid, post }) {
  // A word nobody notifies: Atomics.wait on it is a sleep that times out.
  const pause = new Int32Array(new SharedArrayBuffer(4));
  const screen = new Screen(grid);
  const host = new Host({
    write: (_fd, bytes) => screen.print(bytes),
    input: new LineInput(new Uint8Array(0)),
    files,
    clock: { nowMs: localNowMs, advance: (ms) => Atomics.wait(pause, 0, 0, ms) },
    keys: { available: () => keyPressed(keys), next: () => readKey(keys) },
    screen,
  });
  try {
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
