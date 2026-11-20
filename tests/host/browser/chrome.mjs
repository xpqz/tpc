// Drives the installed Google Chrome headless over the Chrome DevTools Protocol, with Node's
// built-in WebSocket and no npm packages. `withPage` serves the repository (tools/serve.mjs, so
// the page is cross-origin isolated), opens `host/index.html?wasm=...` and hands the test a
// `page` object; Chrome and the server are always shut down afterwards.
import { spawn } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

export const REPO = path.resolve(import.meta.dirname, "../../..");
export const CHROME =
  process.env.CHROME ?? "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
export const chromeInstalled = () => fs.existsSync(CHROME);
const HANG_GUARD_MS = 90000;
export const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

// Polls `probe` until it returns something truthy. The deadline is only a hang guard for a
// condition that can no longer become true; no test asserts on it.
async function until(what, probe, ms = HANG_GUARD_MS) {
  const start = Date.now();
  for (;;) {
    const value = await probe();
    if (value) return value;
    if (Date.now() - start > ms) throw new Error(`timed out waiting for ${what}`);
    await sleep(20);
  }
}

// Resolves with the first match of `pattern` in the child's output stream (`stream` is "stdout"
// or "stderr"): the browser or server announces its endpoint once it is really listening, so
// start-up waits on that announcement instead of probing. Rejects if the child exits first.
function announced(child, stream, pattern, what) {
  return new Promise((resolve, reject) => {
    let seen = "";
    const guard = setTimeout(() => reject(new Error(`timed out waiting for ${what}`)), HANG_GUARD_MS);
    child[stream].setEncoding("utf8");
    child[stream].on("data", (chunk) => {
      seen += chunk;
      const match = pattern.exec(seen);
      if (match) { clearTimeout(guard); resolve(match); }
    });
    child.once("exit", (code) => {
      clearTimeout(guard);
      reject(new Error(`${what}: process exited (${code}) before it was ready:\n${seen}`));
    });
  });
}

// One WebSocket, JSON messages with ids; `send` resolves with the reply's result.
function connect(url) {
  const socket = new WebSocket(url);
  const pending = new Map();
  let nextId = 1;
  socket.onmessage = ({ data }) => {
    const message = JSON.parse(data);
    const waiting = pending.get(message.id);
    if (!waiting) return;
    pending.delete(message.id);
    if (message.error) waiting.reject(new Error(JSON.stringify(message.error)));
    else waiting.resolve(message.result);
  };
  const opened = new Promise((resolve, reject) => {
    socket.onopen = resolve;
    socket.onerror = () => reject(new Error(`cannot connect to ${url}`));
  });
  const send = (method, params = {}) =>
    new Promise((resolve, reject) => {
      const id = nextId++;
      pending.set(id, { resolve, reject });
      socket.send(JSON.stringify({ id, method, params }));
    });
  return { opened, send, close: () => socket.close() };
}

// Starts the repository server on a free port; resolves with { port, stop }.
async function startServer() {
  const port = 20000 + Math.floor(Math.random() * 20000);
  const server = spawn("node", [path.join(REPO, "tools/serve.mjs"), String(port)], {
    stdio: ["ignore", "pipe", "ignore"],
  });
  await announced(server, "stdout", /http:\/\/localhost/, "the dev server");
  return { port, stop: () => server.kill() };
}

async function startChrome(profile) {
  fs.rmSync(profile, { recursive: true, force: true });
  fs.mkdirSync(profile, { recursive: true });
  const chrome = spawn(
    CHROME,
    ["--headless=new", "--remote-debugging-port=0", `--user-data-dir=${profile}`,
     "--no-first-run", "--no-default-browser-check", "--disable-gpu", "about:blank"],
    { stdio: ["ignore", "ignore", "pipe"] },
  );
  const [, port] = await announced(chrome, "stderr", /DevTools listening on ws:\/\/[^:\s]+:(\d+)\//,
    "Chrome's debugging port");
  return { port, stop: () => chrome.kill() };
}

