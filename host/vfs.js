// File system behind the host imports (contract: docs/abi.md section 8). It is a snapshot held
// in memory: a program can create, rewrite and append files, and none of it reaches the source
// of the snapshot or the next run, unless the owner asks to hear about it: `onWritten(name,
// bytes)` is called when a file opened for writing is closed or renamed, and `onErased(name)`
// when a name stops existing (delete or rename). It works on Uint8Array, so it runs in a
// browser and in Node alike.

export const RESET = 0;
export const REWRITE = 1;
export const APPEND = 2;

// Negative results are DOS error numbers negated.
const NOT_FOUND = -2;
const PATH_NOT_FOUND = -3;
const TOO_MANY_OPEN = -4;
const ACCESS_DENIED = -5;
const BAD_HANDLE = -6;

const MAX_OPEN = 20;

// DOS 8.3: 1 to 8 name characters, then optionally a dot and up to 3 extension characters.
const NAME_CHARS = "[A-Z0-9_$~!#%&\\-{}()@'^]";
const DOS_NAME = new RegExp(`^${NAME_CHARS}{1,8}(\\.${NAME_CHARS}{0,3})?$`, "i");

// The error for a name that cannot be a file of this file system, or 0.
function nameError(name) {
  if (/[\\/:]/.test(name)) return PATH_NOT_FOUND;
  return DOS_NAME.test(name) ? 0 : NOT_FOUND;
}

function concat(a, b) {
  const joined = new Uint8Array(a.length + b.length);
  joined.set(a);
  joined.set(b, a.length);
  return joined;
}

export class Vfs {
  // `snapshot` maps file names to their bytes (a Map, or null for an empty file system).
  constructor(snapshot, onWritten = () => {}, onErased = () => {}) {
    this.files = new Map(); // upper-case name -> { name, data: Uint8Array }
    this.handles = new Map(); // handle -> { file, position, mode }
    this.onWritten = onWritten;
    this.onErased = onErased;
    for (const [name, data] of snapshot ?? []) {
      const key = name.toUpperCase();
      this.files.set(key, { name: key, data: new Uint8Array(data) });
    }
  }

  // Returns a handle (1 or more) or a negative error number.
  open(name, mode) {
    const error = nameError(name);
    if (error !== 0) return error;
    if (mode !== RESET && mode !== REWRITE && mode !== APPEND) return ACCESS_DENIED;
    if (this.handles.size >= MAX_OPEN) return TOO_MANY_OPEN;
    const key = name.toUpperCase();
    let file = this.files.get(key);
    if (mode !== REWRITE && file === undefined) return NOT_FOUND;
    if (file === undefined || mode === REWRITE) {
      file = { name: key, data: new Uint8Array(0) };
      this.files.set(key, file);
    }
    const position = mode === APPEND ? file.data.length : 0;
    let handle = 1;
    while (this.handles.has(handle)) handle++;
    this.handles.set(handle, { file, position, mode });
    return handle;
  }

  // Returns a Uint8Array of up to `length` bytes (empty at end of file) or a negative error
  // number.
  read(handle, length) {
    const open = this.handles.get(handle);
    if (open === undefined) return BAD_HANDLE;
    if (open.mode !== RESET) return ACCESS_DENIED;
    const chunk = open.file.data.slice(open.position, open.position + length);
    open.position += chunk.length;
    return chunk;
  }

  // Returns the number of bytes written or a negative error number.
  write(handle, bytes) {
    const open = this.handles.get(handle);
    if (open === undefined) return BAD_HANDLE;
    if (open.mode === RESET) return ACCESS_DENIED;
    open.file.data = concat(open.file.data, bytes);
    return bytes.length;
  }

  close(handle) {
    const open = this.handles.get(handle);
    if (open === undefined) return BAD_HANDLE;
    this.handles.delete(handle);
    // A file erased while open is no longer registered under its name: it is not saved again.
    const registered = this.files.get(open.file.name) === open.file;
    if (open.mode !== RESET && registered) this.onWritten(open.file.name, open.file.data.slice());
    return 0;
  }

  // Removes the name; handles already open keep reading and writing the unlinked file.
  // Returns 0 or a negative error number.
  delete(name) {
    const error = nameError(name);
    if (error !== 0) return error;
    const key = name.toUpperCase();
    if (!this.files.has(key)) return NOT_FOUND;
    this.files.delete(key);
    this.onErased(key);
    return 0;
  }

  // Gives the file `oldName` the name `newName`; open handles follow the file. The new name
  // must not exist (DOS says access denied) and, being a name in this directory, must not
  // contain a path. Returns 0 or a negative error number.
  rename(oldName, newName) {
    const error = nameError(oldName);
    if (error !== 0) return error;
    const file = this.files.get(oldName.toUpperCase());
    if (file === undefined) return NOT_FOUND;
    if (/:/.test(newName)) return PATH_NOT_FOUND;
    if (/[\\/]/.test(newName)) return ACCESS_DENIED;
    if (!DOS_NAME.test(newName)) return NOT_FOUND;
    const newKey = newName.toUpperCase();
    if (this.files.has(newKey)) return ACCESS_DENIED;
    const oldKey = file.name;
    this.files.delete(oldKey);
    file.name = newKey;
    this.files.set(newKey, file);
    this.onWritten(newKey, file.data.slice());
    this.onErased(oldKey);
    return 0;
  }
}
