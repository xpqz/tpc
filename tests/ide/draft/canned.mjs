// The canned compiler and run step of the IDE oracle tests: the same rules as the Pascal
// stand-in unit IDEHOST.PAS that the real Turbo Pascal 5.0 compiles (see
// tests/lang/crt_ide_oracle_pattern_editor_unit_with_host_stub/IDEHOST.PAS):
//   compile fails with error 3 at the first line containing the word ERROR, at the column of
//   its first letter, in the file named by the caller; any other text compiles;
//   run returns the number of lines of the last compiled text, modulo 256, and draws nothing.
// With these injected in place of the real compiler, an IDE program run by the Node runner
// must print the screen dump the real TP 5.0 prints for the same keys.
const MARKER = "ERROR";

export function cannedCompiler() {
  return {
    compile(mainName, text) {
      const lines = Buffer.from(text).toString("latin1").split("\n");
      const at = lines.findIndex((line) => line.includes(MARKER));
      if (at >= 0) {
        return { code: 3, line: at + 1, col: lines[at].indexOf(MARKER) + 1, file: mainName };
      }
      return { module: Uint8Array.of(lines.length - 1) }; // a module is only a line count here
    },
  };
}

export const cannedRun = (module) => module[0] % 256;
