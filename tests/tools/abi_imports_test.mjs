// Checks that the host import list in docs/abi.md (section 8) names the same imports, with the
// same number of parameters and the same result/no-result shape, as the `external` declarations
// in runtime/*.pas and the imports objects of host/runtime.js and host/ide.js.
// Usage: node tests/tools/abi_imports_test.mjs [path/to/abi.md]
// The optional path replaces docs/abi.md, so a deliberately broken copy can be checked.
// The script also checks itself: each mutation of the document in `MUTATIONS` must be reported.
import fs from "node:fs";
import path from "node:path";

const root = path.resolve(import.meta.dirname, "../..");
const docPath = process.argv[2] ?? path.join(root, "docs/abi.md");
const read = (file) => fs.readFileSync(path.join(root, file), "utf8");

const countParams = (list) => (list.trim() === "" ? 0 : list.split(",").length);

// The rows of the table that starts at "| Import | Signature" in the document. A row may name
// several imports (`fs_read` / `fs_write`) that share one signature.
function importsInDoc(text) {
  const imports = new Map();
  const lines = text.split("\n");
  const header = lines.findIndex((line) => line.startsWith("| Import | Signature"));
  for (let i = header + 2; i < lines.length && lines[i].startsWith("|"); i++) {
    const [, names, signature] = lines[i].split("|").map((cell) => cell.trim());
    const match = signature.match(/^`\(([^)]*)\)( -> [^`]*)?`$/);
    if (!match) throw new Error(`abi.md: cannot read the signature in: ${lines[i]}`);
    for (const [, name] of names.matchAll(/`(\w+)`/g)) {
      imports.set(name, { params: countParams(match[1]), result: match[2] !== undefined });
    }
  }
  return imports;
}

// `procedure Name(a, b: T; c: U); external 'import';` in every runtime source. The import is
// the string after `external`, else the routine's name in lower case.
function importsInRuntime() {
  const imports = new Map();
  const declaration =
    /^(procedure|function)\s+(\w+)\s*(?:\(([^)]*)\))?\s*(?::\s*\w+)?\s*;\s*external(?:\s+'(\w+)')?\s*;/gm;
  for (const file of fs.readdirSync(path.join(root, "runtime"))) {
    if (!file.endsWith(".pas")) continue;
    for (const [, kind, name, params = "", alias] of read(`runtime/${file}`).matchAll(declaration)) {
      const count = params
        .split(";")
        .filter((group) => group.trim() !== "")
        .reduce((sum, group) => sum + countParams(group.split(":")[0]), 0);
      const entry = { params: count, result: kind === "function" };
      const import_ = alias ?? name.toLowerCase();
      const earlier = imports.get(import_);
      if (earlier && JSON.stringify(earlier) !== JSON.stringify(entry)) {
        throw new Error(`runtime/*.pas: ${import_} is declared twice with different signatures`);
      }
      imports.set(import_, entry);
    }
  }
  return imports;
}

// `name: (a, b) => ...` entries of the object a host file returns as its imports. `result`
// cannot be told from an arrow function, so it is left out of the comparison.
function importsInHost() {
  const imports = new Map();
  const after = (text, marker) => text.slice(text.lastIndexOf(marker));
  const sources = [
    after(read("host/runtime.js"), "makeImports() {"),
    after(read("host/ide.js"), "export function ideImports"),
  ];
  for (const source of sources) {
    for (const [, name, params] of source.matchAll(/^\s+(\w+): \(([^)]*)\) =>/gm)) {
      imports.set(name, { params: countParams(params) });
    }
  }
  return imports;
}

// The differences between the document and the two code-side lists, as readable lines.
function problems(docText) {
  const doc = importsInDoc(docText);
  const runtime = importsInRuntime();
  const host = importsInHost();
  const found = [];
  const names = new Set([...doc.keys(), ...runtime.keys(), ...host.keys()]);
  for (const name of [...names].sort()) {
    for (const [label, side] of [["runtime/*.pas", runtime], ["host", host]]) {
      if (doc.has(name) !== side.has(name)) {
        found.push(`${name}: ${doc.has(name) ? "in" : "missing from"} abi.md, ${side.has(name) ? "in" : "missing from"} ${label}`);
      } else if (doc.has(name)) {
        const [d, s] = [doc.get(name), side.get(name)];
        if (d.params !== s.params) found.push(`${name}: abi.md has ${d.params} parameters, ${label} ${s.params}`);
        if (s.result !== undefined && d.result !== s.result) found.push(`${name}: result differs between abi.md and ${label}`);
      }
    }
  }
  return { found, count: doc.size };
}

const MUTATIONS = [
  ["a renamed import", (t) => t.replace("| `fs_delete` |", "| `fs_remove` |")],
  ["a removed import", (t) => t.replace(/^\| `sleep` .*\n/m, "")],
  ["a changed parameter list", (t) => t.replace("`(namePtr, nameLen) -> status`", "`(namePtr) -> status`")],
  ["a result added to a procedure", (t) => t.replace("`(ms)`", "`(ms) -> n`")],
];

const doc = fs.readFileSync(docPath, "utf8");
const { found, count } = problems(doc);
if (count < 16) found.push(`only ${count} imports found in abi.md; the table was not read`);
for (const line of found) console.error(`FAIL ${line}`);
if (found.length === 0) console.log(`ok   ${count} imports agree between abi.md, runtime/*.pas and host/`);

// Self-check on the real document only: every mutation must produce a report.
let missed = 0;
if (process.argv[2] === undefined) {
  for (const [name, mutate] of MUTATIONS) {
    const mutated = mutate(doc);
    if (mutated === doc || problems(mutated).found.length === 0) {
      missed++;
      console.error(`FAIL the check does not notice ${name}`);
    }
  }
  if (missed === 0) console.log(`ok   the check notices each of ${MUTATIONS.length} deliberate breakages`);
}
process.exit(found.length + missed > 0 ? 1 : 0);
