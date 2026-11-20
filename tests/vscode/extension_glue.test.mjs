// Drives activate() with a minimal fake of the vscode API: opening a document with an error
// publishes one diagnostic "Error N: text" with source tp5 on that file, and fixing it clears it.
import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { buildWasm, root } from "./build.mjs";

function fakeVscode(documents, compilerPath) {
  const shown = new Map();
  const handlers = {};
  const event = (name) => (handler) => ((handlers[name] = handler), { dispose() {} });
  return {
    shown,
    handlers,
    api: {
      languages: {
        createDiagnosticCollection: () => ({
          set: (uri, list) => shown.set(uri.fsPath, list),
          delete: (uri) => shown.delete(uri.fsPath),
          dispose() {},
        }),
      },
      workspace: {
        textDocuments: documents,
        getConfiguration: () => ({
          get: (key) => ({ compilerPath, "diagnostics.onType": true })[key],
        }),
        onDidOpenTextDocument: event("open"),
        onDidSaveTextDocument: event("save"),
        onDidChangeTextDocument: event("change"),
        onDidCloseTextDocument: event("close"),
        onDidChangeConfiguration: event("config"),
      },
      Uri: { file: (fsPath) => ({ fsPath }) },
      Range: class {
        constructor(line, start, endLine, end) {
          Object.assign(this, { line, start, endLine, end });
        }
      },
      Diagnostic: class {
        constructor(range, message, severity) {
          Object.assign(this, { range, message, severity });
        }
      },
      DiagnosticSeverity: { Error: 0 },
    },
  };
}

const document = (fileName, text) => ({
  fileName,
  languageId: "tp5pascal",
  uri: { scheme: "file" },
  getText: () => text,
});

test("an error in a document is published, and a fix clears it", async () => {
  const dir = fs.mkdtempSync(path.join(root, "tmp/vscode-glue-"));
  try {
    const main = path.join(dir, "main.pas");
    const doc = document(main, "program p;\nbegin\n  oops := 1;\nend.\n");
    const { api, shown, handlers } = fakeVscode([doc], buildWasm());
    const { activate, deactivate } = await import("../../vscode/src/extension.mjs");
    activate(api, { extensionPath: path.join(root, "vscode"), subscriptions: [] });
    await new Promise((resolve) => setTimeout(resolve, 500));
    const [diagnostic] = shown.get(main);
    assert.equal(diagnostic.message, "Error 3: Unknown identifier");
    assert.equal(diagnostic.source, "tp5");
    assert.equal(diagnostic.range.line, 2);

    doc.getText = () => "program p;\nbegin\nend.\n";
    await handlers.save(doc);
    assert.deepEqual(shown.get(main), []);

    deactivate();
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});
