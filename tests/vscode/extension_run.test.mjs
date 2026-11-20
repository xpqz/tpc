// The run commands through activate(), against the fake vscode API.
import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { buildNative, buildWasm, root } from "./build.mjs";
import { fakeDocument, fakeVscode } from "./fake_vscode.mjs";
import { scratchDirectory, until } from "./programs.mjs";

const wasmPath = buildWasm();
buildNative();
const { activate, deactivate } = await import("../../vscode/src/extension.mjs");

async function withExtension(source, settings, body, fileName = "main.pas") {
  const directory = scratchDirectory("run-glue-");
  const main = path.join(directory, fileName);
  const document = fakeDocument(main, source);
  const vscode = fakeVscode([document], wasmPath, { settings });
  vscode.activeEditor = { document };
  activate(vscode.api, { extensionPath: path.join(root, "vscode"), subscriptions: [] });
  try {
    await body({ vscode, directory, main, document, run: (name) => vscode.commands.get(name)() });
  } finally {
    deactivate();
    for (const terminal of vscode.terminals) terminal.dispose();
    fs.rmSync(directory, { recursive: true, force: true });
  }
}

const HELLO = "program p;\nbegin\n  WriteLn('Hi');\nend.\n";
const BROKEN = "program p;\nbegin\n  oops := 1;\nend.\n";
const CRT = "program p;\nuses Crt;\nbegin\n  ClrScr;\nend.\n";

test("the three commands are registered", async () => {
  await withExtension(HELLO, {}, ({ vscode }) => {
    for (const name of ["tp5.compile", "tp5.run", "tp5.stop"]) {
      assert.equal(typeof vscode.commands.get(name), "function", name);
    }
  });
});

test("compile writes NAME.wasm beside NAME.pas", async () => {
  await withExtension(HELLO, {}, async ({ vscode, directory, run }) => {
    await run("tp5.compile");
    const wasm = fs.readFileSync(path.join(directory, "main.wasm"));
    assert.deepEqual([...wasm.subarray(0, 4)], [0, 0x61, 0x73, 0x6d]);
    assert.deepEqual(vscode.messages.error, []);
    assert.match(vscode.messages.status.join("\n"), /main\.wasm/);
    assert.equal(vscode.terminals.length, 0, "compile does not run");
  });
});

test("compile with an error: a diagnostic, a message with file, line and error, and no .wasm", async () => {
  await withExtension(BROKEN, {}, async ({ vscode, directory, main, run }) => {
    await run("tp5.compile");
    assert.deepEqual(vscode.messages.error, ["main.pas(3): Error 3: Unknown identifier"]);
    assert.equal(vscode.shown.get(main)[0].message, "Error 3: Unknown identifier");
    assert.equal(fs.existsSync(path.join(directory, "main.wasm")), false);
  });
});

test("compile removes the .wasm of an earlier good build when the source is broken", async () => {
  await withExtension(HELLO, {}, async ({ document, directory, run }) => {
    await run("tp5.compile");
    document.getText = () => BROKEN;
    await run("tp5.compile");
    assert.equal(fs.existsSync(path.join(directory, "main.wasm")), false);
  });
});

test("an error in a unit is named with the unit file", async () => {
  const directory = scratchDirectory("run-glue-");
  try {
    fs.writeFileSync(path.join(directory, "helper.pas"), "unit helper;\ninterface\nimplementation\nbegin\n  oops := 1;\nend.\n");
    const main = path.join(directory, "main.pas");
    const document = fakeDocument(main, "program p;\nuses helper;\nbegin\nend.\n");
    const vscode = fakeVscode([document], wasmPath);
    vscode.activeEditor = { document };
    activate(vscode.api, { extensionPath: path.join(root, "vscode"), subscriptions: [] });
    await vscode.commands.get("tp5.compile")();
    assert.match(vscode.messages.error[0], /^helper\.pas\(\d+\): Error \d+: /);
    deactivate();
  } finally {
    fs.rmSync(directory, { recursive: true, force: true });
  }
});

