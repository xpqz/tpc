// Where the compiler's wasm file is. A configured path wins. Otherwise the extension is
// running from the repository and the file is the release build in the checkout's target
// directory, which scripts/cargo-target.sh names.
import { execFileSync } from "node:child_process";
import path from "node:path";

export function compilerPath(setting, extensionDir) {
  if (setting) return setting;
  const script = path.join(extensionDir, "../scripts/cargo-target.sh");
  const targetDir = execFileSync(script, { encoding: "utf8" }).trim();
  return path.join(targetDir, "wasm32-unknown-unknown/release/tp5c.wasm");
}
