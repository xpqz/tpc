// Probe (prints a table, asserts only that the page lived): which chords does a page receive
// when a browser driver injects them, and which bytes does today's host/keys.js make of them?
// Chrome gets them over CDP (Input.dispatchKeyEvent goes straight to the renderer), Firefox
// over WebDriver BiDi. Neither goes through the browser's own shortcut handling when headless
// and driven, so "received" here means "the page would see it if the browser let it through";
// docs/prs/67.md lists what real browsers keep for themselves.
// Usage: BROWSER=chrome|firefox node tests/host/browser/chords.mjs
import { withPage, publishProgram, sleep } from "./chrome.mjs";
import { loopEchoProgram } from "../browser_helpers.mjs";

const ALT = 1, CTRL = 2, SHIFT = 8;
const letter = (ch) => ({ key: ch, code: `Key${ch.toUpperCase()}`, vk: ch.toUpperCase().charCodeAt(0), w3c: ch });
const fkey = (n) => ({ key: `F${n}`, code: `F${n}`, vk: 111 + n, w3c: String.fromCharCode(0xe030 + n) });
const named = (key, vk, w3c) => ({ key, code: key, vk, w3c });
const TAB = named("Tab", 9, "");
const LEFT = named("ArrowLeft", 37, "");
const RIGHT = named("ArrowRight", 39, "");
const HOME = named("Home", 36, "");
const END = named("End", 35, "");
const PGUP = named("PageUp", 33, "");
const PGDN = named("PageDown", 34, "");
const digit = { key: "1", code: "Digit1", vk: 49, w3c: "1" };

const chords = [];
const add = (name, spec, modifiers) => chords.push({ name, spec, modifiers });
for (const ch of "abcdefghijklmnopqrstuvwxyz") add(`Ctrl+${ch.toUpperCase()}`, letter(ch), CTRL);
add("Alt+X", letter("x"), ALT);
add("Alt+A", letter("a"), ALT);
add("Alt+1", digit, ALT);
for (let n = 1; n <= 10; n++) {
  add(`F${n}`, fkey(n), 0);
  add(`Shift+F${n}`, fkey(n), SHIFT);
  add(`Ctrl+F${n}`, fkey(n), CTRL);
  add(`Alt+F${n}`, fkey(n), ALT);
}
add("Shift+Tab", TAB, SHIFT);
add("Ctrl+Tab", TAB, CTRL);
for (const [name, spec] of [["Left", LEFT], ["Right", RIGHT], ["Home", HOME], ["End", END],
  ["PgUp", PGUP], ["PgDn", PGDN]]) add(`Ctrl+${name}`, spec, CTRL);

const results = await withPage(async (page) => {
  await page.open(publishProgram("loopecho", loopEchoProgram()));
  await page.eval(`
    window.__events = [];
    addEventListener("keydown", (e) => __events.push({ key: e.key, code: e.code, ctrlKey: e.ctrlKey,
      altKey: e.altKey, shiftKey: e.shiftKey, metaKey: e.metaKey }), true);
    window.__ready = import("/host/keys.js").then((m) => (window.__keyBytes = m.keyBytes));
    true;
  `);
  await page.eval("__ready.then(() => true)");
  const tabsBefore = await page.contexts();
  const rows = [];
  for (const { name, spec, modifiers } of chords) {
    const before = await page.eval("__events.length");
    await page.key({ ...spec, modifiers });
    let event = null;
    for (let i = 0; i < 15 && event === null; i++) {
      await sleep(10);
      // Firefox also reports the modifier's own keydown first; the chord is the other one.
      event = await page.eval(`__events.slice(${before}).find((e) =>
        !["Control", "Alt", "Shift", "Meta"].includes(e.key)) ?? null`);
    }
    const bytes = event ? await page.eval(`__keyBytes(${JSON.stringify(event)})`) : null;
    rows.push({ name, received: event !== null, event, bytes });
  }
  const tabsAfter = await page.contexts();
  return { rows, tabsBefore, tabsAfter };
});

const show = (r) => `${r.name.padEnd(10)} ${r.received ? "received" : "NOT received"}` +
  `  key=${r.event?.key ?? "-"} code=${r.event?.code ?? "-"}  today's bytes: ${JSON.stringify(r.bytes)}`;
for (const r of results.rows) console.log(show(r));
console.log(`tabs/windows before ${results.tabsBefore}, after ${results.tabsAfter}`);
console.log(`${results.rows.filter((r) => r.received).length}/${results.rows.length} chords received`);
