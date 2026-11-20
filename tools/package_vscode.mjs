// Assembles the VS Code extension into vscode-ext/ and packs it as tp5-pascal-VERSION.vsix.
// Usage: node tools/package_vscode.mjs path/to/release/tp5c.wasm [output-directory]
//
// The package is the vscode/ directory plus what it imports: the host modules (host/*.js and
// host/package.json) and the compiler. src/host.mjs is the only file that names the host
// directory, so the package gets a different copy of it. A VSIX is a zip of
// `extension.vsixmanifest`, `[Content_Types].xml` and the extension under `extension/`; it is
// written here from package.json, so no packaging tool is needed.
import { execFileSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.join(path.dirname(fileURLToPath(import.meta.url)), "..");
const tmp = process.argv[3] ?? path.join(root, "tmp");
const extensionDir = path.join(tmp, "vscode-ext");
const stageDir = path.join(tmp, "vsix-stage");
const wasmPath = process.argv[2];

// What src/host.mjs imports and what those import; the rest of host/ is the browser page.
const HOST_FILES = [
  "compiler.js", "error.js", "fmt_real.js", "keys.js", "package.json",
  "runtime.js", "screen.js", "terminal.js", "vfs.js",
];

function assembleExtension() {
  fs.mkdirSync(tmp, { recursive: true });
  fs.rmSync(extensionDir, { recursive: true, force: true });
  fs.cpSync(path.join(root, "vscode"), extensionDir, { recursive: true });

  const hostDir = path.join(extensionDir, "host");
  fs.mkdirSync(hostDir);
  for (const name of fs.readdirSync(path.join(root, "host"))) {
    if (HOST_FILES.includes(name)) {
      fs.copyFileSync(path.join(root, "host", name), path.join(hostDir, name));
    }
  }
  const hostModule = path.join(extensionDir, "src/host.mjs");
  fs.writeFileSync(hostModule, fs.readFileSync(hostModule, "utf8").replaceAll("../../host/", "../host/"));

  fs.copyFileSync(wasmPath, path.join(extensionDir, "tp5c.wasm"));
  fs.copyFileSync(path.join(root, "LICENSE"), path.join(extensionDir, "LICENSE.txt"));
}

const escapeXml = (text) =>
  text.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;").replaceAll('"', "&quot;");

function vsixManifest(pkg) {
  const asset = (type, file) => `<Asset Type="${type}" Path="extension/${file}" Addressable="true" />`;
  return `<?xml version="1.0" encoding="utf-8"?>
<PackageManifest Version="2.0.0" xmlns="http://schemas.microsoft.com/developer/vsx-schema/2011">
  <Metadata>
    <Identity Language="en-US" Id="${escapeXml(pkg.name)}" Version="${escapeXml(pkg.version)}" Publisher="${escapeXml(pkg.publisher)}" />
    <DisplayName>${escapeXml(pkg.displayName)}</DisplayName>
    <Description xml:space="preserve">${escapeXml(pkg.description)}</Description>
    <Categories>${escapeXml(pkg.categories.join(","))}</Categories>
    <GalleryFlags>Public</GalleryFlags>
    <Properties>
      <Property Id="Microsoft.VisualStudio.Code.Engine" Value="${escapeXml(pkg.engines.vscode)}" />
    </Properties>
    <License>extension/LICENSE.txt</License>
  </Metadata>
  <Installation>
    <InstallationTarget Id="Microsoft.VisualStudio.Code" />
  </Installation>
  <Dependencies />
  <Assets>
    ${asset("Microsoft.VisualStudio.Code.Manifest", "package.json")}
    ${asset("Microsoft.VisualStudio.Services.Content.Details", "README.md")}
    ${asset("Microsoft.VisualStudio.Services.Content.License", "LICENSE.txt")}
  </Assets>
</PackageManifest>
`;
}

// Every file extension in the package needs a content type.
function contentTypes(files) {
  const known = { json: "application/json", md: "text/markdown", txt: "text/plain", js: "text/javascript" };
  const extensions = new Set(files.map((file) => path.extname(file).slice(1)).concat("vsixmanifest"));
  const defaults = [...extensions].sort().map((extension) => {
    const type = extension === "vsixmanifest" ? "text/xml" : (known[extension] ?? "application/octet-stream");
    return `  <Default Extension="${extension}" ContentType="${type}" />`;
  });
  return `<?xml version="1.0" encoding="utf-8"?>\n<Types>\n${defaults.join("\n")}\n</Types>\n`;
}

function listFiles(directory) {
  return fs.readdirSync(directory, { recursive: true }).filter((name) =>
    fs.statSync(path.join(directory, name)).isFile(),
  );
}

function packVsix(pkg) {
  fs.rmSync(stageDir, { recursive: true, force: true });
  fs.mkdirSync(stageDir, { recursive: true });
  fs.cpSync(extensionDir, path.join(stageDir, "extension"), { recursive: true });
  fs.writeFileSync(path.join(stageDir, "extension.vsixmanifest"), vsixManifest(pkg));
  fs.writeFileSync(path.join(stageDir, "[Content_Types].xml"), contentTypes(listFiles(extensionDir)));

  const vsixPath = path.join(tmp, `${pkg.name}-${pkg.version}.vsix`);
  fs.rmSync(vsixPath, { force: true });
  execFileSync("zip", ["-q", "-r", "-D", vsixPath, "[Content_Types].xml", "extension.vsixmanifest", "extension"], {
    cwd: stageDir,
  });
  return vsixPath;
}

if (!wasmPath || !fs.existsSync(wasmPath)) {
  console.error("usage: node tools/package_vscode.mjs path/to/tp5c.wasm");
  process.exit(2);
}
const manifest = JSON.parse(fs.readFileSync(path.join(root, "vscode/package.json"), "utf8"));
assembleExtension();
console.log(packVsix(manifest));
