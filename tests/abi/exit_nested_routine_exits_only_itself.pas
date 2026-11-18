program ExitNestedRoutineExitsOnlyItself;
{ Exit in a nested routine returns to its caller, not to the enclosing routine }
procedure Outer;
var n: Integer;
  procedure Inner(k: Integer);
  begin
    n := n + 1;
    if k > 2 then Exit;
    n := n + 10
  end;
begin
  n := 0;
  Inner(1);
  Inner(5);
  WriteLn(n);
  if n > 0 then Exit;
  WriteLn('unreachable')
end;
begin
  Outer;
  WriteLn('after')
end.
