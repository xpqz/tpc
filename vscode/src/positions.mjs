// Maps a compiler error position to a range in the text of its file.

export function splitLines(text) {
  return text.split(/\r\n|\n|\r/);
}

const isIdentifierChar = (char) => /[A-Za-z0-9_]/.test(char);

// `line` and `col` are the compiler's 1-based line and byte column. Because text is sent one
// byte per UTF-16 unit, `col - 1` is the unit offset in the line. Line 0 is what an error in
// an embedded runtime unit reports; it has no place in the file, so it maps to the first position.
// The range is zero-based: the identifier at the column, else one character, clamped to the line.
export function errorRange(lines, line, col) {
  if (line === 0) return { line: 0, start: 0, end: 0 };
  const lineIndex = Math.min(line, lines.length) - 1;
  const text = lines[lineIndex];
  if (text.length === 0) return { line: lineIndex, start: 0, end: 0 };
  const start = Math.min(col - 1, text.length - 1);
  let end = start + 1;
  if (isIdentifierChar(text[start])) {
    while (end < text.length && isIdentifierChar(text[end])) end++;
  }
  return { line: lineIndex, start, end };
}
