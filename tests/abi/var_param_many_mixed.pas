program VarParamManyMixed;
{ eight parameters: value and var interleaved, different types }
procedure Many(a: Integer; var b: Integer; c: Byte; var d: LongInt; e: Boolean; var f: Char; g: LongInt; var h: Word);
begin
  b := a + b;
  d := d + g + c;
  if e then f := 'T' else f := 'F';
  h := h + 1;
  a := 0; c := 0; g := 0
end;
var b: Integer; d: LongInt; f: Char; h: Word;
begin
  b := 1; d := 100000; f := '?'; h := 65535;
  Many(2, b, 3, d, True, f, 4, h);
  WriteLn(b, ' ', d, ' ', f, ' ', h);
  Many(b, b, 255, d, False, f, d, h);
  WriteLn(b, ' ', d, ' ', f, ' ', h)
end.
