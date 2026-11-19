// TP-style Real formatting (docs/deviations.md D1, D14, D15). `width` and `decimals` are -1
// when absent. Without decimals the number is scientific with width - 7 fraction digits (1 to
// 10, default 10) and a sign column; with decimals it is fixed. Ties round away from zero on
// the exact binary value.
export function formatReal(x, width, decimals) {
  const neg = x < 0;
  const a = Math.abs(x);
  const w = Math.min(Math.max(width, 0), 255);
  let s;
  if (decimals >= 0) {
    const d = Math.min(decimals, 100);
    const digits = a < 1e21 ? a.toFixed(d) : BigInt(a) + (d ? "." + "0".repeat(d) : "");
    s = (neg ? "-" : "") + digits;
  } else {
    const fraction = width < 0 ? 10 : Math.min(10, Math.max(1, width - 7));
    const [mantissa, exponent] = a.toExponential(fraction).split("e");
    const e = Number(exponent);
    const exp = String(Math.abs(e)).padStart(2, "0");
    s = (neg ? "-" : " ") + mantissa + "E" + (e < 0 ? "-" : "+") + exp;
  }
  return s.padStart(w);
}

// The host import: writes the text at `ptr` and returns its length.
export function fmtReal(memory, x, width, decimals, ptr) {
  const text = formatReal(x, width, decimals);
  for (let i = 0; i < text.length; i++) new Uint8Array(memory.buffer)[ptr + i] = text.charCodeAt(i);
  return text.length;
}
