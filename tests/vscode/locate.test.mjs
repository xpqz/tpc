import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { root } from "./build.mjs";

const load = () => import("../../vscode/src/locate.mjs");

test("a configured path is used as it is", async () => {
  const { compilerPath } = await load();
  assert.equal(compilerPath("/opt/tp5c.wasm", path.join(root, "vscode")), "/opt/tp5c.wasm");
});

test("a compiler bundled in the extension is used before the repository's build", async () => {
  const { compilerPath } = await load();
  const dir = fs.mkdtempSync(path.join(root, "tmp/vscode-locate-"));
  try {
    fs.writeFileSync(path.join(dir, "tp5c.wasm"), "");
    assert.equal(compilerPath("", dir), path.join(dir, "tp5c.wasm"));
    assert.equal(compilerPath("/opt/tp5c.wasm", dir), "/opt/tp5c.wasm");
  } finally {
    fs.rmSync(dir, { recursive: true, force: true });
  }
});

test("an empty setting finds the release build of the repository the extension is in", async () => {
  const { compilerPath } = await load();
  const target = execFileSync(path.join(root, "scripts/cargo-target.sh"), { encoding: "utf8" }).trim();
  assert.equal(
    compilerPath("", path.join(root, "vscode")),
    path.join(target, "wasm32-unknown-unknown/release/tp5c.wasm"),
  );
});
