// A Crt program in the pseudoterminal: the picture, the keys from the terminal, the window size.
import test from "node:test";
import assert from "node:assert/strict";
import { createRunPty } from "../../vscode/src/run_terminal.mjs";
import { FakeEventEmitter } from "./fake_vscode.mjs";
import { AnsiScreen } from "./ansi_screen.mjs";
import { program, scratchDirectory, until } from "./programs.mjs";

const opened = [];
test.afterEach(() => opened.splice(0).forEach((pty) => pty.stop()));

function openPty(wasm) {
  const pty = createRunPty(FakeEventEmitter, { wasm, directory: scratchDirectory("crt-pty-"), writeFiles: false });
  opened.push(pty);
  const written = [];
  let closed = 0;
  pty.onDidWrite((text) => written.push(text));
  pty.onDidClose(() => closed++);
  const text = () => written.join("");
  // What the user sees right now: replay everything written so far.
  const screen = () => {
    const terminal = new AnsiScreen();
    terminal.feed(text());
    return terminal;
  };
  return { pty, text, screen, closed: () => closed, written };
}

test("the program's picture appears on the alternate screen", async () => {
  const { pty, screen, text } = openPty(program("crt_key_loop"));
  pty.open({ columns: 80, rows: 25 });
  await until("the picture", () => screen().onAlternate && screen().rowText(0) === "waiting");
  assert.equal(screen().cursorVisible, true);
  pty.stop();
  await until("the end", () => text().includes("stopped"));
});

test("an arrow key typed in the terminal is ReadKey's 0 and 72", async () => {
  const { pty, screen, text } = openPty(program("crt_key_loop"));
  pty.open({ columns: 80, rows: 25 });
  await until("the picture", () => screen().rowText(0) === "waiting");
  pty.handleInput("\x1b[A");
  await until("the end", () => text().includes("exit code 0"));
  const lastFrame = new AnsiScreen();
  lastFrame.feed(text().split("\x1b[?1049l")[0]);
  assert.equal(lastFrame.rowText(1), "got 72");
});

test("a function key and an Alt chord reach the program as extended keys", async () => {
  for (const [input, code] of [["\x1bOP", 59], ["\x1bx", 45], ["a", 97]]) {
    const { pty, text } = openPty(program("crt_key_loop"));
    pty.open({ columns: 80, rows: 25 });
    await until("the first frame", () => text().includes("waiting"));
    pty.handleInput(input);
    await until("the end", () => text().includes("exit code 0"));
    assert.ok(text().includes(`got ${code}`), `${JSON.stringify(input)} gives ${code}`);
  }
});

test("Ctrl+C stops a Crt program, then the screen is the normal one with the end text", async () => {
  const { pty, text, screen, closed } = openPty(program("crt_spin"));
  pty.open({ columns: 80, rows: 25 });
  await until("a frame", () => text().includes("\x1b[?25l"));
  pty.handleInput("\x03");
  await until("the end", () => text().includes("stopped"));
  assert.equal(screen().onAlternate, false);
  assert.match(text(), /Press any key to close the terminal/);
  pty.handleInput("q");
  assert.equal(closed(), 1);
});

test("opening in a terminal smaller than 80x25 shows the notice and no picture", async () => {
  const { pty, text, screen } = openPty(program("crt_key_loop"));
  pty.open({ columns: 60, rows: 20 });
  await new Promise((resolve) => setTimeout(resolve, 150));
  assert.match(text(), /enlarge/i);
  assert.ok(!text().includes("waiting"), "no picture while the window is too small");
  assert.equal(screen().onAlternate, true);
  pty.stop();
});

test("growing the terminal redraws the whole picture, shrinking shows the notice again", async () => {
  const { pty, text, screen, written } = openPty(program("crt_key_loop"));
  pty.open({ columns: 60, rows: 20 });
  await new Promise((resolve) => setTimeout(resolve, 100));
  pty.setDimensions({ columns: 100, rows: 30 });
  await until("the picture", () => screen().rowText(0) === "waiting");
  const afterGrow = written.length;
  pty.setDimensions({ columns: 70, rows: 25 });
  await until("the second notice", () => written.length > afterGrow && /enlarge/i.test(written.at(-1)));
  pty.setDimensions({ columns: 80, rows: 25 });
  await until("the picture again", () => screen().rowText(0) === "waiting");
  assert.ok(text().split("\x1b[2J").length >= 3, "each redraw clears the screen first");
  pty.stop();
});

test("setDimensions to the same size and before open do no harm", async () => {
  const { pty, text } = openPty(program("crt_key_loop"));
  pty.setDimensions({ columns: 80, rows: 25 });
  pty.open(undefined);
  await until("the picture", () => text().includes("waiting"));
  const length = text().length;
  pty.setDimensions({ columns: 80, rows: 25 });
  await new Promise((resolve) => setTimeout(resolve, 120));
  assert.equal(text().length, length);
  pty.stop();
});

test("closing the terminal ends a Crt program without a word", async () => {
  const { pty, text } = openPty(program("crt_spin"));
  pty.open({ columns: 80, rows: 25 });
  await until("a frame", () => text().includes("\x1b[?25l"));
  pty.close();
  const length = text().length;
  await new Promise((resolve) => setTimeout(resolve, 150));
  assert.equal(text().length, length);
  assert.equal(pty.running, false);
});
