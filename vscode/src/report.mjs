// Turns a compile result into the diagnostics to show: a list of
// { file, message, range } where `file` is the name of the file the error is in.
import { errorRange } from "./positions.mjs";

const FIRST_POSITION = { line: 0, start: 0, end: 0 };

// `linesOf(name)` returns the lines of that file's text, or undefined if it is unknown.
export function buildReport(result, mainName, linesOf) {
  if (result.ok) return [];
  if (result.internal !== undefined) {
    return [
      {
        file: mainName,
        message: `Internal compiler error: ${result.internal}`,
        range: FIRST_POSITION,
      },
    ];
  }
  const { code, line, col, file, message } = result.error;
  const lines = linesOf(file);
  return [
    {
      file,
      message: `Error ${code}: ${message}`,
      range: lines ? errorRange(lines, line, col) : FIRST_POSITION,
    },
  ];
}
