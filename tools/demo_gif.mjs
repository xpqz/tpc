// Records docs/images/ide-demo.gif: the IDE of the built site (tmp/site, `make site`) in
// headless Chrome, driven with real key events; each step waits for the IDE's screen, then
// screenshots the canvas. ImageMagick joins the frames with a shared palette.
// Usage: node tools/demo_gif.mjs  (or: make demo-gif)
import fs from "node:fs";
import path from "node:path";
import { spawnSync } from "node:child_process";
import { chromeBackend, REPO, sleep } from "../tests/host/browser/chrome.mjs";
import { serveDirectory } from "../tests/host/browser/plain_server.mjs";

const site = path.join(REPO, "tmp/site");
const frameDir = path.join(REPO, "tmp/demo-frames");
const output = path.join(REPO, "docs/images/ide-demo.gif");
const MAGICK = process.env.MAGICK ?? "magick";

const named = (key, vk, extra = {}) => ({ key, code: key, vk, ...extra });
const letter = (ch) => ({ key: ch, text: ch, code: `Key${ch.toUpperCase()}`, vk: ch.toUpperCase().charCodeAt(0) });
const word = (text) => [...text].map(letter);
const KEY = {
  down: named("ArrowDown", 40), up: named("ArrowUp", 38), end: named("End", 35),
  backspace: named("Backspace", 8), enter: named("Enter", 13), escape: named("Escape", 27),
  f3: named("F3", 114), f9: named("F9", 120),
  semicolon: { key: ";", text: ";", code: "Semicolon", vk: 186 },
};
const ctrlF9 = { ...KEY.f9, modifiers: 2 };
const altF = { ...letter("f"), text: undefined, modifiers: 1 };

fs.rmSync(frameDir, { recursive: true, force: true });
fs.mkdirSync(frameDir, { recursive: true });
fs.mkdirSync(path.dirname(output), { recursive: true });

const server = await serveDirectory(site, "/", 20000 + Math.floor(Math.random() * 20000));
const browser = await chromeBackend(path.join(REPO, "tmp/demo-profile"));
const frames = []; // { file, seconds }
try {
  const evaluate = async (code) => JSON.parse(await browser.run(`JSON.stringify((() => { ${code} })() ?? null)`));
  const row = (n) => evaluate(`return window.__row(${n})`);
  // Waits for the screen (a hang guard ends the wait; the step then fails the final check).
  async function waitFor(n, test) {
    for (let i = 0; i < 1500; i++) {
      try { if (test(await row(n))) return; } catch { /* page not ready */ }
      await sleep(20);
    }
    throw new Error(`screen row ${n} never matched ${test}`);
  }
  async function snap(seconds) {
    await sleep(120); // let the page repaint the grid
    const box = await evaluate(`const r = document.getElementById("screen").getBoundingClientRect();
      return { x: r.x, y: r.y, width: r.width, height: r.height }`);
    const file = path.join(frameDir, `${String(frames.length).padStart(3, "0")}.png`);
    fs.writeFileSync(file, await browser.screenshot(box));
    frames.push({ file, seconds });
  }
  const press = async (...specs) => { for (const spec of specs) await browser.key(spec); };
  const status = (test) => waitFor(24, test);

  await browser.navigate(`http://localhost:${server.port}/`);
  await waitFor(0, (r) => r.startsWith(" File  Run  Compile"));
  await snap(1);

  // Open the sample program.
  await press(KEY.f3, ...word("demo"));
  await waitFor(24, (r) => r.includes("demo"));
  await snap(1);
  await press(KEY.enter);
  await waitFor(0, (r) => r.endsWith("DEMO.PAS"));
  await snap(1.5);

  // The File menu.
  await press(altF);
  await waitFor(2, (r) => r.length > 0 && !r.startsWith("program"));
  await snap(1.5);
  await press(KEY.escape);
  await waitFor(2, (r) => r.startsWith("uses"));

  // Remove the ';' after ClrScr (line 7) and compile: the error is reported on line 8.
  await press(...Array(6).fill(KEY.down), KEY.end, KEY.backspace);
  await status((r) => r.startsWith(" 7:"));
  await snap(1);
  await press(KEY.f9);
  await status((r) => r.includes("Error 85"));
  await snap(2.5);

  // Put it back, compile, run.
  await press(KEY.up, KEY.end, KEY.semicolon, KEY.f9);
  await status((r) => r.includes("Compiled"));
  await snap(1.5);
  await press(ctrlF9);
  await waitFor(0, (r) => r.startsWith("Turbo Pascal 5.0"));
  await snap(3.5);

  // Back to the editor.
  await press(letter("x"));
  await status((r) => r.startsWith("Exit code 0 - press a key"));
  await snap(1);
  await press(letter("x"));
  await waitFor(0, (r) => r.endsWith("DEMO.PAS"));
  await snap(2);
} finally {
  browser.stop();
  server.stop();
}

// ImageMagick joins the frames: one delay (in 1/100 s) per frame, a shared 32-colour palette,
// and only the changed rectangle stored for each frame after the first.
const args = frames.flatMap((f) => ["-delay", String(Math.round(f.seconds * 100)), f.file]);
const result = spawnSync(MAGICK, [...args, "+dither", "-colors", "32", "-layers", "OptimizePlus",
  "-loop", "0", output], { encoding: "utf8" });
if (result.status !== 0) throw new Error(`magick failed: ${result.stderr}`);
console.log(`${output}: ${fs.statSync(output).size} bytes, ${frames.length} frames`);
