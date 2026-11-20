// Builds the compiler artefacts the extension tests need: the wasm32 library and the native
// tp5c binary, into the checkout's target directory (scripts/cargo-target.sh).
import { execFileSync } from "node:child_process";
import path from "node:path";

export const root = path.resolve(import.meta.dirname, "../..");
const targetDir = execFileSync(path.join(root, "scripts/cargo-target.sh"), { encoding: "utf8" }).trim();

function cargo(...args) {
  execFileSync("cargo", args, {
    cwd: root,
    env: { ...process.env, CARGO_TARGET_DIR: targetDir },
    stdio: ["ignore", "ignore", "inherit"],
  });
}

export function buildWasm() {
  cargo("build", "--release", "--lib", "--target", "wasm32-unknown-unknown");
  return path.join(targetDir, "wasm32-unknown-unknown/release/tp5c.wasm");
}

export function buildNative() {
  cargo("build", "--bin", "tp5c");
  return path.join(targetDir, "debug/tp5c");
}

export { targetDir };
