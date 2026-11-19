// Exit test: key latency under 50 ms. A program reads a key and echoes it, which moves the
// cursor on the shared screen. The test presses a key, and the page polls the shared screen
// (a MessageChannel loop, no timer clamping) for the cursor to move. Reported per key:
//   end to end: Node starts sending the key (before the CDP round trip) to the screen change;
//   in page:    the page's keydown event to the screen change (the host's own path).
// The page's polling and the program run on different threads, so this includes the key ring,
// the Worker's wake-up from Atomics.wait and the write; it excludes the repaint (next frame).
import assert from "node:assert/strict";
import { withPage, publishProgram, sleep, KEYS } from "./chrome.mjs";
import { loopEchoProgram } from "../browser_helpers.mjs";
import { CURSOR } from "../../../host/terminal.js";

const KEY_COUNT = 40;
const LIMIT_MS = 50;
const now = () => performance.timeOrigin + performance.now();

const percentile = (sorted, p) => sorted[Math.min(sorted.length - 1, Math.floor(p * sorted.length))];
const describe = (values) => {
  const sorted = [...values].sort((a, b) => a - b);
  const f = (n) => n.toFixed(1);
  return `min ${f(sorted[0])}  median ${f(percentile(sorted, 0.5))}  p95 ${f(percentile(sorted, 0.95))}  max ${f(sorted.at(-1))} ms`;
};

await withPage(async (page) => {
  await page.open(publishProgram("loopecho", loopEchoProgram()));
  await page.eval(`
    window.__down = 0; window.__seen = [];
    document.addEventListener("keydown", () => (window.__down = performance.timeOrigin + performance.now()), true);
    const channel = new MessageChannel();
    let cursor = window.__grid[${CURSOR}];
    channel.port1.onmessage = () => {
      const now = window.__grid[${CURSOR}];
      if (now !== cursor) { cursor = now; window.__seen.push([window.__down, performance.timeOrigin + performance.now()]); }
      channel.port2.postMessage(0);
    };
    channel.port2.postMessage(0); 0`);

  const endToEnd = [];
  const inPage = [];
  for (let i = 0; i < KEY_COUNT; i++) {
    const sent = now();
    await page.key(KEYS.letter("x"));
    const seen = await page.until("the screen to change", async () =>
      (await page.eval("window.__seen.length")) === i + 1 && (await page.eval("window.__seen[window.__seen.length - 1]")));
    endToEnd.push(seen[1] - sent);
    inPage.push(seen[1] - seen[0]);
    await sleep(60);
  }
  assert.equal(await page.row(0), "x".repeat(KEY_COUNT), "every key was echoed exactly once");
  console.log(`latency over ${KEY_COUNT} keys (target < ${LIMIT_MS} ms):`);
  console.log(`  end to end  ${describe(endToEnd)}`);
  console.log(`  in page     ${describe(inPage)}`);
  const worst = Math.max(...inPage);
  assert.ok(worst < LIMIT_MS, `slowest key took ${worst.toFixed(1)} ms in page`);
});
console.log("latency: ok");
