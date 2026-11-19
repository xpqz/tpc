// Exit test: in the browser a program without Crt that does WriteLn then Write
// continues in column 1 of the next row, not under the end of the previous line. The program is
// tests/programs/console_newline.pas; tools/tp5.sh prints `first`, `second`, `third`.
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { withPage, publishProgram, REPO } from "./chrome.mjs";

const compiler = path.join(
  execFileSync("scripts/cargo-target.sh", { cwd: REPO, encoding: "utf8" }).trim(), "debug/tp5c");
const out = path.join(REPO, "tmp/browser/console_newline.wasm");
fs.mkdirSync(path.dirname(out), { recursive: true });
execFileSync(compiler, ["tests/programs/console_newline.pas", "-o", out], { cwd: REPO });

await withPage(async (page) => {
  await page.open(publishProgram("console_newline", fs.readFileSync(out)));
  await page.until("end of the program", async () => (await page.status()) !== "running");
  assert.notEqual(await page.row(0), "", "the program wrote something");
  assert.deepEqual([await page.row(0), await page.row(1), await page.row(2)],
    ["first", "second", "third"]);
});
console.log("newline: ok");
