// Node side of the IDE host (test code, not counted): wires the draft ide imports into a
// Host, with the real compiler.wasm and a nested, synchronous run of the compiled program on
// the same screen, keyboard and clock. This is the shape tests/run.mjs gets for IDE programs.
import fs from "node:fs";
import { Host } from "../../../host/runtime.js";
import { Compiler } from "./compiler.js";
import { ideImports } from "./ide.js";
import { clearScreen } from "../../../host/terminal.js";

export async function loadCompiler(path) {
  const { instance } = await WebAssembly.instantiate(fs.readFileSync(path), {});
  return new Compiler(instance);
}

// Runs a program inside the IDE's call: a second Host shares the first one's console sink,
// keys, screen and clock; it starts on a cleared screen (decision 40) and its file changes are
// merged back into the IDE's file system (what a browser cannot do, see the decision).
export function nestedRun(outer, { keepFiles }) {
  return (module, files) => {
    clearScreen(outer.screen.cells);
    const inner = new Host({
      write: outer.sink, input: outer.input, files, clock: outer.clock, keys: outer.keys,
      screen: outer.screen,
    });
    const instance = new WebAssembly.Instance(new WebAssembly.Module(module), {
      host: inner.imports,
    });
    inner.attach(instance);
    const { code } = inner.runMain(instance);
    if (keepFiles) for (const [name, file] of inner.vfs.files) outer.vfs.files.set(name, file);
    return code;
  };
}

export function addIdeImports(host, compiler, { keepFiles = true } = {}) {
  Object.assign(host.imports, ideImports(host, { compiler, runModule: nestedRun(host, { keepFiles }) }));
}
