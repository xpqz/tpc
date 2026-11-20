// The extension's behaviour: diagnostics from the wasm compiler, shown as the editor's
// problems. `vscode` is the API object handed in by extension.cjs.
import fs from "node:fs";
import path from "node:path";
import { CompilerService } from "./compiler_service.mjs";
import { compileEntry } from "./entry.mjs";
import { decodeLatin1, encodeLatin1 } from "./latin1.mjs";
import { compilerPath } from "./locate.mjs";
import { splitLines } from "./positions.mjs";
import { buildReport } from "./report.mjs";
import { debouncer } from "./scheduler.mjs";
import { selectSources } from "./sources.mjs";

const DEBOUNCE_MS = 300;

let disposables = [];

export function activate(vscode, context) {
  const diagnostics = vscode.languages.createDiagnosticCollection("tp5");
  const shownBy = new Map(); // main file path -> file paths it has diagnostics on
  let service; // the CompilerService, created on first use and after a setting change

  const settings = () => vscode.workspace.getConfiguration("tp5");

  async function compilerService() {
    if (!service) {
      const wasmPath = compilerPath(settings().get("compilerPath"), context.extensionPath);
      service = await CompilerService.fromFile(wasmPath);
    }
    return service;
  }

  // The open editor documents of `directory`, by file name.
  function openBuffers(directory) {
    const buffers = new Map();
    for (const doc of vscode.workspace.textDocuments) {
      if (doc.uri.scheme === "file" && path.dirname(doc.fileName) === directory) {
        buffers.set(path.basename(doc.fileName), doc.getText());
      }
    }
    return buffers;
  }

  function listDirectory(directory) {
    try {
      return fs.readdirSync(directory);
    } catch {
      return [];
    }
  }

  async function check(document) {
    const mainPath = document.fileName;
    const directory = path.dirname(mainPath);
    const fileName = path.basename(mainPath);
    const { mainName, mainText } = compileEntry(fileName, document.getText());
    const buffers = openBuffers(directory);
    const readDisk = (name) => fs.readFileSync(path.join(directory, name));
    const diskNames = listDirectory(directory);
    const files = selectSources({ mainName, diskNames, readDisk, buffers });

    let result;
    try {
      result = (await compilerService()).compile(mainName, encodeLatin1(mainText), files);
    } catch (failure) {
      result = { ok: false, internal: `cannot load the compiler: ${failure.message}` };
    }

    const linesOf = (name) => {
      if (name === fileName) return splitLines(document.getText());
      if (buffers.has(name)) return splitLines(buffers.get(name));
      return files.has(name) ? splitLines(decodeLatin1(files.get(name))) : undefined;
    };
    publish(mainPath, directory, buildReport(result, fileName, linesOf));
  }

  // Sets the diagnostics of this compile and clears those it set on other files last time.
  function publish(mainPath, directory, report) {
    const byFile = new Map();
    for (const { file, message, range } of report) {
      const diagnostic = new vscode.Diagnostic(
        new vscode.Range(range.line, range.start, range.line, range.end),
        message,
        vscode.DiagnosticSeverity.Error,
      );
      diagnostic.source = "tp5";
      const filePath = path.join(directory, file);
      byFile.set(filePath, [...(byFile.get(filePath) ?? []), diagnostic]);
    }
    byFile.set(mainPath, byFile.get(mainPath) ?? []);
    for (const stale of shownBy.get(mainPath) ?? []) {
      if (!byFile.has(stale)) diagnostics.delete(vscode.Uri.file(stale));
    }
    for (const [filePath, list] of byFile) diagnostics.set(vscode.Uri.file(filePath), list);
    shownBy.set(mainPath, [...byFile.keys()]);
  }

  // Programs and units are compiled as the main file; an include file alone is not.
  const isCompilable = (document) =>
    document.languageId === "tp5pascal" &&
    document.uri.scheme === "file" &&
    document.fileName.toLowerCase().endsWith(".pas");

  const pending = debouncer(DEBOUNCE_MS, (key) => {
    const document = vscode.workspace.textDocuments.find((d) => d.fileName === key);
    if (document) check(document);
  });

  disposables = [
    diagnostics,
    vscode.workspace.onDidOpenTextDocument((d) => isCompilable(d) && check(d)),
    vscode.workspace.onDidSaveTextDocument((d) => isCompilable(d) && check(d)),
    vscode.workspace.onDidChangeTextDocument(({ document }) => {
      if (isCompilable(document) && settings().get("diagnostics.onType")) {
        pending.schedule(document.fileName);
      }
    }),
    vscode.workspace.onDidCloseTextDocument((document) => {
      pending.cancel(document.fileName);
      for (const shown of shownBy.get(document.fileName) ?? []) {
        diagnostics.delete(vscode.Uri.file(shown));
      }
      shownBy.delete(document.fileName);
    }),
    vscode.workspace.onDidChangeConfiguration((event) => {
      if (event.affectsConfiguration("tp5.compilerPath")) service = undefined;
    }),
  ];
  context.subscriptions.push({ dispose: deactivate });
  vscode.workspace.textDocuments.filter(isCompilable).forEach((d) => check(d));
}

export function deactivate() {
  for (const disposable of disposables) disposable.dispose();
  disposables = [];
}
