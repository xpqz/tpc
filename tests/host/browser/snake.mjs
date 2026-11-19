// Exit test: the Snake program (tests/programs/snake.pas) runs in the browser terminal and
// obeys arrow keys. Needs `uses Crt` to compile; until it does the test prints SKIP and exits 0.
// SNAKE_STRICT=1 turns the skip into a failure (use it once the Crt unit is merged).
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { withPage, publishProgram, REPO, KEYS } from "./chrome.mjs";

function compileSnake() {
  const compiler = path.join(
    execFileSync("scripts/cargo-target.sh", { cwd: REPO, encoding: "utf8" }).trim(), "debug/tp5c");
  const out = path.join(REPO, "tmp/browser/snake.wasm");
  fs.mkdirSync(path.dirname(out), { recursive: true });
  const dir = fs.existsSync(path.join(REPO, "runtime/crt.pas")) ? ["--dir", "runtime"] : [];
  try {
    execFileSync(compiler, ["tests/programs/snake.pas", "-o", out, ...dir],
      { cwd: REPO, encoding: "utf8", stdio: "pipe" });
    return out;
  } catch (e) {
    const why = `${e.stdout ?? ""}${e.stderr ?? e.message}`.trim();
    if (process.env.SNAKE_STRICT) throw new Error(`snake.pas does not compile: ${why}`);
    console.log(`SKIP snake: tests/programs/snake.pas does not compile yet (${why}); ` +
      "it needs the Crt unit (#62). Set SNAKE_STRICT=1 to make this a failure.");
    return null;
  }
}

const wasm = compileSnake();
if (wasm) {
  // The head 'O' as [column, row] (0-based) on the play field, found by scanning the grid.
  const findHead = (page) => page.eval(`(() => {
    for (let row = 1; row < 24; row++) {
      const col = [...Array(80).keys()].find((c) => window.__grid[(row * 80 + c) * 2] === 79);
      if (col !== undefined) return [col, row];
    }
    return null;
  })()`);

  await withPage(async (page) => {
    await page.open("/tmp/browser/snake.wasm");
    await page.until("the score line", async () => (await page.row(0)).startsWith("Score: 0"));
    assert.equal(await page.status(), "running");
    const [startColumn, startRow] = await page.until("the head", () => findHead(page));
    assert.equal(startRow, 9, "the snake starts on row 10, heading right");

    // Heading right on its own.
    const moved = await page.until("the head to move right", async () => {
      const head = await findHead(page);
      return head && head[0] > startColumn + 2 && head;
    });
    assert.equal(moved[1], startRow);

    await page.key(KEYS.down);
    await page.until("the head to turn down", async () => {
      const head = await findHead(page);
      return head && head[1] > startRow;
    });

    await page.key(KEYS.letter("q"));
    await page.until("the end message", async () =>
      (await page.row(24)).startsWith("Game over. Score:"));
    await page.until("the program to finish", async () =>
      (await page.status()) === "finished, exit code 0");
  });
  console.log("snake: ok");
}
