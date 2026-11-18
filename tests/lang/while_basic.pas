program WhileBasic;
var i, sum: Integer;
begin
  i := 1;
  sum := 0;
  while i <= 10 do
  begin
    sum := sum + i;
    i := i + 1
  end;
  WriteLn(sum);
  i := 5;
  while i > 0 do
  begin
    Write(i, ' ');
    i := i - 1
  end;
  WriteLn('liftoff')
end.
