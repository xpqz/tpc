// The exit test of plan section 5, phase 7, in the browser: inside the IDE (ide/ide.pas, built
// here by compiler.wasm) open Snake, edit a line, compile, run, introduce a syntax error, see
// the cursor jump to it, fix it, run again. Snake is preloaded into the page's persistent
// files (localStorage) as the plan says, and the IDE compiles it with the real compiler.wasm
// in its Worker. Keys go through the browser's input pipeline, so Alt-X and Ctrl-F9 arrive as
// real chords. Usage: BROWSER=chrome|firefox node tests/host/browser/idemain.mjs
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { execFileSync } from "node:child_process";
import { withPage, publishProgram, REPO } from "./chrome.mjs";
import { Compiler } from "../../../host/compiler.js";

const target = execFileSync("sh", ["scripts/cargo-target.sh"], { cwd: REPO, encoding: "utf8" }).trim();
execFileSync("cargo", ["build", "--release", "--lib", "--target", "wasm32-unknown-unknown"],
  { cwd: REPO, env: { ...process.env, CARGO_TARGET_DIR: target }, stdio: "ignore" });
const compilerBytes = fs.readFileSync(`${target}/wasm32-unknown-unknown/release/tp5c.wasm`);
const instantiate = async () => new Compiler((await WebAssembly.instantiate(compilerBytes, {})).instance);

// The IDE program, compiled from the units of ide/.
const sources = new Map(fs.readdirSync(path.join(REPO, "ide"))
  .map((name) => [name, fs.readFileSync(path.join(REPO, "ide", name))]));
assert.ok(sources.has("ide.pas"), "ide/ide.pas, the IDE program, does not exist");
const ide = (await instantiate()).compile("IDE.PAS", sources.get("ide.pas"), sources);
assert.ok(ide.module, `the IDE does not compile: ${JSON.stringify(ide)}`);

const snake = fs.readFileSync(path.join(REPO, "tests/programs/snake.pas"), "latin1");

// Key specs for CDP (`vk`, `code`) and WebDriver (`w3c`).
const ALT = 1, CTRL = 2;
const named = (key, vk, w3c, extra = {}) => ({ key, code: key, vk, w3c, ...extra });
const letter = (ch) => ({ key: ch, text: ch, code: `Key${ch.toUpperCase()}`,
  vk: ch.toUpperCase().charCodeAt(0), w3c: ch });
const KEY = {
  down: named("ArrowDown", 40, ""), up: named("ArrowUp", 38, ""),
  right: named("ArrowRight", 39, ""), home: named("Home", 36, ""),
  end: named("End", 35, ""), del: named("Delete", 46, ""),
  backspace: named("Backspace", 8, ""), enter: named("Enter", 13, ""),
  f3: named("F3", 114, ""), f9: named("F9", 120, ""),
  semicolon: { key: ";", text: ";", code: "Semicolon", vk: 186, w3c: ";" },
};
const ctrlF9 = { ...KEY.f9, modifiers: CTRL };
const altX = { ...letter("x"), text: undefined, modifiers: ALT };
const word = (text) => [...text].map(letter);

await withPage(async (page) => {
  const ideUrl = publishProgram("ide_main", ide.module);
  const query = `&compiler=${publishProgram("compiler", compilerBytes)}`;
  const press = async (...specs) => {
    for (const spec of specs) await page.key(spec);
  };
  const repeat = (count, spec) => Array(count).fill(spec);
  const row = (n) => page.row(n);
  const screenHas = (what, n, test) => page.until(what, async () => test(await row(n)));
  const status = (what, test) => screenHas(what, 24, test);
  const startsWith = (text) => (row) => row.startsWith(text);
  const includes = (text) => (row) => row.includes(text);

  // Snake in the persistent files; the IDE reads them when the page loads.
  await page.open(ideUrl, query);
  await page.eval(`localStorage.setItem("tp5:SNAKE.PAS", ${JSON.stringify(snake)}); 0`);
  await page.open(ideUrl, query);
  assert.equal(await page.status(), "running");
  await screenHas("the menu bar and the file name", 0, includes("NONAME.PAS"));
  assert.ok((await row(0)).startsWith(" File  Run  Compile  Options"));

  // Open Snake: File name on the status prompt, then the text.
  await press(KEY.f3, ...word("snake"), KEY.enter);
  await screenHas("Snake's title", 0, (r) => r.endsWith("SNAKE.PAS"));
  await screenHas("Snake's first line", 1, startsWith("{ Snake for the browser terminal."));

  // Edit line 42, `  Write('Score: ', Score, '   ');`: Score becomes Points.
  await press(...repeat(41, KEY.down), KEY.home, ...repeat(9, KEY.right), ...repeat(5, KEY.del),
    ...word("Points"));
  await status("the cursor on line 42", startsWith(" 42:16"));
  await page.until("the edited line", async () => {
    for (let n = 1; n < 24; n++) if ((await row(n)).startsWith("  Write('Points: ', Score")) return true;
    return false;
  });

  // Compile, run: Snake's first row shows the edit; q ends the game; the prompt waits for a key.
  await press(KEY.f9);
  await status("Compiled", includes("Compiled"));
  await press(ctrlF9);
  await screenHas("the program's score line", 0, startsWith("Points: 0"));
  assert.equal(await row(24), "", "the IDE's status line is gone while the program runs");
  await press(letter("q"));
  await status("the end of the run", startsWith("Exit code 0 - press a key"));
  await press(letter("x"));
  await screenHas("the IDE back after the run", 0, (r) => r.endsWith("SNAKE.PAS"));
  await status("the status line after the run", (r) => r.startsWith(" 42:16") && !r.includes("Exit"));

  // Break line 40, `  GotoXY(1, 1);`, by deleting its `;`: the error is reported at line 41.
  await press(KEY.up, KEY.up, KEY.end, KEY.backspace, KEY.f9);
  await status("the error message", includes('Error 85: ";" expected'));
  await status("the cursor on the next line", startsWith(" 41:3 "));

  // Fix it: back to line 40, put the `;` back, run again.
  await press(KEY.up, KEY.end, KEY.semicolon, KEY.f9);
  await status("Compiled again", includes("Compiled"));
  await press(ctrlF9);
  await screenHas("the second run", 0, startsWith("Points: 0"));
  await press(letter("q"));
  await status("the end of the second run", startsWith("Exit code 0 - press a key"));
  await press(letter("x"));
  await screenHas("the IDE back again", 0, (r) => r.endsWith("SNAKE.PAS"));

  // Quit: the edits are unsaved, so the IDE asks; Y saves them to localStorage and the IDE ends.
  await press(altX);
  await status("the question", includes("Save changes to SNAKE.PAS? (Y/N)"));
  await press(letter("y"));
  await page.until("the IDE to end", async () => (await page.status()).startsWith("finished"));
  const saved = await page.eval(`localStorage.getItem("tp5:SNAKE.PAS")`);
  assert.ok(saved.includes("Write('Points: ', Score, '   ');"), "the edit was saved");
  assert.ok(saved.includes("GotoXY(1, 1);"), "the fix was saved");
});
console.log("idemain: ok");
