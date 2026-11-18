program VarParamSignExtensionOnRead;
{ values read through a var parameter are sign-extended (ShortInt, Integer) or
  zero-extended (Byte, Word) to the width they are used at }
procedure Widen(var s: ShortInt; var i: Integer; var b: Byte; var w: Word);
var l: LongInt;
begin
  l := s; WriteLn(l);
  l := i; WriteLn(l);
  l := b; WriteLn(l);
  l := w; WriteLn(l);
  WriteLn(s + 1, ' ', i - 1, ' ', b + 1, ' ', w + 1)
end;
var s: ShortInt; i: Integer; b: Byte; w: Word;
begin
  s := -128; i := -32768; b := 255; w := 65535;
  Widen(s, i, b, w);
  s := -1; i := -1; b := 128; w := 32768;
  Widen(s, i, b, w);
  s := 127; i := 32767; b := 0; w := 0;
  Widen(s, i, b, w)
end.
