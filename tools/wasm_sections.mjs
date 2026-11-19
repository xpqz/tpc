// Prints the section sizes of WebAssembly modules, and the size with custom sections removed.
// Usage: node tools/wasm_sections.mjs [--no-custom] a.wasm [b.wasm ...]
// With --no-custom the exit status is 1 if a module has a custom section (name, producers, ...).
import fs from "node:fs";

const SECTION_NAMES = ["custom", "type", "import", "function", "table", "memory", "global",
  "export", "start", "element", "code", "data", "datacount"];

function readLeb(bytes, at) {
  let value = 0, shift = 0, pos = at;
  for (;;) {
    const b = bytes[pos++];
    value += (b & 0x7f) * 2 ** shift;
    shift += 7;
    if (!(b & 0x80)) return [value, pos];
  }
}

// One entry per section: its name, total bytes (id and length prefix included), entry count.
export function sections(bytes) {
  const found = [];
  let pos = 8;
  while (pos < bytes.length) {
    const start = pos;
    const id = bytes[pos++];
    const [size, bodyStart] = readLeb(bytes, pos);
    pos = bodyStart + size;
    let name = SECTION_NAMES[id];
    let count = null;
    if (id === 0) {
      const [nameLen, nameStart] = readLeb(bytes, bodyStart);
      name += ":" + Buffer.from(bytes.subarray(nameStart, nameStart + nameLen)).toString();
    } else if (id !== 8 && id !== 5 && id !== 4) {
      count = readLeb(bytes, bodyStart)[0];
    }
    found.push({ name, bytes: pos - start, count, id });
  }
  return found;
}

if (process.argv[1] === new URL(import.meta.url).pathname) {
  const args = process.argv.slice(2);
  const requireNoCustom = args[0] === "--no-custom";
  let failed = false;
  for (const file of requireNoCustom ? args.slice(1) : args) {
    const bytes = fs.readFileSync(file);
    const list = sections(bytes);
    const custom = list.filter((s) => s.id === 0).reduce((n, s) => n + s.bytes, 0);
    if (requireNoCustom && custom > 0) failed = true;
    console.log(`${file}: ${bytes.length} B, ${bytes.length - custom} B without custom sections`);
    for (const s of list) {
      console.log(`  ${s.name.padEnd(28)} ${String(s.bytes).padStart(8)} B${s.count === null ? "" : "  " + s.count + " entries"}`);
    }
  }
  if (failed) {
    console.error("custom sections present");
    process.exit(1);
  }
}
