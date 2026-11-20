// The pseudoterminal of a run: terminal input goes to the program, its output to the
// terminal, Ctrl+C and close stop it, and the terminal stays until a key after the end.
import test from "node:test";
import assert from "node:assert/strict";
import { createRunPty } from "../../vscode/src/run_terminal.mjs";
import { FakeEventEmitter } from "./fake_vscode.mjs";
import { program, askProgram, scratchDirectory, until } from "./programs.mjs";

function openPty(wasm) {
  const pty = createRunPty(FakeEventEmitter, {
    wasm,
    directory: scratchDirectory("pty-"),
    writeFiles: true,
  });
  const written = [];
  let closed = 0;
  pty.onDidWrite(({ length }, ...rest) => rest);
  pty.onDidWrite((text) => written.push(text));
  pty.onDidClose(() => closed++);
  return { pty, text: () => written.join(""), closed: () => closed };
}

test("opening the terminal starts the program and its output is written", async () => {
  const { pty, text } = openPty(program("hello"));
  pty.open(undefined);
  await until("the end", () => text().includes("exit code 0"));
  assert.ok(text().startsWith("Hello\r\nline two\r\n"));
});

test("nothing runs before open", async () => {
  const { pty, text } = openPty(program("hello"));
  await new Promise((resolve) => setTimeout(resolve, 150));
  assert.equal(text(), "");
  pty.close();
});

test("typed keys reach ReadLn; the program echoes them", async () => {
  const { pty, text } = openPty(askProgram());
  pty.open(undefined);
  await until("the prompt", () => text().includes("name? "));
  pty.handleInput("Zed\r");
  await until("the second prompt", () => text().includes("a b? "));
  pty.handleInput("1 2\r");
  await until("the end", () => text().includes("exit code 0"));
  assert.match(text(), /hello Zed 3\r\n/);
});

test("after the end the terminal says so, stays open, and closes on the next key", async () => {
  const { pty, text, closed } = openPty(program("hello"));
  pty.open(undefined);
  await until("the end", () => text().includes("exit code 0"));
  assert.match(text(), /Press any key to close the terminal/i);
  await new Promise((resolve) => setTimeout(resolve, 100));
  assert.equal(closed(), 0);
  pty.handleInput("x");
  assert.equal(closed(), 1);
});

test("Ctrl+C stops a program that waits for input, then a key closes the terminal", async () => {
  const { pty, text, closed } = openPty(askProgram());
  pty.open(undefined);
  await until("the prompt", () => text().includes("name? "));
  pty.handleInput("\x03");
  await until("stopped", () => /stopped/.test(text()));
  assert.match(text(), /Press any key/i);
  assert.equal(closed(), 0);
  pty.handleInput("\r");
  assert.equal(closed(), 1);
});

test("Ctrl+C stops a spinning program", async () => {
  const { pty, text } = openPty(program("spin"));
  pty.open(undefined);
  await until("the output", () => text().includes("spinning"));
  pty.handleInput("\x03");
  await until("stopped", () => /stopped/.test(text()));
});

test("Ctrl+C typed among other keys stops the program and the rest is dropped", async () => {
  const { pty, text } = openPty(askProgram());
  pty.open(undefined);
  await until("the prompt", () => text().includes("name? "));
  pty.handleInput("ab\x03cd");
  await until("stopped", () => /stopped/.test(text()));
});

test("closing the terminal terminates the program and writes nothing more", async () => {
  const { pty, text, closed } = openPty(program("spin"));
  pty.open(undefined);
  await until("the output", () => text().includes("spinning"));
  pty.close();
  const written = text();
  await new Promise((resolve) => setTimeout(resolve, 150));
  assert.equal(text(), written);
  assert.equal(closed(), 0, "the user closed it, there is nothing to fire");
});

test("stop() is what the Stop command calls: it ends the run like Ctrl+C", async () => {
  const { pty, text } = openPty(program("spin"));
  pty.open(undefined);
  await until("the output", () => text().includes("spinning"));
  pty.stop();
  await until("stopped", () => /stopped/.test(text()));
  assert.equal(pty.running, false);
});

test("running is true from open until the end", async () => {
  const { pty, text } = openPty(program("hello"));
  assert.equal(pty.running, false);
  pty.open(undefined);
  assert.equal(pty.running, true);
  await until("the end", () => text().includes("exit code 0"));
  assert.equal(pty.running, false);
});

test("input after the program ended and the terminal closed does nothing", async () => {
  const { pty, text, closed } = openPty(program("hello"));
  pty.open(undefined);
  await until("the end", () => text().includes("exit code 0"));
  pty.handleInput("x");
  pty.handleInput("y");
  assert.equal(closed(), 1, "closes once");
});
