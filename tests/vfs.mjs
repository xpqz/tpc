// Node adapters for the host file system and console input (host/vfs.js): they read an aux
// directory and the process stdin, and hand results back as Buffers.
import fs from "node:fs";
import path from "node:path";
import { Vfs as HostVfs, LineInput } from "../host/vfs.js";

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

function readStdin() {
  try {
    return fs.readFileSync(0);
  } catch {
    return Buffer.alloc(0);
  }
}
