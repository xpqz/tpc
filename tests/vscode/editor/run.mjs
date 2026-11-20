// Runs the in-editor tests (index.cjs) in the installed VS Code, with its own profile and
// extension directory so the user's settings and extensions are not touched. The profile is
// under the system temp directory, not tmp/: VS Code listens on a socket in it, and a Unix
// socket path is limited to about 100 characters. It opens a window.
// TP5_EXTENSION_DIR tests another extension directory, e.g. tmp/vscode-ext from `make vscode`. VSCODE_BIN names the executable to use; otherwise the VS Code app is
// looked for in the usual places.
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "../../..");
const tmp = path.join(root, "tmp");

function findVscode() {
  const candidates = [
    process.env.VSCODE_BIN,
    "/Applications/Visual Studio Code.app/Contents/MacOS/Electron",
    "/Applications/Visual Studio Code.app/Contents/MacOS/Code",
    "/usr/share/code/code",
    "/usr/bin/code",
  ];
  const found = candidates.find((candidate) => candidate && fs.existsSync(candidate));
  if (!found) {
    console.error("VS Code not found; set VSCODE_BIN to its executable");
    process.exit(2);
  }
  return found;
}

const scratch = path.join(tmp, "vsc-test-work");
fs.rmSync(scratch, { recursive: true, force: true });
fs.mkdirSync(scratch, { recursive: true });

const profile = fs.mkdtempSync(path.join(os.tmpdir(), "tp5vsc-"));
const env = { ...process.env, TP5_EDITOR_SCRATCH: scratch };
delete env.ELECTRON_RUN_AS_NODE;
const result = spawnSync(
  findVscode(),
  [
    `--extensionDevelopmentPath=${process.env.TP5_EXTENSION_DIR ?? path.join(root, "vscode")}`,
    `--extensionTestsPath=${path.join(root, "tests/vscode/editor/index.cjs")}`,
    `--user-data-dir=${path.join(profile, "data")}`,
    `--extensions-dir=${path.join(profile, "exts")}`,
    "--disable-extensions",
    "--disable-workspace-trust",
    "--skip-welcome",
    "--skip-release-notes",
    scratch,
  ],
  { env, encoding: "utf8", timeout: 180000 },
);
// VS Code's agent host logs a line per second to stderr; the test results are on stdout.
process.stdout.write(result.stdout);
process.stderr.write(result.stderr.split("\n").filter((line) => !line.includes("[AgentHost]")).join("\n"));
fs.rmSync(profile, { recursive: true, force: true });
if (result.error) console.error(result.error.message);
process.exit(result.status ?? 1);
