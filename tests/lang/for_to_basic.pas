program ForToBasic;
var i, sum: Integer;
begin
  sum := 0;
  for i := 1 to 5 do
  begin
    Write(i, ' ');
    sum := sum + i
  end;
  WriteLn;
  WriteLn(sum);
  for i := -3 to 3 do Write(i:3);
  WriteLn;
  for i := 10 to 12 do WriteLn('line ', i)
end.
