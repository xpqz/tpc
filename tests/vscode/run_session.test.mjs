// The run session: a program in a worker thread, its output as terminal text, its keys, its
// files. Programs are compiled with the native tp5c.
import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { startRun, usesCrt } from "../../vscode/src/run_session.mjs";
import { program, askProgram, scratchDirectory, until } from "./programs.mjs";

function run(wasm, options = {}) {
  const output = [];
  const session = startRun({
    wasm,
    directory: options.directory ?? scratchDirectory("session-"),
    writeFiles: options.writeFiles ?? true,
    onOutput: (text) => output.push(text),
  });
  return { session, text: () => output.join("") };
}

const withDirectory = async (body) => {
  const directory = scratchDirectory("session-");
  try {
    await body(directory);
  } finally {
    fs.rmSync(directory, { recursive: true, force: true });
  }
};

const typed = (text) => [...text].map((c) => c.charCodeAt(0));

test("hello: output has CR LF line ends and the end is reported as exit code 0", async () => {
  const { session, text } = run(program("hello"));
  assert.deepEqual(await session.ended, { kind: "exit", code: 0 });
  assert.equal(text(), "Hello\r\nline two\r\n\r\nexit code 0\r\n");
});

test("Halt(n) is exit code n, after the output that came before", async () => {
  const { session, text } = run(program("exit_code"));
  assert.deepEqual(await session.ended, { kind: "exit", code: 3 });
  assert.equal(text(), "before\r\n\r\nexit code 3\r\n");
});

test("a runtime error shows the runtime's line, then the exit code", async () => {
  const { session, text } = run(program("divide_by_zero"));
  assert.deepEqual(await session.ended, { kind: "exit", code: 200 });
  assert.equal(text(), "before\r\nRuntime error 200\r\n\r\nexit code 200\r\n");
});

test("bytes above 127 reach the terminal as CP437 characters", async () => {
  const { session, text } = run(program("cp437_text"));
  await session.ended;
  assert.ok(text().startsWith("ü░┌\r\n"), JSON.stringify(text()));
});

test("ReadLn edits the line from the keys sent: echo, Backspace, Enter", async () => {
  const { session, text } = run(askProgram());
  await until("the first prompt", () => text().includes("name? "));
  session.sendKeys(typed("Bobx"));
  session.sendKeys([8]); // Backspace
  session.sendKeys([13]);
  await until("the second prompt", () => text().includes("a b? "));
  session.sendKeys(typed("12 30"));
  session.sendKeys([13]);
  assert.deepEqual(await session.ended, { kind: "exit", code: 0 });
  assert.match(text(), /hello Bob 42\r\n/);
  assert.match(text(), /Bob/, "typed characters are echoed by the program");
});

test("a program waiting for a line prints nothing more and does not end", async () => {
  const { session, text } = run(askProgram());
  await until("the prompt", () => text() === "name? ");
  await new Promise((resolve) => setTimeout(resolve, 150));
  assert.equal(text(), "name? ");
  session.stop();
  await session.ended;
});

test("stop ends a program blocked in ReadLn", async () => {
  const { session, text } = run(askProgram());
  await until("the prompt", () => text().includes("name? "));
  session.stop();
  assert.deepEqual(await session.ended, { kind: "stopped" });
  assert.match(text(), /stopped/);
});

test("a spinning program does not block the calling thread, and stop ends it", async () => {
  const { session, text } = run(program("spin"));
  await until("the output", () => text().includes("spinning"));
  let ticks = 0;
  const timer = setInterval(() => ticks++, 10);
  await new Promise((resolve) => setTimeout(resolve, 150));
  clearInterval(timer);
  assert.ok(ticks >= 3, `only ${ticks} ticks while the program spun`);
  session.stop();
  const outcome = await Promise.race([
    session.ended,
    new Promise((resolve) => setTimeout(() => resolve("hung"), 3000)),
  ]);
  assert.deepEqual(outcome, { kind: "stopped" });
});

test("stop twice, and after the end, does nothing", async () => {
  const { session, text } = run(program("hello"));
  await session.ended;
  const before = text();
  session.stop();
  session.stop();
  assert.equal(text(), before);
  assert.equal((await session.ended).kind, "exit");
});

test("output after stop is not reported", async () => {
  const { session, text } = run(program("spin"));
  await until("the output", () => text().includes("spinning"));
  session.stop();
  await session.ended;
  const final = text();
  await new Promise((resolve) => setTimeout(resolve, 100));
  assert.equal(text(), final);
});

test("a module that does not instantiate ends as an error with a message", async () => {
  const { session, text } = run(new Uint8Array([1, 2, 3]));
  const outcome = await session.ended;
  assert.equal(outcome.kind, "crashed");
  assert.ok(outcome.message.length > 0);
  assert.match(text(), /error/i);
});

