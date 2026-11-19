// The scripted keyboard of the test runner: the key script format of NAME.keys (documented
// in tools/tp5.sh) and a key source for host/runtime.js. Key names and codes are the table
// tools/crt/KEYS.TBL, which tools/crt/keys.awk reads for the real Turbo Pascal.
import fs from "node:fs";

const KEYS_TABLE = new URL("../tools/crt/KEYS.TBL", import.meta.url);

export class KeyScriptError extends Error {}

// Lower-case key name -> INT 16h word, from the lines `Name $HHHH` of the table.
function namedKeys() {
  const keys = new Map();
  for (const line of fs.readFileSync(KEYS_TABLE, "latin1").split("\n")) {
    const [name, hex] = line.split(" ");
    if (hex !== undefined) keys.set(name.toLowerCase(), parseInt(hex.slice(1), 16));
  }
  return keys;
}

// The INT 16h words of a key script: whitespace-separated tokens, each a key name (any case)
// or one printable character, which is typed literally.
export function parseKeys(text) {
  const named = namedKeys();
  return text
    .split(/\s+/)
    .filter((token) => token !== "")
    .map((token) => {
      if (named.has(token.toLowerCase())) return named.get(token.toLowerCase());
      const code = token.charCodeAt(0);
      if (token.length === 1 && code >= 0x20 && code < 0x7f) return code;
      throw new KeyScriptError(`unknown key ${token}`);
    });
}

// A key source for host/runtime.js: serves `words` in order.
export class KeyScript {
  constructor(words) {
    this.words = words;
    this.at = 0;
  }

  available() {
    return this.at < this.words.length;
  }

  next() {
    return this.words[this.at++];
  }
}