// A browser backend: { navigate(url), run(expression) -> JSON text, key(spec), stop() }.
// Chrome speaks CDP, Firefox WebDriver BiDi; both over Node's WebSocket.
export async function chromeBackend(profile) {
  const chrome = await startChrome(profile);
  const targets = await (await fetch(`http://localhost:${chrome.port}/json/list`)).json();
  const cdp = connect(targets.find((t) => t.type === "page").webSocketDebuggerUrl);
  await cdp.opened;
  await cdp.send("Page.enable");
  await cdp.send("Runtime.enable");
  await cdp.send("Page.addScriptToEvaluateOnNewDocument", { source: CAPTURE });
  const press = (type, spec) => cdp.send("Input.dispatchKeyEvent", { type, ...spec });
  return {
    navigate: (url) => cdp.send("Page.navigate", { url }),
    async run(expression) {
      const reply = await cdp.send("Runtime.evaluate", { expression, awaitPromise: true });
      if (reply.exceptionDetails) throw new Error(reply.exceptionDetails.exception?.description);
      return reply.result.value;
    },
    async key({ vk, w3c, modifiers = 0, ...event }) {
      const base = { ...event, modifiers, windowsVirtualKeyCode: vk };
      await press("rawKeyDown", base);
      if (event.text) await press("char", base);
      await press("keyUp", base);
    },
    // PNG bytes of a page rectangle { x, y, width, height } (Chrome only).
    async screenshot(clip) {
      const { data } = await cdp.send("Page.captureScreenshot",
        { format: "png", clip: { ...clip, scale: 1 } });
      return Buffer.from(data, "base64");
    },
    async contexts() {
      return (await (await fetch(`http://localhost:${chrome.port}/json/list`)).json())
        .filter((t) => t.type === "page").length;
    },
    stop: chrome.stop,
  };
}

// CDP's `modifiers` bits (Alt 1, Ctrl 2, Meta 4, Shift 8) and the WebDriver modifier keys.
export const MODIFIER_KEYS = [[1, "\uE00A"], [2, "\uE009"], [4, "\uE03D"], [8, "\uE008"]];
export const FIREFOX = "/Applications/Firefox.app/Contents/MacOS/firefox";

async function firefoxBackend(profile) {
  fs.rmSync(profile, { recursive: true, force: true });
  fs.mkdirSync(profile, { recursive: true });
  const port = 20000 + Math.floor(Math.random() * 20000);
  const firefox = spawn(
    process.env.FIREFOX ?? FIREFOX,
    ["--headless", "--no-remote", "--remote-debugging-port", String(port), "--profile", profile,
     "about:blank"],
    { stdio: ["ignore", "ignore", "pipe"] },
  );
  // Firefox prints this line only once the BiDi server accepts connections.
  const [, host, listening] = await announced(firefox, "stderr",
    /WebDriver BiDi listening on ws:\/\/([^:\s]+):(\d+)/, "Firefox's BiDi socket");
  const bidi = connect(`ws://${host}:${listening}/session`);
  await bidi.opened;
  await bidi.send("session.new", { capabilities: {} });
  const { contexts } = await bidi.send("browsingContext.getTree");
  const context = contexts[0].context;
  await bidi.send("script.addPreloadScript", { functionDeclaration: `() => { ${CAPTURE} }` });
  return {
    navigate: (url) => bidi.send("browsingContext.navigate", { context, url, wait: "complete" }),
    async run(expression) {
      const reply = await bidi.send("script.evaluate", {
        expression, awaitPromise: true, target: { context },
      });
      if (reply.type === "exception") throw new Error(reply.exceptionDetails.text);
      return reply.result.value;
    },
    async key({ w3c, modifiers = 0 }) {
      const held = MODIFIER_KEYS.filter(([bit]) => modifiers & bit).map(([, value]) => value);
      const press = (type, values) => values.map((value) => ({ type, value }));
      await bidi.send("input.performActions", { context, actions: [{ type: "key", id: "kbd",
        actions: [...press("keyDown", [...held, w3c]),
                  ...press("keyUp", [w3c, ...held.reverse()])] }] });
    },
    // The browser's own view of how many tabs and windows exist (a swallowed chord such as
    // Ctrl+T would change it).
    async contexts() {
      return (await bidi.send("browsingContext.getTree")).contexts.length;
    },
    stop: () => firefox.kill(),
  };
}

