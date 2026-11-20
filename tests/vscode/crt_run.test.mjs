// Crt programs run in the worker on a shared grid. The final grid is compared with the screen
// dump the real Turbo Pascal 5.0 wrote for the same program and keys (tests/lang/crt_*.out,
// produced by tools/tp5.sh), and the text sent to the terminal is replayed in an emulator.
import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { startRun } from "../../vscode/src/run_session.mjs";
const ENTER_SCREEN = "\x1b[?1049h"; // the alternate screen
import { parseKeys } from "../keys.mjs";
import { dump } from "../screen_dump.mjs";
import { root } from "./build.mjs";
import { AnsiScreen } from "./ansi_screen.mjs";
import { compileProgram, program, scratchDirectory, until } from "./programs.mjs";

// Every run is stopped when its test ends, so a failing test leaves no thread behind.
const started = [];
test.afterEach(() => started.splice(0).forEach((session) => session.stop?.()));

function run(wasm, { directory = scratchDirectory("crt-") } = {}) {
  const chunks = [];
  const session = startRun({ wasm, directory, writeFiles: false, onOutput: (t) => chunks.push([Date.now(), t]) });
  started.push(session);
  return { session, chunks, text: () => chunks.map(([, t]) => t).join("") };
}

// The ReadKey bytes of a key script (tests/keys.mjs): an extended key is 0 and its scan code.
const keyBytes = (script) =>
  parseKeys(script).flatMap((word) => (word & 0xff ? [word & 0xff] : [0, word >> 8]));

// Sends the keys as the program takes them; the queue holds 64 bytes.
async function feed(session, bytes) {
  for (let at = 0; at < bytes.length; ) {
    const next = bytes[at] === 0 ? 2 : 1;
    if (session.sendKeys(bytes.slice(at, at + next))) at += next;
    else await new Promise((resolve) => setTimeout(resolve, 1));
  }
}

const langDirectory = path.join(root, "tests/lang");
// Programs that need the IDE's compiler or shared directories (.compiler, .dirs) are left to
// the test runner of tests/lang; a fixture directory beside the program holds its units.
const needsRunnerSupport = (name) =>
  ["compiler", "dirs"].some((extension) => fs.existsSync(path.join(langDirectory, `${name}.${extension}`)));
const oracleCases = fs
  .readdirSync(langDirectory)
  .filter((name) => /^crt_.*\.pas$/.test(name))
  .map((name) => name.slice(0, -4))
  .filter((name) => !needsRunnerSupport(name));

// The program compiled in a scratch directory with its fixture files beside it.
function compileFixture(name) {
  const directory = scratchDirectory("crt-fixture-");
  fs.copyFileSync(path.join(langDirectory, `${name}.pas`), path.join(directory, `${name}.pas`));
  const fixtures = path.join(langDirectory, name);
  if (fs.existsSync(fixtures)) fs.cpSync(fixtures, directory, { recursive: true });
  return compileProgram(path.join(directory, `${name}.pas`));
}

test("there are oracle programs to compare with", () => {
  assert.ok(oracleCases.length >= 45, `found ${oracleCases.length}`);
});

for (const name of oracleCases) {
  test(`${name}: the grid and the terminal text equal the real TP 5.0 screen dump`, async () => {
    const wasm = compileFixture(name);
    const keysFile = path.join(langDirectory, `${name}.keys`);
    const keys = fs.existsSync(keysFile) ? keyBytes(fs.readFileSync(keysFile, "utf8")) : [];
    const expected = fs.readFileSync(path.join(langDirectory, `${name}.out`), "latin1");
    const { session, text } = run(wasm);
    assert.ok(session.grid instanceof Uint8Array, "a Crt program has a grid");
    await feed(session, keys);
    const outcome = await session.ended;
    assert.equal(outcome.kind, "exit");
    assert.equal(dump(session.grid, outcome.code), expected);

    // The text the terminal received draws the same screen: the last frame left it on the
    // alternate screen, which the closing text then leaves.
    const terminal = new AnsiScreen();
    const beforeLeaving = text().split("\x1b[?1049l")[0];
    terminal.feed(beforeLeaving);
    assert.equal(dump(terminal.grid(), outcome.code), expected, "the replayed terminal text");
  });
}

