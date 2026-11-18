program MultiplicationTable;
var i, j: Integer;
begin
  Write('   |');
  for j := 1 to 9 do Write(j:4);
  WriteLn;
  for i := 1 to 9 do
  begin
    Write(i:2, ' |');
    for j := 1 to 9 do Write(i * j:4);
    WriteLn
  end
end.
