// Runs the browser tests one after another in each installed browser (each test file starts
// its own browser) and summarises. Exit 1 if any fails or Chrome is missing; Firefox is
// skipped with a message when it is not installed.
// Usage: node tests/host/browser/run.mjs  (or: make test-browser)
import fs from "node:fs";
import { spawnSync } from "node:child_process";
import { chromeInstalled, CHROME, FIREFOX } from "./chrome.mjs";

if (!chromeInstalled()) {
  console.error(`Chrome not found at ${CHROME}; set CHROME to its executable.`);
  process.exit(1);
}
const browsers = ["chrome"];
if (fs.existsSync(process.env.FIREFOX ?? FIREFOX)) browsers.push("firefox");
else console.log("SKIP firefox: not installed");

let failed = 0;
for (const browser of browsers) {
  for (const name of ["stop", "keys", "latency", "snake", "ide"]) {
    console.log(`[${browser}] ${name}`);
    const run = spawnSync("node", [new URL(`${name}.mjs`, import.meta.url).pathname], {
      encoding: "utf8", timeout: 120000, env: { ...process.env, BROWSER: browser },
    });
    process.stdout.write(run.stdout);
    if (run.status !== 0) {
      failed++;
      process.stderr.write(`FAIL [${browser}] ${name}\n${run.stderr}`);
    }
  }
}
console.log(failed ? `${failed} browser test(s) failed` : "browser tests passed");
process.exit(failed ? 1 : 0);
