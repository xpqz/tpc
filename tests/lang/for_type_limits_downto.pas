program ForTypeLimitsDownTo;
var
  b: Byte;
  s: ShortInt;
  i: Integer;
  w: Word;
  l, m: LongInt;
begin
  for b := 2 downto 0 do Write(b, ' ');
  WriteLn;
  for s := -126 downto -128 do Write(s, ' ');
  WriteLn;
  for i := -32766 downto -32768 do Write(i, ' ');
  WriteLn;
  for w := 1 downto 0 do Write(w, ' ');
  WriteLn;
  m := -2147483647;
  m := m - 1;
  for l := -2147483646 downto m do Write(l, ' ');
  WriteLn
end.
