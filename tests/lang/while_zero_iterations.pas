program WhileZeroIterations;
var i: Integer;
begin
  i := 10;
  while i < 5 do
  begin
    WriteLn('never');
    i := i + 1
  end;
  while False do WriteLn('never');
  WriteLn(i)
end.
