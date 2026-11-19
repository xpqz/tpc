// Test runner: node tests/run.mjs prog.wasm [expected.out]
// Exit: program code on match or print mode, 1 on output mismatch, 2 on runner error.
import fs from "node:fs";
import { pathToFileURL } from "node:url";
import { fmtReal } from "../host/fmt_real.js";

class Halt {
  constructor(code) {
    this.code = code;
  }
}

export class RunnerError extends Error {
  constructor(message, stdout = Buffer.alloc(0)) {
    super(message);
    this.stdout = stdout;
  }
}

const STUBS = ["readkey", "keypressed", "readline", "screen", "fs_open", "fs_read", "fs_write",
  "fs_close", "time", "compile"];

export async function run(bytes) {
  let module;
  try {
    module = await WebAssembly.compile(bytes);
  } catch (e) {
    throw new RunnerError(`invalid wasm: ${e.message}`);
  }
  const out = [];
  let memory = null;
  const stdout = () => Buffer.concat(out);
  const fail = (msg) => new RunnerError(msg, stdout());
  const host = {
    write(fd, ptr, len) {
      if (fd !== 1 && fd !== 2) throw fail(`host.write: bad fd ${fd}`);
      if (len === 0) return;
      if (ptr < 0 || len < 0 || ptr + len > memory.buffer.byteLength) {
        throw fail(`host.write: out of bounds (${ptr}, ${len})`);
      }
      const chunk = Buffer.from(memory.buffer.slice(ptr, ptr + len));
      if (fd === 1) out.push(chunk);
      else process.stderr.write(chunk);
    },
    halt(code) {
      throw new Halt(code);
    },
    fmt_real(x, width, decimals, ptr) {
      return fmtReal(memory, x, width, decimals, ptr);
    },
    rterror(code) {
      out.push(Buffer.from(`Runtime error ${code}\n`));
      throw new Halt(code);
    },
  };
  for (const name of STUBS) {
    host[name] = () => {
      throw fail(`host import ${name} not available in test runner`);
    };
  }
  for (const imp of WebAssembly.Module.imports(module)) {
    if (imp.module !== "host" || imp.kind !== "function" || !(imp.name in host)) {
      throw new RunnerError(`unknown import ${imp.module}.${imp.name}`);
    }
  }
  for (const [name, kind] of [["main", "function"], ["memory", "memory"]]) {
    if (!WebAssembly.Module.exports(module).some((x) => x.name === name && x.kind === kind)) {
      throw new RunnerError(`missing export ${name}`);
    }
  }
  const instance = await WebAssembly.instantiate(module, { host });
  memory = instance.exports.memory;
  const sp = instance.exports.sp;
  const sp0 = sp?.value;
  try {
    instance.exports.main();
    if (sp && sp.value !== sp0) throw fail(`sp not restored: ${sp0} before main, ${sp.value} after`);
  } catch (e) {
    if (e instanceof Halt) return { stdout: stdout(), code: e.code };
    if (e instanceof RunnerError) throw e;
    if (e instanceof RangeError) {
      out.push(Buffer.from("Runtime error 202\n"));
      return { stdout: stdout(), code: 202 };
    }
    if (e instanceof WebAssembly.RuntimeError) throw fail(`trap: ${e.message}`);
    throw e;
  }
  return { stdout: stdout(), code: 0 };
}

const show = (s) => s.replace(/[^\x20-\x7e]/g, (c) => `\\x${c.charCodeAt(0).toString(16).padStart(2, "0")}`);

function diffLines(a, b) {
  const n = a.length;
  const m = b.length;
  const lcs = Array.from({ length: n + 1 }, () => new Array(m + 1).fill(0));
  for (let i = n - 1; i >= 0; i--) {
    for (let j = m - 1; j >= 0; j--) {
      lcs[i][j] = a[i] === b[j] ? lcs[i + 1][j + 1] + 1 : Math.max(lcs[i + 1][j], lcs[i][j + 1]);
    }
  }
  const lines = [];
  let i = 0;
  let j = 0;
  while (i < n || j < m) {
    if (i < n && j < m && a[i] === b[j]) lines.push(` ${show(a[i++])}`), j++;
    else if (j >= m || (i < n && lcs[i + 1][j] >= lcs[i][j + 1])) lines.push(`-${show(a[i++])}`);
    else lines.push(`+${show(b[j++])}`);
  }
  return lines;
}

function mismatch(path, expected, actual) {
  let at = 0;
  while (at < expected.length && at < actual.length && expected[at] === actual[at]) at++;
  const lines = (buf) => buf.toString("latin1").split("\n");
  return [`FAIL ${path}`, "--- expected", "+++ actual", ...diffLines(lines(expected), lines(actual)),
    `first difference at byte ${at}`, ""].join("\n");
}

function read(path) {
  try {
    return fs.readFileSync(path);
  } catch (e) {
    throw new RunnerError(`cannot read ${path}: ${e.code}`);
  }
}

async function main(args) {
  if (args.length < 1 || args.length > 2) throw new RunnerError("usage: run.mjs prog.wasm [expected.out]");
  const wasm = read(args[0]);
  const expected = args[1] === undefined ? null : read(args[1]);
  const { stdout, code } = await run(wasm);
  if (expected === null) {
    process.stdout.write(stdout);
  } else if (!stdout.equals(expected)) {
    process.stderr.write(mismatch(args[0], expected, stdout));
    return 1;
  }
  return code;
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  try {
    process.exitCode = await main(process.argv.slice(2));
  } catch (e) {
    if (!(e instanceof RunnerError)) throw e;
    process.stdout.write(e.stdout);
    process.stderr.write(`run.mjs: ${e.message}\n`);
    process.exitCode = 2;
  }
}