test("the files of the directory are the program's: it reads one, and its changes reach the disk", async () => {
  await withDirectory(async (directory) => {
    const write = (name, content) => fs.writeFileSync(path.join(directory, name), content);
    write("EXISTING.TXT", "first line\r\nsecond\r\n");
    write("OLD.TXT", "old");
    write("FROM.TXT", "from");
    const { session, text } = run(program("file_changes"), { directory });
    assert.deepEqual(await session.ended, { kind: "exit", code: 0 });
    assert.match(text(), /^read first line\r\ndone\r\n/);
    const read = (name) => fs.readFileSync(path.join(directory, name), "latin1");
    assert.equal(read("CREATED.TXT"), "made by the program\r\n");
    assert.equal(fs.existsSync(path.join(directory, "OLD.TXT")), false, "Erase deletes");
    assert.equal(fs.existsSync(path.join(directory, "FROM.TXT")), false, "Rename moves");
    assert.equal(read("TO.TXT"), "from");
    assert.equal(read("EXISTING.TXT"), "first line\r\nsecond\r\n");
  });
});

test("with writeFiles off the program sees the files but the disk is not touched", async () => {
  await withDirectory(async (directory) => {
    const write = (name, content) => fs.writeFileSync(path.join(directory, name), content);
    write("EXISTING.TXT", "first line\r\n");
    write("OLD.TXT", "old");
    write("FROM.TXT", "from");
    const { session, text } = run(program("file_changes"), { directory, writeFiles: false });
    assert.deepEqual(await session.ended, { kind: "exit", code: 0 });
    assert.match(text(), /^read first line\r\ndone\r\n/);
    assert.deepEqual(fs.readdirSync(directory).sort(), ["EXISTING.TXT", "FROM.TXT", "OLD.TXT"]);
    assert.equal(fs.readFileSync(path.join(directory, "OLD.TXT"), "latin1"), "old");
  });
});

test("only 8.3 regular files are in the snapshot: a long name is not visible or changed", async () => {
  await withDirectory(async (directory) => {
    fs.writeFileSync(path.join(directory, "EXISTING.TXT"), "x\r\n");
    fs.writeFileSync(path.join(directory, "OLD.TXT"), "old");
    fs.writeFileSync(path.join(directory, "FROM.TXT"), "from");
    fs.writeFileSync(path.join(directory, "a long name.txt"), "keep");
    fs.mkdirSync(path.join(directory, "SUB.DIR"));
    const { session } = run(program("file_changes"), { directory });
    await session.ended;
    assert.equal(fs.readFileSync(path.join(directory, "a long name.txt"), "latin1"), "keep");
    assert.ok(fs.statSync(path.join(directory, "SUB.DIR")).isDirectory());
  });
});

test("existing files keep their spelling on disk, new files and renames are upper case", async () => {
  await withDirectory(async (directory) => {
    fs.writeFileSync(path.join(directory, "existing.txt"), "x\r\n");
    fs.writeFileSync(path.join(directory, "old.txt"), "old");
    fs.writeFileSync(path.join(directory, "from.txt"), "from");
    const { session } = run(program("file_changes"), { directory });
    assert.deepEqual(await session.ended, { kind: "exit", code: 0 });
    assert.equal(fs.readFileSync(path.join(directory, "CREATED.TXT"), "latin1"), "made by the program\r\n");
    assert.deepEqual(fs.readdirSync(directory).sort(), ["CREATED.TXT", "TO.TXT", "existing.txt"],
      "erase and rename act on the files under the names they have on disk");
  });
});

test("a symbolic link is never written through: the link target outside the directory stays", async () => {
  await withDirectory(async (directory) => {
    const outside = scratchDirectory("outside-");
    try {
      const target = path.join(outside, "secret.txt");
      fs.writeFileSync(target, "secret");
      fs.symlinkSync(target, path.join(directory, "CREATED.TXT"));
      fs.writeFileSync(path.join(directory, "EXISTING.TXT"), "x\r\n");
      fs.writeFileSync(path.join(directory, "OLD.TXT"), "old");
      fs.writeFileSync(path.join(directory, "FROM.TXT"), "from");
      const { session } = run(program("file_changes"), { directory });
      await session.ended;
      assert.equal(fs.readFileSync(target, "utf8"), "secret");
    } finally {
      fs.rmSync(outside, { recursive: true, force: true });
    }
  });
});

test("a missing directory is an empty file system, not a crash", async () => {
  const { session } = run(program("hello"), { directory: path.join(scratchDirectory("gone-"), "nope") });
  assert.deepEqual(await session.ended, { kind: "exit", code: 0 });
});

test("usesCrt: a program that imports screen is detected, a plain one is not", () => {
  assert.equal(usesCrt(program("hello")), false);
  assert.equal(usesCrt(askProgram()), false);
  assert.equal(usesCrt(program("crt_program")), true);
});