// Runs `body(page)` in the browser named by $BROWSER (chrome by default, or firefox).
// page.open(wasmPath) loads the host page with that program; page.eval(expression) runs
// JavaScript in the page (awaiting promises) and returns its JSON value; page.key(spec)
// sends a real key press through the browser's input pipeline; page.click(id) clicks a button.
export async function withPage(body) {
  const server = await startServer();
  const name = process.env.BROWSER ?? "chrome";
  const backend = await (name === "firefox" ? firefoxBackend : chromeBackend)(
    path.join(REPO, `tmp/${name}-profile`));
  try {
    const page = {
      name,
      origin: `http://localhost:${server.port}`,
      // `query` adds page parameters, such as `&compiler=/tmp/browser/compiler.wasm`.
      async open(wasm, query = "") {
        await backend.navigate(`${this.origin}/host/index.html?wasm=${wasm}${query}`);
        await until("the page's terminal", () => this.eval("!!window.__grid"));
      },
      // Statements are fine: the last value is returned, as JSON, whatever the browser.
      async eval(code) {
        const wrapped = `(async () => JSON.stringify((await (0, eval)(${JSON.stringify(code)})) ?? null))()`;
        return JSON.parse(await backend.run(wrapped));
      },
      key: (spec) => backend.key(spec),
      contexts: () => backend.contexts(),
      click: (id) => page.eval(`document.getElementById(${JSON.stringify(id)}).click()`),
      status: () => page.eval(`document.getElementById("status").textContent`),
      // Text of one screen row, read from the shared grid (80 columns, char then attribute).
      row: (n) => page.eval(`window.__row(${n})`),
      until: (what, probe, ms) => until(what, probe, ms),
    };
    return await body(page);
  } finally {
    backend.stop();
    server.stop();
  }
}

// Runs in every page before its own scripts. Wraps Worker.postMessage to keep the grid and
// the key queue, and defines window.__row(n): the row as ASCII (CP437 above 127 shown as ?).
const CAPTURE = `
  const post = Worker.prototype.postMessage;
  Worker.prototype.postMessage = function (message, ...rest) {
    if (message && message.grid) { window.__grid = message.grid; window.__keys = message.keys; }
    return post.call(this, message, ...rest);
  };
  window.__row = (n) => {
    let text = "";
    for (let c = 0; c < 80; c++) {
      const ch = window.__grid[(n * 80 + c) * 2];
      text += ch >= 32 && ch < 127 ? String.fromCharCode(ch) : ch === 0 ? " " : "?";
    }
    return text.trimEnd();
  };
`;

// Key specs: CDP's Input.dispatchKeyEvent fields (`vk` is the Windows virtual key code) and
// `w3c`, the WebDriver key value.
export const KEYS = {
  letter: (ch) => ({ key: ch, text: ch, vk: ch.toUpperCase().charCodeAt(0), w3c: ch }),
  down: { key: "ArrowDown", vk: 40, w3c: "\uE015" },
  right: { key: "ArrowRight", vk: 39, w3c: "\uE014" },
  f12: { key: "F12", vk: 123, w3c: "\uE03C" },
  shift: { key: "Shift", vk: 16, w3c: "\uE008" },
  f2: { key: "F2", vk: 113, w3c: "\uE032" },
  f3: { key: "F3", vk: 114, w3c: "\uE033" },
  f9: { key: "F9", vk: 120, w3c: "\uE039" },
  enter: { key: "Enter", vk: 13, w3c: "\uE007" },
  backspace: { key: "Backspace", vk: 8, w3c: "\uE003" },
  space: { key: " ", text: " ", vk: 32, w3c: " " },
  dot: { key: ".", text: ".", vk: 190, w3c: "." },
};

// Writes wasm bytes under tmp/browser/ (served with the repository); returns the URL path.
export function publishProgram(name, bytes) {
  const dir = path.join(REPO, "tmp/browser");
  fs.mkdirSync(dir, { recursive: true });
  fs.writeFileSync(path.join(dir, `${name}.wasm`), bytes);
  return `/tmp/browser/${name}.wasm`;
}
