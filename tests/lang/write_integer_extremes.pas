program WriteIntegerExtremes;
var
  b: Byte;
  s: ShortInt;
  i: Integer;
  w: Word;
  l: LongInt;
begin
  b := 255;
  s := -128;
  i := -32768;
  w := 65535;
  l := -2147483647 - 1;
  WriteLn(b);
  WriteLn(s);
  WriteLn(i);
  WriteLn(w);
  WriteLn(l);
  b := 0;
  s := 127;
  i := 32767;
  w := 0;
  l := 2147483647;
  WriteLn(b, ' ', s, ' ', i, ' ', w, ' ', l)
end.
