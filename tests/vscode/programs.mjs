// Compiles the test programs of tests/vscode/programs/ (and tests/programs/) with the native
// tp5c binary, once per process.
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { buildNative, root } from "./build.mjs";

let tp5c;
const compiled = new Map();

export function compileProgram(file) {
  tp5c ??= buildNative();
  if (!compiled.has(file)) {
    fs.mkdirSync(path.join(root, "tmp"), { recursive: true });
    const out = path.join(root, "tmp", `vscode-${path.basename(file, ".pas")}.wasm`);
    execFileSync(tp5c, [file, "-o", out]);
    compiled.set(file, new Uint8Array(fs.readFileSync(out)));
  }
  return compiled.get(file);
}

export const program = (name) => compileProgram(path.join(root, "tests/vscode/programs", `${name}.pas`));
export const askProgram = () => compileProgram(path.join(root, "tests/programs/readln_browser.pas"));

export function scratchDirectory(prefix) {
  fs.mkdirSync(path.join(root, "tmp"), { recursive: true });
  return fs.mkdtempSync(path.join(root, "tmp", prefix));
}

export async function until(what, condition, ms = 5000) {
  const start = Date.now();
  while (!condition()) {
    if (Date.now() - start > ms) throw new Error(`timed out waiting for ${what}`);
    await new Promise((resolve) => setTimeout(resolve, 5));
  }
}