test("a Crt program's output starts with the alternate screen and ends on the normal one", async () => {
  const { session, text } = run(program("crt_program"));
  assert.ok(text().startsWith(ENTER_SCREEN));
  await session.ended;
  assert.equal(session.screenHeld, true);
  session.showEnd();
  assert.equal(session.screenHeld, false);
  const all = text();
  assert.ok(all.indexOf("\x1b[?1049l") > all.indexOf("Hello"));
  assert.ok(all.endsWith("\r\nexit code 0\r\n"), JSON.stringify(all.slice(-40)));
  const terminal = new AnsiScreen();
  terminal.feed(all);
  assert.equal(terminal.onAlternate, false);
  assert.equal(terminal.cursorVisible, true);
  assert.equal(terminal.rowText(0), "", "the normal screen has none of the program's drawing");
  assert.ok(!terminal.normal.slice(0, 160).includes(72), "no 'H' of Hello on the normal screen");
});

test("a program that does not use Crt has no grid and no alternate screen", async () => {
  const { session, text } = run(program("hello"));
  assert.equal(session.grid, undefined);
  await session.ended;
  assert.ok(!text().includes("\x1b["));
});

test("KeyPressed and ReadKey: an arrow key typed after the program started ends its loop", async () => {
  const { session, text } = run(program("crt_key_loop"));
  await until("the first frame", () => text().includes("waiting"));
  await new Promise((resolve) => setTimeout(resolve, 100));
  assert.equal(session.running, true, "still waiting for a key");
  session.sendKeys([0, 72]);
  const outcome = await session.ended;
  assert.deepEqual(outcome, { kind: "exit", code: 0 });
  const terminal = new AnsiScreen();
  terminal.feed(text().split("\x1b[?1049l")[0]);
  assert.equal(terminal.rowText(0), "waiting");
  assert.equal(terminal.rowText(1), "got 72");
});

test("frames are sent only when the grid changed", async () => {
  const { session, chunks } = run(program("crt_idle"));
  await until("the picture", () => chunks.some(([, t]) => t.includes("Hi")));
  const settled = chunks.length;
  await new Promise((resolve) => setTimeout(resolve, 300));
  assert.equal(chunks.length, settled, "nothing is written while the screen is idle");
  await session.ended;
  assert.ok(chunks.some(([, t]) => t.includes("late")), "the later change is drawn");
});

test("frames come at most about 30 times a second, and do come", async () => {
  const { session, chunks } = run(program("crt_spin"));
  await until("the first frame", () => chunks.length > 1);
  const start = Date.now();
  const before = chunks.length;
  await new Promise((resolve) => setTimeout(resolve, 600));
  const frames = chunks.length - before;
  const seconds = (Date.now() - start) / 1000;
  session.stop();
  assert.ok(frames >= 6, `${frames} frames in ${seconds}s`);
  assert.ok(frames <= seconds * 35 + 2, `${frames} frames in ${seconds}s`);
});

test("Delay waits on the real clock; Sound and NoSound do nothing", async () => {
  const started = Date.now();
  const { session, text } = run(program("crt_delay_sound"));
  const outcome = await session.ended;
  assert.deepEqual(outcome, { kind: "exit", code: 0 });
  assert.ok(Date.now() - started >= 290, "Delay(300) took its time");
  const terminal = new AnsiScreen();
  terminal.feed(text().split("\x1b[?1049l")[0]);
  assert.equal(terminal.rowText(0), "done");
});

test("stop ends a Crt program that spins and leaves the alternate screen", async () => {
  const { session, text } = run(program("crt_spin"));
  await until("a frame", () => text().includes("\x1b[?25l"));
  session.stop();
  assert.deepEqual(await session.ended, { kind: "stopped" });
  session.showEnd();
  const all = text();
  assert.ok(all.endsWith("\r\nstopped\r\n"));
  const terminal = new AnsiScreen();
  terminal.feed(all);
  assert.equal(terminal.onAlternate, false);
  assert.equal(terminal.cursorVisible, true);
});

test("the final picture stays on the alternate screen until showEnd", async () => {
  const { session, text } = run(program("crt_idle"));
  const outcome = await session.ended;
  assert.deepEqual(outcome, { kind: "exit", code: 0 });
  const held = new AnsiScreen();
  held.feed(text());
  assert.equal(held.onAlternate, true, "still on the alternate screen after the program ended");
  assert.equal(held.rowText(0), "Hi");
  assert.equal(held.rowText(1), "late", "the last frame was drawn");
  assert.ok(!text().includes("exit code"), "no end text yet");
  session.showEnd();
  session.showEnd();
  const left = new AnsiScreen();
  left.feed(text());
  assert.equal(left.onAlternate, false);
  assert.equal(text().split("exit code 0").length, 2, "the end text is written once");
  assert.ok(text().endsWith("\r\nexit code 0\r\n"));
});
