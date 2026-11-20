// The commands Compile, Run and Stop. `vscode` is the API object handed in by extension.mjs;
// `compileDocument(document)` compiles a document (and publishes its diagnostics) and returns
// the compile result of compiler_service.mjs.
import fs from "node:fs";
import path from "node:path";
import { createRunPty } from "./run_terminal.mjs";
import { CRT_MESSAGE, usesCrt } from "./run_session.mjs";

// NAME.pas -> NAME.wasm, in the same directory.
export const wasmPathOf = (sourcePath) => sourcePath.replace(/\.pas$/i, ".wasm");

// The text of a failed compile, as TP showed it: NAME.PAS(line): Error N: message.
export function errorMessage(result) {
  if (result.internal !== undefined) return `Internal compiler error: ${result.internal}`;
  const { file, line, code, message } = result.error;
  return `${file}(${line}): Error ${code}: ${message}`;
}

export function registerRunCommands(vscode, { compileDocument, isCompilable, settings }) {
  const runs = new Set(); // the pseudoterminals of programs that were started

  // The compiled module of the active Pascal file, written beside it; undefined (after saying
  // why) when it does not compile.
  async function compile(document) {
    const result = await compileDocument(document);
    const wasmPath = wasmPathOf(document.fileName);
    if (!result.ok) {
      fs.rmSync(wasmPath, { force: true });
      vscode.window.showErrorMessage(errorMessage(result));
      return undefined;
    }
    fs.writeFileSync(wasmPath, result.wasm);
    vscode.window.setStatusBarMessage(`Wrote ${path.basename(wasmPath)}`, 3000);
    return result.wasm;
  }

  function activeProgram() {
    const document = vscode.window.activeTextEditor?.document;
    return document && isCompilable(document) ? document : undefined;
  }

  async function run() {
    const document = activeProgram();
    const wasm = document && (await compile(document));
    if (wasm === undefined) return;
    if (usesCrt(wasm)) {
      vscode.window.showErrorMessage(CRT_MESSAGE);
      return;
    }
    const pty = createRunPty(vscode.EventEmitter, {
      wasm,
      directory: path.dirname(document.fileName),
      writeFiles: settings().get("run.writeFiles"),
    });
    runs.add(pty);
    vscode.window.createTerminal({ name: `TP5: ${path.basename(document.fileName)}`, pty }).show();
  }

  function stop() {
    for (const pty of runs) pty.stop();
    runs.clear();
  }

  const compileActive = async () => {
    const document = activeProgram();
    if (document) await compile(document);
  };
  const register = (name, handler) => vscode.commands.registerCommand(name, handler);
  return [
    register("tp5.compile", compileActive),
    register("tp5.run", run),
    register("tp5.stop", stop),
  ];
}
