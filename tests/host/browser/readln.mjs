// Exit test (decision 46): a program without Crt reads typed lines with ReadLn in the browser.
// The keys go through the real input pipeline; the line editor of System echoes them on the
// teletype screen. The program is tests/programs/readln_browser.pas.
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { withPage, publishProgram, REPO, KEYS } from "./chrome.mjs";

const compiler = path.join(
  execFileSync("scripts/cargo-target.sh", { cwd: REPO, encoding: "utf8" }).trim(), "debug/tp5c");
const out = path.join(REPO, "tmp/browser/readln_browser.wasm");
fs.mkdirSync(path.dirname(out), { recursive: true });
execFileSync(compiler, ["tests/programs/readln_browser.pas", "-o", out], { cwd: REPO });

await withPage(async (page) => {
  const type = async (text) => {
    for (const ch of text) await page.key(ch === " " ? KEYS.space : KEYS.letter(ch));
  };
  await page.open(publishProgram("readln_browser", fs.readFileSync(out)));
  await page.until("the prompt", async () => (await page.row(0)) === "name?");
  assert.equal(await page.status(), "running", "the program waits for the line");

  await type("Bxo");
  await page.until("echo", async () => (await page.row(0)) === "name? Bxo");
  await page.key(KEYS.backspace);
  await page.until("backspace", async () => (await page.row(0)) === "name? Bx");
  await page.key(KEYS.backspace);
  await type("ob");
  await page.key(KEYS.enter);
  await page.until("second prompt", async () => (await page.row(0)) === "name? Bob" &&
    (await page.row(1)) === "a b?");

  await type("12 30");
  await page.key(KEYS.enter);
  await page.until("result", async () => (await page.row(2)) === "hello Bob 42");
  assert.equal(await page.row(1), "a b? 12 30");
  await page.until("end of the program", async () => (await page.status()) !== "running");
});
console.log("readln: ok");
