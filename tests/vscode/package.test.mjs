// tools/package_vscode.mjs: the extension directory is self-contained and the VSIX has the
// structure VS Code installs from.
import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";
import { root } from "./build.mjs";

const out = fs.mkdtempSync(path.join(root, "tmp/vscode-package-"));
const fakeWasm = path.join(out, "tp5c.wasm");
fs.writeFileSync(fakeWasm, Buffer.from([0, 0x61, 0x73, 0x6d, 1, 0, 0, 0]));
const vsix = execFileSync("node", [path.join(root, "tools/package_vscode.mjs"), fakeWasm, out], {
  encoding: "utf8",
}).trim();
const entries = execFileSync("unzip", ["-Z1", vsix], { encoding: "utf8" }).split("\n");
const unzipText = (entry) => execFileSync("unzip", ["-p", vsix, entry.replace(/[\[\]]/g, "\\$&")], { encoding: "utf8" });

test("the VSIX holds the manifest, the content types and the extension", () => {
  assert.equal(path.basename(vsix), "tp5-pascal-0.1.0.vsix");
  for (const entry of [
    "extension.vsixmanifest",
    "[Content_Types].xml",
    "extension/package.json",
    "extension/extension.cjs",
    "extension/tp5c.wasm",
    "extension/LICENSE.txt",
    "extension/README.md",
    "extension/host/runtime.js",
    "extension/host/package.json",
  ]) {
    assert.ok(entries.includes(entry), `${entry} is in the VSIX`);
  }
});

test("the manifest carries the identity from package.json", () => {
  const manifest = unzipText("extension.vsixmanifest");
  assert.match(manifest, /Id="tp5-pascal" Version="0\.1\.0" Publisher="xpqz"/);
  assert.match(manifest, /Microsoft\.VisualStudio\.Code\.Engine" Value="\^1\.90\.0"/);
  assert.match(manifest, /<Categories>Programming Languages<\/Categories>/);
  const types = unzipText("[Content_Types].xml");
  for (const extension of ["cjs", "mjs", "js", "json", "wasm", "md", "txt", "vsixmanifest"]) {
    assert.match(types, new RegExp(`Extension="${extension}"`));
  }
});

test("the packaged extension imports the host modules from inside itself", async () => {
  const hostModule = fs.readFileSync(path.join(out, "vscode-ext/src/host.mjs"), "utf8");
  assert.doesNotMatch(hostModule, /\.\.\/\.\.\/host/);
  const host = await import(pathToFileURL(path.join(out, "vscode-ext/src/host.mjs")));
  assert.equal(typeof host.Compiler, "function");
  assert.equal(typeof host.keyBytes, "function");
  assert.equal(fs.existsSync(path.join(out, "vscode-ext/host/main.js")), false);
});

test.after(() => fs.rmSync(out, { recursive: true, force: true }));
