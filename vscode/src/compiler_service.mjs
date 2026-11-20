// Runs the wasm compiler. The module is compiled once; each compile gets a fresh instance,
// because the compiler aborts on a panic (the instance is then unusable) and the host's
// Compiler keeps every file ever registered.
import fs from "node:fs";
import { Compiler } from "./host.mjs";
import { decodeLatin1 } from "./latin1.mjs";

export class CompilerService {
  constructor(module) {
    this.module = module;
  }

  // Compiled synchronously: the asynchronous WebAssembly.compile hands the work to V8's
  // background threads, and on Node 23 it sometimes never completes (see tests/run.mjs).
  static async fromFile(path) {
    return new CompilerService(new WebAssembly.Module(fs.readFileSync(path)));
  }

  // Compiles `main` (bytes) as the file `mainName`, with `files` (Map name -> bytes) as the
  // units and includes. Returns { ok: true, wasm }, { ok: false, error } with the compiler's
  // error and its message text, or { ok: false, internal } if the compiler trapped.
  compile(mainName, main, files) {
    const instance = new WebAssembly.Instance(this.module);
    try {
      const result = new Compiler(instance).compile(mainName, main, files);
      if (result.module) return { ok: true, wasm: result.module };
      const { code, line, col } = result;
      const file = this.nameAsGiven(result.file, mainName, files);
      return { ok: false, error: { code, line, col, file, message: this.message(instance) } };
    } catch (trap) {
      return { ok: false, internal: trap.message };
    }
  }

  // The compiler reports upper-case registered names; the caller knows them as given.
  nameAsGiven(reported, mainName, files) {
    if (reported === mainName) return mainName;
    const sameName = (name) => name.toUpperCase() === reported.toUpperCase();
    return [...files.keys()].find(sameName) ?? reported;
  }

  message(instance) {
    const { memory, err_message_ptr, err_message_len } = instance.exports;
    return decodeLatin1(new Uint8Array(memory.buffer, err_message_ptr(), err_message_len()));
  }
}
