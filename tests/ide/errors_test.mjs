// The IDE's table of error messages (ide/errors.pas) mirrors TABLE in compiler/src/diag.rs:
// the same numbers in the same order with the same texts. Usage: node tests/ide/errors_test.mjs
import assert from "node:assert/strict";
import fs from "node:fs";

const read = (path) => fs.readFileSync(new URL(`../../${path}`, import.meta.url), "latin1");

// `    (85, "\";\" expected"),` in diag.rs
const compilerTable = [...read("compiler/src/diag.rs").matchAll(/^    \((\d+), "((?:[^"\\]|\\.)*)"\),$/gm)]
  .map(([, code, text]) => [Number(code), text.replace(/\\"/g, '"')]);
// `    (Code: 85; Text: '";" expected'),` in errors.pas
const ideTable = [...read("ide/errors.pas").matchAll(/^\s+\(Code: (\d+); Text: '([^']*)'\),?$/gm)]
  .map(([, code, text]) => [Number(code), text]);

assert.ok(compilerTable.length > 70, `diag.rs table not found (${compilerTable.length} entries)`);
assert.deepEqual(ideTable, compilerTable);
const declared = Number(read("ide/errors.pas").match(/EntryCount = (\d+);/)[1]);
assert.equal(declared, compilerTable.length, "EntryCount");
const width = Number(read("ide/errors.pas").match(/Text: string\[(\d+)\]/)[1]);
assert.equal(width, Math.max(...compilerTable.map(([, text]) => text.length)), "the text field is as wide as the longest text");
console.log(`errors: ${compilerTable.length} messages agree`);
