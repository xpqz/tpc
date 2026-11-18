program ForAccumulate;
var
  i: Integer;
  f, sum: LongInt;
begin
  f := 1;
  for i := 1 to 12 do f := f * i;
  WriteLn(f);
  sum := 0;
  for i := 1 to 180 do sum := sum + i * i div 3;
  WriteLn(sum);
  sum := 0;
  for i := 100 downto 1 do sum := sum + i;
  WriteLn(sum)
end.