test("compile and run without a Pascal editor do nothing and do not throw", async () => {
  await withExtension(HELLO, {}, async ({ vscode, run }) => {
    vscode.activeEditor = undefined;
    await run("tp5.compile");
    await run("tp5.run");
    assert.equal(vscode.terminals.length, 0);
    assert.deepEqual(vscode.messages.error, []);
  });
});

test("run compiles, opens a terminal named after the program and shows it", async () => {
  await withExtension(HELLO, {}, async ({ vscode, directory, run }) => {
    await run("tp5.run");
    assert.equal(vscode.terminals.length, 1);
    const [terminal] = vscode.terminals;
    assert.equal(terminal.options.name, "TP5: main.pas");
    assert.equal(terminal.shown, true);
    assert.ok(fs.existsSync(path.join(directory, "main.wasm")));
    const written = [];
    terminal.options.pty.onDidWrite((text) => written.push(text));
    terminal.options.pty.open(undefined);
    await until("the end", () => written.join("").includes("exit code 0"));
    assert.ok(written.join("").startsWith("Hi\r\n"));
  });
});

test("run with a compile error opens no terminal", async () => {
  await withExtension(BROKEN, {}, async ({ vscode, run }) => {
    await run("tp5.run");
    assert.equal(vscode.terminals.length, 0);
    assert.equal(vscode.messages.error.length, 1);
  });
});

test("run of a program that uses Crt opens a terminal and draws on the alternate screen", async () => {
  await withExtension(CRT, {}, async ({ vscode, run }) => {
    await run("tp5.run");
    assert.deepEqual(vscode.messages.error, []);
    assert.equal(vscode.terminals.length, 1);
    const written = [];
    const { pty } = vscode.terminals[0].options;
    pty.onDidWrite((text) => written.push(text));
    pty.open({ columns: 80, rows: 25 });
    await until("the end", () => !vscode.terminals[0].options.pty.running);
    pty.handleInput(" ");
    await until("the end", () => written.join("").includes("exit code 0"));
    assert.ok(written.join("").startsWith("\x1b[?1049h"));
  });
});

const WRITER = "program w;\nvar f: Text;\nbegin\n  Assign(f, 'OUT.TXT');\n  Rewrite(f);\n  WriteLn(f, 'x');\n  Close(f);\nend.\n";

async function runWriter(settings) {
  let onDisk;
  await withExtension(WRITER, settings, async ({ vscode, directory, run }) => {
    await run("tp5.run");
    const written = [];
    const { pty } = vscode.terminals[0].options;
    pty.onDidWrite((text) => written.push(text));
    pty.open(undefined);
    await until("the end", () => written.join("").includes("exit code 0"));
    onDisk = fs.existsSync(path.join(directory, "OUT.TXT"));
  });
  return onDisk;
}

test("the setting tp5.run.writeFiles decides whether the program's files reach the disk", async () => {
  assert.equal(await runWriter({ "run.writeFiles": true }), true);
  assert.equal(await runWriter({ "run.writeFiles": false }), false);
});

test("stop ends the running programs", async () => {
  const spinner = "program s;\nbegin\n  while True do\n    ;\nend.\n";
  await withExtension(spinner, {}, async ({ vscode, run }) => {
    await run("tp5.run");
    const written = [];
    const { pty } = vscode.terminals[0].options;
    pty.onDidWrite((text) => written.push(text));
    pty.open(undefined);
    await new Promise((resolve) => setTimeout(resolve, 200));
    assert.equal(pty.running, true);
    await run("tp5.stop");
    await until("stopped", () => /stopped/.test(written.join("")));
    assert.equal(pty.running, false);
  });
});

test("an include file is not compiled or run", async () => {
  await withExtension(HELLO, {}, async ({ vscode, run }) => {
    vscode.activeEditor = { document: fakeDocument("/x/defs.inc", "const a = 1;") };
    await run("tp5.run");
    assert.equal(vscode.terminals.length, 0);
  });
});
