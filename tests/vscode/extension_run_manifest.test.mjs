import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { root } from "./build.mjs";

const pkg = JSON.parse(fs.readFileSync(path.join(root, "vscode/package.json"), "utf8"));
const { contributes } = pkg;

test("the commands are contributed with titles", () => {
  const byId = new Map(contributes.commands.map((c) => [c.command, c]));
  assert.equal(byId.get("tp5.compile").title, "Compile");
  assert.equal(byId.get("tp5.run").title, "Run");
  assert.equal(byId.get("tp5.stop").title, "Stop");
  for (const id of ["tp5.compile", "tp5.run", "tp5.stop"]) {
    assert.equal(byId.get(id).category, "TP5");
  }
  assert.ok(byId.get("tp5.run").icon, "the title-bar button needs an icon");
});

test("F9 compiles and Ctrl+F9 runs, only in Pascal editors", () => {
  const key = (command) => contributes.keybindings.find((k) => k.command === command);
  assert.equal(key("tp5.compile").key, "f9");
  assert.equal(key("tp5.run").key, "ctrl+f9");
  for (const id of ["tp5.compile", "tp5.run"]) {
    assert.match(key(id).when, /editorLangId == tp5pascal/);
  }
});

test("the editor title bar has a run button for Pascal files", () => {
  const entry = contributes.menus["editor/title"].find((m) => m.command === "tp5.run");
  assert.ok(entry);
  assert.match(entry.when, /resourceLangId == tp5pascal/);
});

test("commands are enabled for Pascal files only in the command palette", () => {
  for (const id of ["tp5.compile", "tp5.run"]) {
    const entry = contributes.menus.commandPalette.find((m) => m.command === id);
    assert.match(entry.when, /editorLangId == tp5pascal/);
  }
});

test("tp5.run.writeFiles is a boolean setting, on by default", () => {
  const setting = contributes.configuration.properties["tp5.run.writeFiles"];
  assert.equal(setting.type, "boolean");
  assert.equal(setting.default, true);
  assert.ok(setting.description.length > 0);
});

test("the README has a VS Code section with the launch, keys and the write-through setting", () => {
  const readme = fs.readFileSync(path.join(root, "README.md"), "utf8");
  const section = readme.slice(readme.indexOf("## VS Code"));
  assert.ok(readme.includes("## VS Code"));
  for (const needle of ["--extensionDevelopmentPath", "--user-data-dir", "F9", "Ctrl+F9",
    "tp5.run.writeFiles", "Crt"]) {
    assert.ok(section.includes(needle), needle);
  }
});
