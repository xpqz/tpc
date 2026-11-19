// Node adapters for the host file system and console input (host/vfs.js): they read an aux
// directory and the process stdin, and hand results back as Buffers.
import fs from "node:fs";
import path from "node:path";
import { Vfs as HostVfs } from "../host/vfs.js";
import { LF, CR } from "../host/terminal.js";

export { RESET, REWRITE, APPEND } from "../host/vfs.js";

// The regular files of `auxDir` as a name -> bytes Map; empty when `auxDir` is null.
export function loadSnapshot(auxDir) {
  const files = new Map();
  if (auxDir === null) return files;
  for (const entry of fs.readdirSync(auxDir, { withFileTypes: true })) {
    if (entry.isFile()) files.set(entry.name, fs.readFileSync(path.join(auxDir, entry.name)));
  }
  return files;
}

export class Vfs extends HostVfs {
  constructor(auxDir) {
    super(loadSnapshot(auxDir));
  }

  read(handle, length) {
    const chunk = super.read(handle, length);
    return typeof chunk === "number" ? chunk : Buffer.from(chunk);
  }
}

// Console input from `buffer`, or from the process stdin (read on first use) when null.
export class Input {
  constructor(buffer) {
    this.buffer = buffer;
    this.lines = null;
  }

  readLine(max) {
    this.buffer ??= readStdin();
    this.lines ??= new LineInput(this.buffer);
    const line = this.lines.readLine(max);
    return line === null ? null : Buffer.from(line);
  }
}

// Splits console input held as bytes into lines on demand.
class LineInput {
  constructor(bytes) {
    this.bytes = bytes;
    this.position = 0;
  }

  // The next line without its terminator (LF or CRLF), cut to `max` bytes; null at the end.
  readLine(max) {
    const text = this.bytes;
    if (this.position >= text.length) return null;
    let end = text.indexOf(LF, this.position);
    const next = end === -1 ? text.length : end + 1;
    if (end === -1) end = text.length;
    if (end > this.position && text[end - 1] === CR && next > end) end--;
    const line = text.slice(this.position, Math.min(end, this.position + max));
    this.position = next;
    return line;
  }
}

function readStdin() {
  try {
    return fs.readFileSync(0);
  } catch {
    return Buffer.alloc(0);
  }
}
