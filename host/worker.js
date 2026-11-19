// The program Worker: runs one compiled module with the host runtime. The page sends
// { bytes, files, keys, grid }: the module, the file snapshot of this run, and the two
// SharedArrayBuffer views it shares with the program (key queue, screen bytes).
// Output needs no messages: the program writes the shared screen itself and the page repaints
// it every frame. Only the end of the run is posted back: { done } or { error }.
// Stopping is the page calling Worker.terminate(), which ends even a blocked or spinning
// program.
import { Host } from "./runtime.js";
import { readKey, keyPressed } from "./keys.js";
import { COLS, ROWS, CURSOR, DEFAULT_ATTR } from "./terminal.js";
import { LineInput } from "./vfs.js";

const BEL = 7;
const BS = 8;
const LF = 10;
const CR = 13;

// Console output for programs that do not use the Crt screen: a teletype that writes the
// shared screen at the cursor (kept in the screen bytes), wrapping and scrolling at the end.
export function teletype(cells) {
  const scroll = () => {
    cells.copyWithin(0, COLS * 2, CURSOR);
    for (let at = CURSOR - COLS * 2; at < CURSOR; at += 2) {
      cells[at] = 0x20;
      cells[at + 1] = DEFAULT_ATTR;
    }
  };
  return (_fd, bytes) => {
    for (const byte of bytes) {
      let [col, row] = [cells[CURSOR], cells[CURSOR + 1]];
      if (byte === CR) col = 0;
      else if (byte === LF) row++;
      else if (byte === BS) col = Math.max(col - 1, 0);
      else if (byte !== BEL) {
        cells[(row * COLS + col) * 2] = byte;
        if (++col === COLS) [col, row] = [0, row + 1];
      }
      if (row === ROWS) {
        scroll();
        row = ROWS - 1;
      }
      [cells[CURSOR], cells[CURSOR + 1]] = [col, row];
    }
  };
}

const localNowMs = () => Date.now() - new Date().getTimezoneOffset() * 60000;

export async function runProgram({ bytes, files, keys, grid, post }) {
  const pause = new Int32Array(new SharedArrayBuffer(4)); // never notified: waits time out
  const host = new Host({
    write: teletype(grid),
    input: new LineInput(new Uint8Array(0)),
    files,
    nowMs: localNowMs,
  });
  Object.assign(host.imports, {
    readkey: () => readKey(keys),
    keypressed: () => keyPressed(keys),
    sleep: (ms) => Atomics.wait(pause, 0, 0, ms),
  });
  try {
    const { instance } = await WebAssembly.instantiate(bytes, { host: host.imports });
    host.attach(instance);
    post({ done: host.runMain(instance) });
  } catch (e) {
    post({ error: e.message });
  }
}

if (typeof WorkerGlobalScope !== "undefined") {
  onmessage = ({ data }) => runProgram({ ...data, post: postMessage });
}
