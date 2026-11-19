program TypedconstScalarsInitialValues;
type
  TColor = (Red, Green, Blue);
  TSmall = 1..9;
const
  i: Integer = -1234;
  w: Word = 65535;
  b: Byte = 255;
  sh: ShortInt = -128;
  l: LongInt = 2147483647;
  c: Char = 'Z';
  flag: Boolean = True;
  col: TColor = Blue;
  sm: TSmall = 9;
  r: Real = 2.5;
  hex: Integer = $7FFF;
begin
  WriteLn(i, ' ', w, ' ', b, ' ', sh, ' ', l);
  WriteLn(c, ' ', flag, ' ', Ord(col), ' ', sm);
  WriteLn(r:0:2, ' ', hex);
end.
