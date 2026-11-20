// A program that writes a lot of output: the messages from the worker are batched, the output
// is complete and in order, and Stop works in the middle.
import test from "node:test";
import assert from "node:assert/strict";
import { startRun } from "../../vscode/src/run_session.mjs";
import { program, scratchDirectory, until } from "./programs.mjs";

const started = [];
test.afterEach(() => started.splice(0).forEach((session) => session.stop()));

function run(wasm) {
  const chunks = [];
  const session = startRun({
    wasm, directory: scratchDirectory("flood-"), writeFiles: false, onOutput: (text) => chunks.push(text),
  });
  started.push(session);
  return { session, chunks };
}

test("a million lines arrive complete, in order, in a bounded number of messages", async () => {
  const { session, chunks } = run(program("flood_lines"));
  assert.deepEqual(await session.ended, { kind: "exit", code: 0 });
  assert.ok(chunks.length > 2, "more than one chunk: the bytes did not wait for the end");
  assert.ok(chunks.length < 2000, `${chunks.length} messages`);
  const lines = chunks.join("").split("\r\n");
  assert.equal(lines.length, 1_000_000 + 3, "the lines, an empty one, exit code 0 and the empty rest");
  for (let i = 0; i < 1_000_000; i++) {
    if (lines[i] !== `line ${i + 1}`) assert.fail(`line ${i + 1} is ${JSON.stringify(lines[i])}`);
  }
  assert.deepEqual(lines.slice(-3), ["", "exit code 0", ""]);
});

test("Stop works in the middle of a flood: it ends, and nothing arrives afterwards", async () => {
  const { session, chunks } = run(program("flood_forever"));
  await until("output", () => chunks.length > 3);
  session.stop();
  assert.deepEqual(await session.ended, { kind: "stopped" });
  const count = chunks.length;
  assert.ok(chunks.at(-1).endsWith("\r\nstopped\r\n"));
  await new Promise((resolve) => setTimeout(resolve, 200));
  assert.equal(chunks.length, count);
});

test("output written before ReadLn is delivered while the program waits", async () => {
  const { session, chunks } = run(program("prompt_then_readln"));
  await until("the prompt", () => chunks.join("").includes("name? "));
  assert.equal(session.running, true);
  session.sendKeys([13]);
  await session.ended;
  session.stop();
});

test("output before the end of a short program is not lost to batching", async () => {
  const { session, chunks } = run(program("hello"));
  await session.ended;
  assert.equal(chunks.join(""), "Hello\r\nline two\r\n\r\nexit code 0\r\n");
});
