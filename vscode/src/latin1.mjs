// TP sources are 8-bit. The compiler reads one byte per source character, and an error column
// counts bytes, so text is exchanged with it one byte per UTF-16 unit.

// A unit above 255 becomes "?". This keeps every column equal to the unit offset in the
// editor's text, which is what error positions are mapped back to.
export function encodeLatin1(text) {
  const bytes = new Uint8Array(text.length);
  for (let i = 0; i < text.length; i++) {
    const unit = text.charCodeAt(i);
    bytes[i] = unit <= 0xff ? unit : "?".charCodeAt(0);
  }
  return bytes;
}

export function decodeLatin1(bytes) {
  let text = "";
  for (const byte of bytes) text += String.fromCharCode(byte);
  return text;
}
