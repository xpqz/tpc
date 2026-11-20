// The unit and include files a compile can see: the .pas and .inc files of the main file's
// directory, as the native tp5c finds them, except that an unsaved editor buffer replaces the
// file of the same name on disk.
import { encodeLatin1 } from "./latin1.mjs";

const isSourceName = (name) => /\.(pas|inc)$/i.test(name);

// `diskNames` lists the directory; `readDisk(name)` returns a file's bytes; `buffers` maps the
// names of open editor documents to their text. Returns a Map of name -> bytes, without the
// main file.
export function selectSources({ mainName, diskNames, readDisk, buffers }) {
  const isMain = (name) => name.toUpperCase() === mainName.toUpperCase();
  const sources = new Map();
  const buffered = new Set();
  for (const [name, text] of buffers) {
    if (isSourceName(name) && !isMain(name)) {
      sources.set(name, encodeLatin1(text));
      buffered.add(name.toUpperCase());
    }
  }
  for (const name of diskNames) {
    if (isSourceName(name) && !isMain(name) && !buffered.has(name.toUpperCase())) {
      sources.set(name, readDisk(name));
    }
  }
  return sources;
}
