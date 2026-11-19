// Draft of host/ide.js: the two imports behind the IDE host unit (decision 40). Added to
// `host.imports` only for the IDE program. Everything environment-specific is injected:
//   compiler    { compile(mainName, bytes, files) } (draft/compiler.js)
//   runModule   (moduleBytes, files) -> exit code: runs a compiled program to its end, on the
//               shared screen and keyboard, and returns how it ended (-1 when stopped)
// Record layout of CompileResult in IDE memory (Pascal, packed, little endian): Code: Integer
// (2 bytes), Line and Col: LongInt (4 bytes each: a 64 KB buffer of short lines passes 32767
// lines), FileName: string[12] (length byte, 12 characters).
const RECORD_BYTES = 23;
const NAME_AT = 10;

const latin1 = (bytes) => String.fromCharCode(...bytes);
const filesOf = (vfs) => new Map([...vfs.files].map(([name, file]) => [name, file.data]));

export function ideImports(host, { compiler, runModule }) {
  let module = null; // the last successful compile; the module never enters IDE memory
  return {
    // procedure HostCompile(Name: PString; Text: Pointer; Len: LongInt; Result: Pointer)
    ide_compile: (namePtr, textPtr, textLen, resultPtr) => {
      const nameLength = host.bytesAt("ide_compile", namePtr, 1)[0];
      const name = latin1(host.bytesAt("ide_compile", namePtr + 1, nameLength));
      const text = host.bytesAt("ide_compile", textPtr, textLen).slice();
      const outcome = compiler.compile(name, text, filesOf(host.vfs));
      module = outcome.module ?? null;
      const record = host.bytesAt("ide_compile", resultPtr, RECORD_BYTES);
      const view = new DataView(record.buffer, record.byteOffset, RECORD_BYTES);
      view.setInt16(0, outcome.code ?? 0, true);
      view.setInt32(2, outcome.line ?? 0, true);
      view.setInt32(6, outcome.col ?? 0, true);
      const file = outcome.file ?? "";
      record[NAME_AT] = file.length;
      record.set(Uint8Array.from(file, (c) => c.charCodeAt(0)), NAME_AT + 1);
    },
    // function HostRun: Integer
    ide_run: () => (module === null ? -2 : runModule(module, filesOf(host.vfs))),
  };
}
