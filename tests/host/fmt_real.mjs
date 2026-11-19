// Checks host/fmt_real.js against TP 5.0 output (cases generated with tools/tp5.sh; exact ties of
// magnitude 8 and above are left out, TP 5.0 rounds some of them down). Exit 0 on success.
import fs from "node:fs";
import { formatReal, fmtReal } from "../../host/fmt_real.js";

const here = new URL(".", import.meta.url);
const cases = JSON.parse(fs.readFileSync(new URL("fmt_real_cases.json", here), "utf8"));
if (cases.length < 400) throw new Error(`only ${cases.length} cases`);
const bad = [];
for (const [x, w, d, want] of cases) {
  const got = formatReal(x, w, d);
  if (got !== want) bad.push(`formatReal(${x}, ${w}, ${d}) = [${got}], TP5 [${want}]`);
}
// TP 5.0 has no negative zero: -0 formats as 0.
for (const [w, d] of [[-1, -1], [0, 1], [8, 2], [12, -1]]) {
  const want = formatReal(0, w, d);
  const got = formatReal(-0, w, d);
  if (got !== want || got.includes("-")) bad.push(`formatReal(-0, ${w}, ${d}) = [${got}], want [${want}]`);
}
// fmtReal stores the bytes at ptr and returns the length, touching nothing else.
const memory = new WebAssembly.Memory({ initial: 1 });
const bytes = new Uint8Array(memory.buffer);
for (const [x, w, d] of [[3.75, 0, 2], [-1234.5, 12, -1], [0.5, -1, -1], [100, 10, 3]]) {
  bytes.fill(0x7a);
  const want = formatReal(x, w, d);
  const len = fmtReal(memory, x, w, d, 100);
  const got = Buffer.from(bytes.slice(100, 100 + len)).toString("latin1");
  if (len !== want.length || got !== want) bad.push(`fmtReal(${x}, ${w}, ${d}) = ${len} [${got}], want [${want}]`);
  if (bytes[99] !== 0x7a || bytes[100 + len] !== 0x7a) bad.push(`fmtReal(${x}, ${w}, ${d}) wrote outside its result`);
}
if (bad.length) {
  console.error(bad.slice(0, 30).join("\n"), bad.length > 30 ? `\n... ${bad.length} failures` : "");
  process.exit(1);
}
