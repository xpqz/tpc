program ForNested;
var i, j: Integer;
begin
  for i := 1 to 4 do
  begin
    for j := 1 to i do Write('*');
    WriteLn
  end;
  for i := 3 downto 1 do
  begin
    for j := i to 3 do Write(j);
    WriteLn
  end
end.
