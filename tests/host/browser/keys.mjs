// Exit test: key presses sent through Chrome's input pipeline reach ReadKey. The program is
// hand-built (loop: read a key, echo it) because the Crt unit is not needed for the transport.
// Extended keys arrive as two bytes, 0 then the scan code, like ReadKey returns them.
import assert from "node:assert/strict";
import { withPage, publishProgram, KEYS } from "./chrome.mjs";
import { loopEchoProgram } from "../browser_helpers.mjs";

await withPage(async (page) => {
  await page.open(publishProgram("loopecho", loopEchoProgram()));
  assert.equal(await page.status(), "running");
  assert.equal(await page.row(0), "", "nothing is echoed before a key is pressed");

  await page.key(KEYS.letter("a"));
  await page.until("echo of a", async () => (await page.row(0)) === "a");

  await page.key(KEYS.right); // ReadKey returns 0, then 77
  await page.until("echo of Right", async () => (await page.row(0)) === "a M");

  await page.key(KEYS.letter("b"));
  await page.until("echo of b", async () => (await page.row(0)) === "a Mb");

  // Negative: a key with no ReadKey byte (F12) reaches the page but not the program.
  await page.key(KEYS.f12);
  await page.key(KEYS.letter("c"));
  await page.until("echo of c", async () => (await page.row(0)) === "a Mbc");
});
console.log("keys: ok");
