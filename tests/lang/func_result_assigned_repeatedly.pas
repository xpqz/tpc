program FuncResultAssignedRepeatedly;
function Count(n: Integer): Integer;
var i, s: Integer;
begin
  Count := 0;
  s := 0;
  for i := 1 to n do
  begin
    s := s + i;
    Count := s
  end;
  if n > 100 then Count := -1
end;
begin
  WriteLn(Count(0), ' ', Count(4), ' ', Count(100), ' ', Count(101))
end.
