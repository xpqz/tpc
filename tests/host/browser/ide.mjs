// Exit test of the browser IDE host: the toy IDE (tests/lang/ide_host_toy_open_save_compile_run)
// runs in the page's IDE Worker with the real compiler.wasm; Ctrl-F9 compiles the typed file
// name's program from the page's persistent files, hands the module to a program Worker, and
// blocks the IDE until the program ends or Stop is clicked. Checks compile -> run -> Stop, the
// cleared screen, the shared keyboard, and persistence in localStorage across a reload.
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { execFileSync } from "node:child_process";
import { withPage, publishProgram, KEYS, REPO } from "./chrome.mjs";
import { Compiler } from "../../../host/compiler.js";

const target = execFileSync("sh", ["scripts/cargo-target.sh"], { cwd: REPO, encoding: "utf8" }).trim();
execFileSync("cargo", ["build", "--release", "--lib", "--target", "wasm32-unknown-unknown"],
  { cwd: REPO, env: { ...process.env, CARGO_TARGET_DIR: target }, stdio: "ignore" });
const compilerBytes = fs.readFileSync(`${target}/wasm32-unknown-unknown/release/tp5c.wasm`);
const toy = new Compiler((await WebAssembly.instantiate(compilerBytes, {})).instance).compile(
  "TOY.PAS", fs.readFileSync(path.join(REPO, "tests/lang/ide_host_toy_open_save_compile_run.pas")),
  new Map());
assert.ok(toy.module, JSON.stringify(toy));

const guest = (name) => fs.readFileSync(path.join(REPO, "tests/ide/guests", name), "latin1");
const letters = (text) => [...text].map((c) => (c === "." ? KEYS.dot : KEYS.letter(c)));

await withPage(async (page) => {
  const ide = publishProgram("ide_toy", toy.module);
  const compiler = publishProgram("compiler", compilerBytes);
  const query = `&compiler=${compiler}`;
  const press = async (...specs) => {
    for (const spec of specs) await page.key(spec);
  };
  const open = (name) => press(KEYS.f3, ...letters(name), KEYS.enter);
  const row = (n) => page.row(n);
  const screenHas = (what, n, text) => page.until(what, async () => (await row(n)).startsWith(text));

  // Seed the page's persistent files, then start again so the IDE reads them at page load.
  await page.open(ide, query);
  await page.eval(`
    localStorage.setItem("tp5:LOOP.PAS", ${JSON.stringify(guest("LOOP.PAS"))});
    localStorage.setItem("tp5:SHOUT.PAS", ${JSON.stringify(guest("SHOUT.PAS"))});
    localStorage.setItem("tp5:BAD.PAS", "begin\\nERROR\\nend.\\n"); 0`);
  await page.open(ide, query);
  assert.equal(await page.status(), "running");
  await screenHas("the IDE's header", 0, "File: P.PAS");

  // Open, compile error: the IDE Worker compiled in the browser with the page's files.
  await open("bad.pas");
  await press(KEYS.f9);
  await screenHas("the compile error", 2, "Error 3 in BAD.PAS 2:1");

  // Compile and run a program that never ends: the screen is cleared, the IDE is blocked.
  await open("loop.pas");
  await press({ ...KEYS.f9, modifiers: 2 });
  await page.until("the cleared screen", async () => (await row(0)) === "");
  assert.equal(await row(2), "", "the IDE's status line is gone");
  assert.equal(await page.status(), "running");
  await page.eval("new Promise((resolve) => setTimeout(resolve, 300))");
  assert.equal(await row(0), "", "the blocked IDE does not repaint");

  // Stop ends the program, not the IDE: the run returns -1 and the IDE shows it.
  await page.click("stop");
  await screenHas("the stopped run", 24, "Exit code -1 - press a key");
  assert.equal(await page.status(), "running", "the IDE Worker is still alive");
  await press(KEYS.letter("x"));
  await screenHas("the repaint after the run", 0, "File: LOOP.PAS");

  // A program that ends by itself: its picture stays until a key, exit code 0.
  await open("shout.pas");
  await press({ ...KEYS.f9, modifiers: 2 });
  await screenHas("the program's picture", 0, "guest finished");
  await screenHas("the exit code", 24, "Exit code 0 - press a key");

  // Save writes localStorage; a reload sees the file and the IDE can open it again.
  await press(KEYS.letter("x"));
  await screenHas("the repaint", 0, "File: SHOUT.PAS");
  await press(KEYS.letter("z"), KEYS.f2);
  await screenHas("the save", 2, "Saved SHOUT.PAS");
  const saved = `${guest("SHOUT.PAS")}z`;
  await page.until("the file in localStorage", async () =>
    (await page.eval(`localStorage.getItem("tp5:SHOUT.PAS")`)) === saved);
  await page.open(ide, query);
  await open("shout.pas");
  await screenHas("the reopened file", 2, "Opened SHOUT.PAS");

  // Stop with no program running ends the IDE.
  await page.click("stop");
  assert.equal(await page.status(), "stopped");
});
console.log("ide: ok");
