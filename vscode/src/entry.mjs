// A unit cannot be the main file of a compile (the compiler expects BEGIN after it), so a unit
// is checked through a one-line program that uses it.
const UNIT_HEADER = /^\s*unit\s+([A-Za-z_][A-Za-z0-9_]*)/i;
const UNIT_CHECK_NAME = "Tp5UnitCheck.pas";

// Returns { mainName, mainText } for the document `text` saved as `fileName`.
export function compileEntry(fileName, text) {
  const unit = text.replace(/\{[^}]*\}|\(\*[\s\S]*?\*\)/g, " ").match(UNIT_HEADER);
  if (!unit) return { mainName: fileName, mainText: text };
  return { mainName: UNIT_CHECK_NAME, mainText: `program Tp5UnitCheck; uses ${unit[1]}; begin end.` };
}
