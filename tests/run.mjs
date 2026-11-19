// Test runner: node tests/run.mjs [--dir AUXDIR] [--in INPUT] [--keys KEYS] prog.wasm [expected.out]
// AUXDIR holds the files the program can open (a snapshot; see tests/vfs.mjs); INPUT is the
// text readline returns, by default the process stdin; KEYS is a key script (NAME.keys, see
// tests/keys.mjs) that readkey and keypressed serve, by default no keys. A program that
// imports `screen` ends with the screen dump on stdout, as tools/tp5.sh prints it.
// Exit: program code on match or print mode, 1 on output mismatch, 2 on runner error.
import fs from "node:fs";
import { pathToFileURL } from "node:url";
import { Host, HostError } from "../host/runtime.js";
import { KeyScript, KeyScriptError, parseKeys } from "./keys.mjs";
import { Input, loadSnapshot } from "./vfs.mjs";

export class RunnerError extends Error {
  constructor(message, stdout = Buffer.alloc(0)) {
    super(message);
    this.stdout = stdout;
  }
}

// A virtual clock (decision 38): it starts at the local time in milliseconds since
// 1970-01-01 00:00:00, TP5_NOW_MS when set (tests fix the clock with it) else the real local
// time, and moves only when the program sleeps, so a run never waits.
function virtualClock() {
  let now = Number(process.env.TP5_NOW_MS ?? Date.now() - new Date().getTimezoneOffset() * 60000);
  return { nowMs: () => now, advance: (ms) => (now += ms) };
}

// The INT 16h words of the key script in `text` (a Buffer, or null for no keys).
function keyWords(text) {
  try {
    return text === null ? [] : parseKeys(text.toString("latin1"));
  } catch (e) {
    if (e instanceof KeyScriptError) throw new RunnerError(`bad key script: ${e.message}`);
    throw e;
  }
}

// `files` is the aux directory path or null; `input` is a Buffer of console input or null
// for the process stdin; `keys` is a Buffer holding a key script, or null for no keys.
export async function run(bytes, { files = null, input = null, keys = null } = {}) {
  let module;
  try {
    module = await WebAssembly.compile(bytes);
  } catch (e) {
    throw new RunnerError(`invalid wasm: ${e.message}`);
  }
  const out = [];
  const stdout = () => Buffer.concat(out);
  const sink = (fd, bytes) => {
    if (fd === 1) out.push(Buffer.from(bytes));
    else process.stderr.write(bytes);
  };
  const host = new Host({
    write: sink,
    input: new Input(input),
    files: loadSnapshot(files),
    clock: virtualClock(),
    keys: new KeyScript(keyWords(keys)),
  });
  for (const imp of WebAssembly.Module.imports(module)) {
    if (imp.module !== "host" || imp.kind !== "function" || !(imp.name in host.imports)) {
      throw new RunnerError(`unknown import ${imp.module}.${imp.name}`);
    }
  }
  for (const [name, kind] of [["main", "function"], ["memory", "memory"]]) {
    if (!WebAssembly.Module.exports(module).some((x) => x.name === name && x.kind === kind)) {
      throw new RunnerError(`missing export ${name}`);
    }
  }
  const instance = await WebAssembly.instantiate(module, { host: host.imports });
  host.attach(instance);
  const sp = instance.exports.sp;
  const sp0 = sp?.value;
  try {
    const { code, halted } = host.runMain(instance);
    if (!halted && sp && sp.value !== sp0) {
      throw new RunnerError(`sp not restored: ${sp0} before main, ${sp.value} after`, stdout());
    }
    const usesScreen = WebAssembly.Module.imports(module).some((i) => i.name === "screen");
    if (usesScreen) out.push(Buffer.from(host.screen.dump(code), "latin1"));
    return { stdout: stdout(), code };
  } catch (e) {
    if (e instanceof HostError) throw new RunnerError(e.message, stdout());
    if (e instanceof WebAssembly.RuntimeError) throw new RunnerError(`trap: ${e.message}`, stdout());
    throw e;
  }
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

const USAGE = "usage: run.mjs [--dir AUXDIR] [--in INPUT] [--keys KEYS] prog.wasm [expected.out]";
const OPTIONS = { "--dir": "dir", "--in": "input", "--keys": "keys" };

// Splits the command line into its options and the positional arguments.
function parseArgs(args) {
  const options = { dir: null, input: null, keys: null };
  const positional = [];
  for (let i = 0; i < args.length; i++) {
    if (Object.hasOwn(OPTIONS, args[i])) {
      if (i + 1 >= args.length) throw new RunnerError(USAGE);
      options[OPTIONS[args[i]]] = args[++i];
    } else {
      positional.push(args[i]);
    }
  }
  if (positional.length < 1 || positional.length > 2) throw new RunnerError(USAGE);
  return { options, positional };
}

async function main(argv) {
  const { options, positional: args } = parseArgs(argv);
  const wasm = read(args[0]);
  const expected = args[1] === undefined ? null : read(args[1]);
  if (options.dir !== null && !fs.existsSync(options.dir)) {
    throw new RunnerError(`cannot read directory ${options.dir}`);
  }
  const input = options.input === null ? null : read(options.input);
  const keys = options.keys === null ? null : read(options.keys);
  const { stdout, code } = await run(wasm, { files: options.dir, input, keys });
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
