// File system and console input for the test runner (contract: docs/abi.md section 8).
// The file system is a snapshot of an aux directory held in memory: a program can create,
// rewrite and append files, and none of it reaches the directory or the next run.
import fs from "node:fs";
import path from "node:path";

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

export class Vfs {
  // `auxDir` is a directory path, or null for an empty file system.
  constructor(auxDir) {
    this.files = new Map(); // upper-case name -> { data: Buffer }
    this.handles = new Map(); // handle -> { file, position, mode }
    if (auxDir === null) return;
    for (const entry of fs.readdirSync(auxDir, { withFileTypes: true })) {
      if (!entry.isFile()) continue;
      const data = fs.readFileSync(path.join(auxDir, entry.name));
      this.files.set(entry.name.toUpperCase(), { data });
    }
  }

  // Returns a handle (1 or more) or a negative error number.
  open(name, mode) {
    if (/[\\/:]/.test(name)) return PATH_NOT_FOUND;
    if (!DOS_NAME.test(name)) return NOT_FOUND;
    if (mode !== RESET && mode !== REWRITE && mode !== APPEND) return ACCESS_DENIED;
    if (this.handles.size >= MAX_OPEN) return TOO_MANY_OPEN;
    const key = name.toUpperCase();
    let file = this.files.get(key);
    if (mode === RESET && file === undefined) return NOT_FOUND;
    if (file === undefined || mode === REWRITE) {
      file = { data: Buffer.alloc(0) };
      this.files.set(key, file);
    }
    const position = mode === APPEND ? file.data.length : 0;
    let handle = 1;
    while (this.handles.has(handle)) handle++;
    this.handles.set(handle, { file, position, mode });
    return handle;
  }

  // Returns a Buffer of up to `length` bytes (empty at end of file) or a negative error number.
  read(handle, length) {
    const open = this.handles.get(handle);
    if (open === undefined) return BAD_HANDLE;
    if (open.mode !== RESET) return ACCESS_DENIED;
    const chunk = open.file.data.subarray(open.position, open.position + length);
    open.position += chunk.length;
    return Buffer.from(chunk);
  }

  // Returns the number of bytes written or a negative error number.
  write(handle, bytes) {
    const open = this.handles.get(handle);
    if (open === undefined) return BAD_HANDLE;
    if (open.mode === RESET) return ACCESS_DENIED;
    open.file.data = Buffer.concat([open.file.data, bytes]);
    return bytes.length;
  }

  close(handle) {
    return this.handles.delete(handle) ? 0 : BAD_HANDLE;
  }
}

// Console input split into lines. A null buffer means the process stdin, read on first use.
export class Input {
  constructor(buffer) {
    this.buffer = buffer;
    this.position = 0;
  }

  // The next line without its terminator (LF or CRLF), cut to `max` bytes; null at the end.
  readLine(max) {
    if (this.buffer === null) this.buffer = readStdin();
    const text = this.buffer;
    if (this.position >= text.length) return null;
    let end = text.indexOf(0x0a, this.position);
    const next = end === -1 ? text.length : end + 1;
    if (end === -1) end = text.length;
    if (end > this.position && text[end - 1] === 0x0d && next > end) end--;
    const line = text.subarray(this.position, Math.min(end, this.position + max));
    this.position = next;
    return Buffer.from(line);
  }
}

function readStdin() {
  try {
    return fs.readFileSync(0);
  } catch {
    return Buffer.alloc(0);
  }
}
