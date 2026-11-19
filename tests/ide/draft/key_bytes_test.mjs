// Every named key of tools/crt/KEYS.TBL, as a browser event, must map (draft/key_bytes.js)
// to the bytes the runner serves for that name (tests/keys.mjs), so the scripted keys of the
// Node runner and a real keyboard in the browser cannot drift apart. Also checks the old
// plain keys, modifier combinations that are not DOS keys, and that the table matches the
// Reference Guide's Table C.2 for a spread of codes.
import assert from "node:assert/strict";
import fs from "node:fs";
import { KeyScript, parseKeys } from "../../keys.mjs";
import { keyBytes } from "./key_bytes.js";

const TABLE = fs.readFileSync(new URL("../../../tools/crt/KEYS.TBL", import.meta.url), "latin1");
const NAMES = TABLE.split("\n").filter((line) => line !== "").map((line) => line.split(" ")[0]);

const KEY_OF = { Left: "ArrowLeft", Right: "ArrowRight", Up: "ArrowUp", Down: "ArrowDown",
  PgUp: "PageUp", PgDn: "PageDown", Ins: "Insert", Del: "Delete", Esc: "Escape", Space: " " };
const base = { ctrlKey: false, altKey: false, shiftKey: false };

// The keydown event a browser sends for a key name of the table (null: no such key event).
function eventFor(name) {
  if (name === "Ctrl-PrtSc") return null; // browsers do not deliver PrintScreen with Ctrl
  if (/^Alt-[0-9=-]$/.test(name)) return null; // Alt+digit is left out (draft/key_bytes.js)
  const caret = /^\^([A-Z])$/.exec(name);
  if (caret) return { ...base, ctrlKey: true, key: caret[1].toLowerCase(), code: `Key${caret[1]}` };
  const [, modifier, rest] = /^(?:(Ctrl|Alt|Shift)-)?(.+)$/.exec(name);
  const flags = modifier ? { [`${modifier.toLowerCase()}Key`]: true } : {};
  const event = { ...base, ...flags };
  if (modifier === "Alt" && rest.length === 1) {
    return { ...event, key: rest.toLowerCase(), code: `Key${rest}` };
  }
  return { ...event, key: KEY_OF[rest] ?? rest, code: KEY_OF[rest] ?? rest };
}

let checked = 0;
for (const name of NAMES) {
  const event = eventFor(name);
  if (event === null) continue;
  const wanted = new KeyScript(parseKeys(name)).bytes;
  assert.deepEqual(keyBytes(event), wanted, `${name}: ${JSON.stringify(event)}`);
  checked++;
}
assert.ok(checked > 120, `only ${checked} keys checked`);

// Not DOS keys: no bytes (and so the page leaves the event to the browser).
const none = (event) => assert.deepEqual(keyBytes({ ...base, ...event }), [], JSON.stringify(event));
none({ key: "F13", code: "F13" });
none({ key: "Shift", code: "ShiftLeft" });
none({ key: "1", code: "Digit1", ctrlKey: true });
none({ key: "F13", code: "F13", altKey: true });
none({ key: "Ctrl", code: "ControlLeft", ctrlKey: true });
// A symbol typed with Option on macOS is identified by its physical key.
assert.deepEqual(keyBytes({ ...base, altKey: true, key: "≈", code: "KeyX" }), [0, 45]);
// Typing wins where Alt or AltGr makes a character: Option+L is "@" on a German Mac, AltGr+Q
// (Ctrl+Alt) is "@" on a German PC, Option+5 is "[".
const typed = (event, char) => assert.deepEqual(keyBytes({ ...base, ...event }), [char.charCodeAt(0)]);
typed({ key: "@", code: "KeyL", altKey: true }, "@");
typed({ key: "@", code: "KeyQ", altKey: true, ctrlKey: true }, "@");
typed({ key: "[", code: "Digit5", altKey: true }, "[");
// Spot checks against Table C.2 itself (page 424), independent of KEYS.TBL.
assert.deepEqual(keyBytes({ ...base, ctrlKey: true, key: "F9", code: "F9" }), [0, 102]);
assert.deepEqual(keyBytes({ ...base, altKey: true, key: "F10", code: "F10" }), [0, 113]);
assert.deepEqual(keyBytes({ ...base, shiftKey: true, key: "F12", code: "F12" }), [0, 136]);
assert.deepEqual(keyBytes({ ...base, ctrlKey: true, key: "k", code: "KeyK" }), [11]);
console.log(`key_bytes: ${checked} table keys and the negative cases ok`);
