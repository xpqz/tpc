import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { root } from "./build.mjs";

const dir = path.join(root, "vscode");
const readJson = (file) => JSON.parse(fs.readFileSync(path.join(dir, file), "utf8"));

test("package.json declares the extension without dependencies", () => {
  const pkg = readJson("package.json");
  assert.equal(pkg.name, "tp5-pascal");
  assert.equal(pkg.displayName, "Turbo Pascal 5.0 (TP5-WASM)");
  assert.equal(pkg.engines.vscode, "^1.90.0");
  assert.equal(pkg.main, "./extension.cjs");
  assert.equal(pkg.dependencies, undefined);
  assert.equal(pkg.devDependencies, undefined);
  assert.ok(fs.existsSync(path.join(dir, pkg.main)));
  assert.ok(pkg.activationEvents.includes("onLanguage:tp5pascal"));
});

test("the language, grammar and settings are contributed", () => {
  const { contributes } = readJson("package.json");
  const [language] = contributes.languages;
  assert.equal(language.id, "tp5pascal");
  assert.deepEqual(language.extensions.sort(), [".inc", ".pas"]);
  assert.ok(language.aliases.includes("Turbo Pascal"));
  assert.ok(fs.existsSync(path.join(dir, language.configuration)));
  const [grammar] = contributes.grammars;
  assert.equal(grammar.language, "tp5pascal");
  assert.equal(grammar.scopeName, "source.tp5pascal");
  assert.ok(fs.existsSync(path.join(dir, grammar.path)));
  const settings = contributes.configuration.properties;
  assert.equal(settings["tp5.diagnostics.onType"].type, "boolean");
  assert.equal(settings["tp5.diagnostics.onType"].default, true);
  assert.equal(settings["tp5.compilerPath"].type, "string");
});

test("the CommonJS entry only requires vscode and loads the ES modules with import()", () => {
  const source = fs.readFileSync(path.join(dir, "extension.cjs"), "utf8");
  const required = [...source.matchAll(/require\(["']([^"']+)["']\)/g)].map((m) => m[1]);
  assert.ok(required.every((name) => name === "vscode"), required.join());
  assert.match(source, /import\(/);
  assert.match(source, /exports\.activate|module\.exports/);
});

test("no source file imports the vscode module or an npm package", () => {
  for (const file of fs.readdirSync(path.join(dir, "src"))) {
    const source = fs.readFileSync(path.join(dir, "src", file), "utf8");
    for (const [, spec] of source.matchAll(/(?:from|import\()\s*["']([^"']+)["']/g)) {
      assert.ok(spec.startsWith(".") || spec.startsWith("node:"), `${file} imports ${spec}`);
    }
  }
});

test("the host ES modules load as ES modules in an extension host without module detection", () => {
  const hostPackage = path.join(root, "host/package.json");
  assert.deepEqual(JSON.parse(fs.readFileSync(hostPackage, "utf8")), { type: "module" });
});

test("the language configuration parses and its regular expressions compile", () => {
  const config = readJson("language-configuration.json");
  assert.deepEqual(config.comments.blockComment, ["{", "}"]);
  assert.equal(config.comments.lineComment, undefined, "TP 5.0 has no line comments");
  assert.ok(config.brackets.some(([open, close]) => open === "(" && close === ")"));
  assert.ok(config.autoClosingPairs.some((p) => p.open === "'" && p.close === "'"));
  assert.ok(config.autoClosingPairs.some((p) => p.open === "{" && p.close === "}"));
  const word = new RegExp(config.wordPattern);
  assert.equal("my_var1".match(new RegExp(config.wordPattern, "g"))[0], "my_var1");
  assert.ok(word.test("x"));
  const { start, end } = config.folding.markers;
  const startRe = new RegExp(start, "i");
  const endRe = new RegExp(end, "i");
  for (const line of ["begin", "  CASE x OF", "type T = record", "  { region }"]) {
    assert.ok(startRe.test(line), line);
  }
  for (const line of ["end;", "  END.", "  end"]) assert.ok(endRe.test(line), line);
  assert.ok(!startRe.test("x := 1;"));
});
