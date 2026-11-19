// Exit test: an infinite-loop program is stopped with the Stop button and the page stays
// responsive. Responsiveness is a JS timer in the page that keeps firing: before the click
// (the program runs in its own Worker, so it must not starve the page) and after it.
import assert from "node:assert/strict";
import { withPage, publishProgram, sleep } from "./chrome.mjs";
import { spinProgram } from "../browser_helpers.mjs";

await withPage(async (page) => {
  await page.open(publishProgram("spin", spinProgram()));
  assert.equal(await page.status(), "running");
  await page.eval(`
    window.__ticks = []; setInterval(() => window.__ticks.push(performance.now()), 20); 0`);

  await sleep(500);
  const during = await page.eval("window.__ticks.length");
  assert.ok(during >= 15, `page timer starved while the program spins: ${during} ticks in 500 ms`);
  assert.equal(await page.status(), "running", "the loop must still be running");

  const clicked = performance.now();
  await page.click("stop");
  assert.equal(await page.status(), "stopped");
  const stopMs = performance.now() - clicked;
  const afterStop = await page.eval("window.__ticks.length");
  await sleep(500);
  const later = await page.eval("window.__ticks.length");
  assert.ok(later - afterStop >= 15, `timer stopped after Stop: ${later - afterStop} ticks`);

  const gaps = await page.eval(`
    JSON.stringify(window.__ticks.slice(1).map((t, i) => t - window.__ticks[i]))`);
  const worst = Math.max(...JSON.parse(gaps));
  assert.ok(worst < 250, `worst gap between timer ticks was ${worst.toFixed(0)} ms`);
  console.log(`stop: ${later} timer ticks, worst gap ${worst.toFixed(1)} ms, ` +
    `click Stop to "stopped" in ${stopMs.toFixed(0)} ms (incl. protocol)`);

  // The page can run a program again after a Stop.
  await page.click("run");
  assert.equal(await page.status(), "running");
  await page.click("stop");
  assert.equal(await page.status(), "stopped");
});
console.log("stop: ok");
