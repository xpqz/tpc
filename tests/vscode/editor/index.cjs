// The in-editor tests: run inside a real VS Code by tests/vscode/editor/run.mjs, against the
// real vscode API. The workspace folder is the scratch directory the runner made and names in
// TP5_EDITOR_SCRATCH; programs are written there.
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const vscode = require("vscode");

const scratch = process.env.TP5_EDITOR_SCRATCH;
const EXTENSION_ID = "xpqz.tp5-pascal";

async function waitFor(what, condition, timeoutMs = 15000) {
  const deadline = Date.now() + timeoutMs;
  for (;;) {
    const value = await condition();
    if (value) return value;
    if (Date.now() > deadline) throw new Error(`timed out waiting for ${what}`);
    await new Promise((resolve) => setTimeout(resolve, 50));
  }
}

async function open(name, text) {
  const file = path.join(scratch, name);
  fs.writeFileSync(file, text);
  const document = await vscode.workspace.openTextDocument(vscode.Uri.file(file));
  await vscode.window.showTextDocument(document);
  return document;
}

const tp5Diagnostics = (uri) =>
  vscode.languages.getDiagnostics(uri).filter((diagnostic) => diagnostic.source === "tp5");

async function replaceAll(document, text) {
  const edit = new vscode.WorkspaceEdit();
  const all = new vscode.Range(0, 0, document.lineCount, 0);
  edit.replace(document.uri, document.validateRange(all), text);
  await vscode.workspace.applyEdit(edit);
}

const MISSING_SEMICOLON = "program p;\nvar x: Integer;\nbegin\n  x := 1\n  WriteLn(x);\nend.\n";
const FIXED = "program p;\nvar x: Integer;\nbegin\n  x := 1;\n  WriteLn(x);\nend.\n";

const tests = {
  async "the extension activates for a Pascal file"() {
    await open("activate.pas", FIXED);
    const extension = vscode.extensions.getExtension(EXTENSION_ID);
    assert.ok(extension, "extension is installed");
    await extension.activate();
    assert.equal(extension.isActive, true);
    const document = vscode.window.activeTextEditor.document;
    assert.equal(document.languageId, "tp5pascal");
  },

  async "an error is a tp5 diagnostic on its line, and fixing it clears it"() {
    const document = await open("syntax.pas", MISSING_SEMICOLON);
    const [diagnostic] = await waitFor("the diagnostic", () => {
      const found = tp5Diagnostics(document.uri);
      return found.length > 0 && found;
    });
    assert.equal(diagnostic.message, 'Error 85: ";" expected');
    assert.equal(diagnostic.range.start.line, 4);
    assert.equal(diagnostic.severity, vscode.DiagnosticSeverity.Error);

    await replaceAll(document, FIXED);
    await waitFor("the diagnostic to clear", () => tp5Diagnostics(document.uri).length === 0);
  },

  async "an error in a unit is shown on the unit's file"() {
    const unitText = "unit U1;\ninterface\nprocedure Go;\nimplementation\nprocedure Go;\nbegin\n  Oops := 1;\nend;\nend.\n";
    const unit = await open("U1.pas", unitText);
    const [diagnostic] = await waitFor("the unit diagnostic", () => {
      const found = tp5Diagnostics(unit.uri);
      return found.length > 0 && found;
    });
    assert.equal(diagnostic.message, "Error 3: Unknown identifier");
    assert.equal(diagnostic.range.start.line, 6);
  },

  async "Compile writes NAME.wasm"() {
    const document = await open("hello.pas", "begin\n  WriteLn('Hello');\nend.\n");
    const wasm = path.join(scratch, "hello.wasm");
    fs.rmSync(wasm, { force: true });
    await vscode.commands.executeCommand("tp5.compile");
    const bytes = await waitFor("hello.wasm", () => fs.existsSync(wasm) && fs.readFileSync(wasm));
    assert.deepEqual([...bytes.subarray(0, 4)], [0, 0x61, 0x73, 0x6d]);
    assert.equal(document.isDirty, false);
  },

  async "Compile of a program with an error writes nothing"() {
    await open("broken.pas", MISSING_SEMICOLON);
    const wasm = path.join(scratch, "broken.wasm");
    await vscode.commands.executeCommand("tp5.compile");
    assert.equal(fs.existsSync(wasm), false);
  },

  async "Run opens a terminal named for the program and the program ends"() {
    await open("hello_run.pas", "begin\n  WriteLn('Hello');\nend.\n");
    const before = vscode.window.terminals.length;
    await vscode.commands.executeCommand("tp5.run");
    const terminal = await waitFor("the terminal", () =>
      vscode.window.terminals.find((t) => t.name === "TP5: hello_run.pas"),
    );
    assert.equal(vscode.window.terminals.length, before + 1);
    const { runs } = vscode.extensions.getExtension(EXTENSION_ID).exports;
    const [session] = [...runs].slice(-1);
    await waitFor("the program to end", () => !session.running);
    terminal.dispose();
  },

  async "a Crt program runs until Stop"() {
    await open("crt_run.pas", "uses Crt;\nbegin\n  ClrScr;\n  repeat until KeyPressed;\nend.\n");
    await vscode.commands.executeCommand("tp5.run");
    await waitFor("the terminal", () =>
      vscode.window.terminals.find((t) => t.name === "TP5: crt_run.pas"),
    );
    const { runs } = vscode.extensions.getExtension(EXTENSION_ID).exports;
    const session = [...runs].slice(-1)[0];
    await waitFor("the program to start", () => session.running);
    await new Promise((resolve) => setTimeout(resolve, 500));
    assert.equal(session.running, true, "still running without input");
    await vscode.commands.executeCommand("tp5.stop");
    await waitFor("the program to stop", () => !session.running);
  },
};

async function run() {
  let failed = 0;
  for (const [name, body] of Object.entries(tests)) {
    try {
      await body();
      console.log(`ok - ${name}`);
    } catch (error) {
      failed += 1;
      console.log(`not ok - ${name}\n  ${error.stack}`);
    }
  }
  if (failed > 0) throw new Error(`${failed} of ${Object.keys(tests).length} editor tests failed`);
}

module.exports = { run };
