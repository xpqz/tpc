program ForDowntoBoundaries;
var
  i: Integer;
  w: Word;
  b: Byte;
begin
  for i := 3 downto 4 do WriteLn('never ', i);
  for i := 0 downto 1 do WriteLn('never ', i);
  for i := 2 downto 0 do Write(i, ' ');
  WriteLn;
  for w := 2 downto 0 do Write(w, ' ');
  WriteLn;
  for b := 2 downto 0 do Write(b, ' ');
  WriteLn;
  for i := -1 downto -3 do Write(i, ' ');
  WriteLn;
  for i := 0 downto 0 do Write('zero');
  WriteLn
end.
