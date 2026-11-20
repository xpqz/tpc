import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import path from "node:path";
import { root } from "./build.mjs";

const load = () => import("../../vscode/src/locate.mjs");

test("a configured path is used as it is", async () => {
  const { compilerPath } = await load();
  assert.equal(compilerPath("/opt/tp5c.wasm", path.join(root, "vscode")), "/opt/tp5c.wasm");
});

test("an empty setting finds the release build of the repository the extension is in", async () => {
  const { compilerPath } = await load();
  const target = execFileSync(path.join(root, "scripts/cargo-target.sh"), { encoding: "utf8" }).trim();
  assert.equal(
    compilerPath("", path.join(root, "vscode")),
    path.join(target, "wasm32-unknown-unknown/release/tp5c.wasm"),
  );
});
