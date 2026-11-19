// The `host` import module the compiled programs call (docs/abi.md section 8). Everything
// that touches the outside world is injected, so the same code serves the test runner and
// the browser:
//   write(fd, bytes)  receives console output (fd 1) and diagnostics (fd 2)
//   input             has readLine(max), the console line source (null at the end)
//   files             maps file names to bytes, the starting file system (see vfs.js)
//   clock             { nowMs(), advance(ms) }: the local time in milliseconds since
//                     1970-01-01 00:00:00, and a way to move it forward (`sleep`)
//   keys              { available(), next() }: the keyboard as ReadKey bytes; `next` blocks,
//                     or returns null when no key can come any more
//   screen            the Screen grid (default: a private one), kept as `host.screen`; a
//                     runner prints its dump after the program ends
import { HostError } from "./error.js";
import { fmtReal } from "./fmt_real.js";
import { Screen } from "./screen.js";
import { Vfs } from "./vfs.js";

export { HostError };

// Thrown by `halt` and `rterror` to unwind the program.
class Halt {
  constructor(code) {
    this.code = code;
  }
}

const MS_PER_DAY = 86400000;
const LATIN1 = (bytes) => String.fromCharCode(...bytes);
const TEXT = (string) => Uint8Array.from(string, (c) => c.charCodeAt(0));

// Exit code of a program that reads a key after the scripted keys ran out.
export const KEYS_EXHAUSTED = 255;

// Imports no environment provides yet.
const UNAVAILABLE = ["compile"];

export class Host {
  constructor({ write, input, files = null, clock, keys, screen = new Screen() }) {
    this.sink = write;
    this.input = input;
    this.vfs = new Vfs(files);
    this.clock = clock;
    this.keys = keys;
    this.screen = screen;
    this.memory = null;
    this.reading = 0; // the clock reading `time(0)` took
    this.imports = this.makeImports();
  }

  // Binds the program's memory; call before `runMain`.
  attach(instance) {
    this.memory = instance.exports.memory;
  }

  // Calls the program's `main`. Returns { code, halted }: `halted` tells whether `halt` or a
  // runtime error ended the program early. Stack exhaustion (the engine's RangeError) is
  // runtime error 202.
  runMain(instance) {
    try {
      instance.exports.main();
      return { code: 0, halted: false };
    } catch (e) {
      if (e instanceof Halt) return { code: e.code, halted: true };
      if (e instanceof RangeError) {
        this.reportRuntimeError(202);
        return { code: 202, halted: true };
      }
      throw e;
    }
  }

  reportRuntimeError(code) {
    this.sink(1, TEXT(`Runtime error ${String(code).padStart(3, "0")}\n`));
  }

  // A view of `len` bytes of program memory at `ptr`; the view is only valid until the
  // program next runs, because memory can grow.
  bytesAt(what, ptr, len) {
    if (ptr < 0 || len < 0 || ptr + len > this.memory.buffer.byteLength) {
      throw new HostError(`host.${what}: out of bounds (${ptr}, ${len})`);
    }
    return new Uint8Array(this.memory.buffer, ptr, len);
  }

  // `time(0)` takes a reading of the clock and returns its days; 1 and 2 return the seconds
  // since midnight and the hundredths of a second of that same reading (docs/abi.md).
  time(what) {
    if (what === 0) this.reading = this.clock.nowMs();
    const msOfDay = this.reading % MS_PER_DAY;
    if (what === 1) return Math.floor(msOfDay / 1000);
    if (what === 2) return Math.floor((msOfDay % 1000) / 10);
    if (what !== 0) throw new HostError(`host.time: bad argument ${what}`);
    return Math.floor(this.reading / MS_PER_DAY);
  }

  makeImports() {
    const imports = {
      time: (what) => this.time(what),
      sleep: (ms) => this.clock.advance(ms),
      screen: (op, a, b, c) => this.screen.apply(op, a, b, c),
      keypressed: () => (this.keys.available() ? 1 : 0),
      // The next byte of Crt's ReadKey: an extended key is 0, then its scan code.
      readkey: () => {
        const key = this.keys.next();
        if (key === null) throw new Halt(KEYS_EXHAUSTED);
        return key;
      },
      write: (fd, ptr, len) => {
        if (fd !== 1 && fd !== 2) throw new HostError(`host.write: bad fd ${fd}`);
        if (len === 0) return;
        this.sink(fd, this.bytesAt("write", ptr, len).slice());
      },
      halt: (code) => {
        throw new Halt(code);
      },
      fmt_real: (x, width, decimals, ptr) => fmtReal(this.memory, x, width, decimals, ptr),
      rterror: (code) => {
        this.reportRuntimeError(code);
        throw new Halt(code);
      },
      fs_open: (namePtr, nameLen, mode) => {
        const name = LATIN1(this.bytesAt("fs_open", namePtr, nameLen));
        return this.vfs.open(name, mode);
      },
      fs_read: (handle, ptr, len) => {
        const target = this.bytesAt("fs_read", ptr, len);
        const chunk = this.vfs.read(handle, len);
        if (typeof chunk === "number") return chunk;
        target.set(chunk);
        return chunk.length;
      },
      fs_write: (handle, ptr, len) => {
        return this.vfs.write(handle, this.bytesAt("fs_write", ptr, len).slice());
      },
      fs_close: (handle) => this.vfs.close(handle),
      readline: (ptr, max) => {
        const target = this.bytesAt("readline", ptr, max);
        const line = this.input.readLine(max);
        if (line === null) return -1;
        target.set(line);
        return line.length;
      },
    };
    for (const name of UNAVAILABLE) {
      imports[name] = () => {
        throw new HostError(`host import ${name} not available in test runner`);
      };
    }
    return imports;
  }
}
