program ForDowntoBasic;
var i: Integer;
begin
  for i := 5 downto 1 do Write(i, ' ');
  WriteLn;
  for i := 3 downto -3 do Write(i:3);
  WriteLn;
  for i := 10 downto 8 do WriteLn('line ', i)
end.
