program ForTypeLimitsTo;
var
  b: Byte;
  s: ShortInt;
  i: Integer;
  w: Word;
  l: LongInt;
  n: Integer;
begin
  n := 0;
  for b := 253 to 255 do n := n + 1;
  WriteLn(n);
  for b := 253 to 255 do Write(b, ' ');
  WriteLn;
  for s := 125 to 127 do Write(s, ' ');
  WriteLn;
  for i := 32765 to 32767 do Write(i, ' ');
  WriteLn;
  for w := 65533 to 65535 do Write(w, ' ');
  WriteLn;
  for l := 2147483645 to 2147483647 do Write(l, ' ');
  WriteLn
end.
