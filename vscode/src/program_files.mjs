// A program's files are the 8.3-named regular files of its directory. They are read once into
// a snapshot when the program starts; changes the program makes can be written through to the
// directory afterwards. Nothing here can reach outside the directory: names are checked to be
// plain 8.3 names (no separator, no dot-dot), and a symbolic link or directory is never read,
// written or deleted.
import fs from "node:fs";
import path from "node:path";

const NAME_CHARS = "[A-Z0-9_$~!#%&\\-{}()@'^]";
const DOS_NAME = new RegExp(`^${NAME_CHARS}{1,8}(\\.${NAME_CHARS}{0,3})?$`, "i");

function listEntries(directory) {
  try {
    return fs.readdirSync(directory, { withFileTypes: true });
  } catch {
    return [];
  }
}

// Map file name -> bytes of the regular files whose name the program's file system can have.
export function readSnapshot(directory) {
  const files = new Map();
  for (const entry of listEntries(directory)) {
    if (entry.isFile() && DOS_NAME.test(entry.name)) {
      files.set(entry.name, new Uint8Array(fs.readFileSync(path.join(directory, entry.name))));
    }
  }
  return files;
}

// The path of `name` in the directory: a file already there keeps its spelling (the program's
// names are upper case, DOS is case-blind), a new file is named as given. Undefined for a name
// that is not 8.3, or one that exists as something other than a regular file.
function targetPath(directory, name) {
  if (!DOS_NAME.test(name)) return undefined;
  const existing = listEntries(directory).find((e) => e.name.toUpperCase() === name.toUpperCase());
  if (existing && !existing.isFile()) return undefined;
  return path.join(directory, existing ? existing.name : name);
}

// Writes `name` and removes `name` in `directory`; each returns an error text, or undefined.
export function diskWriter(directory) {
  const attempt = (name, change) => {
    const target = targetPath(directory, name);
    if (target === undefined) return `${name}: not changed on disk`;
    try {
      change(target);
    } catch (failure) {
      return `${name}: ${failure.message}`;
    }
    return undefined;
  };
  return {
    write: (name, bytes) => attempt(name, (target) => fs.writeFileSync(target, bytes)),
    erase: (name) => attempt(name, (target) => fs.rmSync(target, { force: true })),
  };
}
