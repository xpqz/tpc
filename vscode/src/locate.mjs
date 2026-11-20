// Where the compiler's wasm file is. A configured path wins, then the copy bundled inside the
// packaged extension. Otherwise the extension is running from the repository and the file is the
// release build in the checkout's target directory, which scripts/cargo-target.sh names.
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

export function compilerPath(setting, extensionDir) {
  if (setting) return setting;
  const bundled = path.join(extensionDir, "tp5c.wasm");
  if (fs.existsSync(bundled)) return bundled;
  const script = path.join(extensionDir, "../scripts/cargo-target.sh");
  const targetDir = execFileSync(script, { encoding: "utf8" }).trim();
  return path.join(targetDir, "wasm32-unknown-unknown/release/tp5c.wasm");
}
