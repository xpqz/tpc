// The Pages site (`make site`, tmp/site/) served with no COOP/COEP headers, at the root and under
// a sub-path: the service worker must make the IDE page cross-origin isolated, and the IDE must
// then work: open the sample program, compile it, run it and show its screen.
// Usage: node tests/host/browser/site.mjs  (or: make test-site; builds the site first)
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { chromeBackend, chromeInstalled, REPO, CHROME, sleep } from "./chrome.mjs";
import { serveDirectory } from "./plain_server.mjs";

if (!chromeInstalled()) {
  console.error(`Chrome not found at ${CHROME}; set CHROME to its executable.`);
  process.exit(1);
}
const site = path.join(REPO, "tmp/site");
assert.ok(fs.existsSync(path.join(site, "index.html")), "run `make site` first");

const named = (key, vk, extra = {}) => ({ key, code: key, vk, ...extra });
const letter = (ch) => ({ key: ch, text: ch, code: `Key${ch.toUpperCase()}`, vk: ch.toUpperCase().charCodeAt(0) });
const F3 = named("F3", 114), F9 = named("F9", 120), ENTER = named("Enter", 13);
const CTRL = 2;

// Polls until `probe` returns something truthy (the deadline is only a hang guard).
async function until(what, probe, ms = 60000) {
  const start = Date.now();
  for (;;) {
    const value = await probe();
    if (value) return value;
    if (Date.now() - start > ms) throw new Error(`timed out waiting for ${what}`);
    await sleep(50);
  }
}

async function checkSite(prefix) {
  const server = await serveDirectory(site, prefix, 20000 + Math.floor(Math.random() * 20000));
  const browser = await chromeBackend(path.join(REPO, `tmp/site-profile${prefix.replaceAll("/", "-")}`));
  try {
    const evaluate = async (code) =>
      JSON.parse(await browser.run(`JSON.stringify((() => { ${code} })() ?? null)`));
    const row = (n) => evaluate(`return window.__row(${n})`);
    const press = async (...specs) => { for (const spec of specs) await browser.key(spec); };
    const screenHas = (what, n, test) => until(what, async () => {
      try { return test(await row(n)); } catch { return false; }
    });

    await browser.navigate(`http://localhost:${server.port}${prefix}`);
    await until("the IDE page", () =>
      evaluate(`return location.pathname.endsWith("/host/index.html") && !!window.__grid`).catch(() => false));
    assert.equal(await evaluate(`return location.pathname`), `${prefix}host/index.html`);
    assert.equal(await evaluate(`return crossOriginIsolated`), true, "the page is isolated");
    await screenHas("the IDE's menu bar", 0, (r) => r.startsWith(" File  Run  Compile"));

    // The sample program is in the page's files: open it, compile it, run it.
    await press(F3, ...[..."demo"].map(letter), ENTER);
    await screenHas("DEMO.PAS opened", 0, (r) => r.endsWith("DEMO.PAS"));
    await screenHas("the program's first line", 1, (r) => r.startsWith("program Colours;"));
    await press(F9);
    await screenHas("Compiled", 24, (r) => r.includes("Compiled"));
    await press({ ...F9, modifiers: CTRL });
    await screenHas("the program's output", 0, (r) => r === "Turbo Pascal 5.0, compiled to WebAssembly");
    assert.match(await row(2), /^ {3}1 {2}.{8} {2}colour$/);
    assert.equal(await evaluate(`return document.getElementById("status").textContent`), "running");
  } finally {
    browser.stop();
    server.stop();
  }
}

for (const prefix of ["/", "/tpc/"]) {
  await checkSite(prefix);
  console.log(`site at ${prefix}: ok`);
}
