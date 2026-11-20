import test from "node:test";
import assert from "node:assert/strict";

const load = () => import("../../vscode/src/entry.mjs");

test("a program is compiled as it is", async () => {
  const { compileEntry } = await load();
  const text = "program p;\nbegin\nend.\n";
  assert.deepEqual(compileEntry("p.pas", text), { mainName: "p.pas", mainText: text });
});

test("a unit is checked through a program that uses it", async () => {
  const { compileEntry } = await load();
  const entry = compileEntry("h.pas", "{ header }\n(* more *)\n  UNIT Helper;\ninterface\nimplementation\nend.\n");
  assert.notEqual(entry.mainName, "h.pas");
  assert.equal(entry.mainText, "program Tp5UnitCheck; uses Helper; begin end.");
});

test("the word unit in a comment or later in a program does not make it a unit", async () => {
  const { compileEntry } = await load();
  const text = "{ unit X; }\nprogram p;\nbegin\nend.\n";
  assert.equal(compileEntry("p.pas", text).mainName, "p.pas");
});
