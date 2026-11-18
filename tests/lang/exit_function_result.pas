program ExitFunctionResult;
{ Exit in a function returns the value assigned so far }
function Sign(n: Integer): Integer;
begin
  Sign := 0;
  if n = 0 then Exit;
  Sign := 1;
  if n > 0 then Exit;
  Sign := -1
end;
begin
  WriteLn(Sign(-9), ' ', Sign(0), ' ', Sign(9))
end.
