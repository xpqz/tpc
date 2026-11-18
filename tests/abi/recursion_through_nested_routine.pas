program RecursionThroughNestedRoutine;
{ a nested function that recurses on itself and reads and writes an outer local }
procedure Outer(n: Integer);
var depthSeen, bumps: Integer;
  function Down(k: Integer): Integer;
  begin
    bumps := bumps + 1;
    if k > depthSeen then depthSeen := k;
    if k = 0 then Down := 0
    else Down := k + Down(k - 1)
  end;
begin
  depthSeen := 0;
  bumps := 0;
  WriteLn(Down(n), ' ', depthSeen, ' ', bumps)
end;
begin
  Outer(0);
  Outer(10);
  Outer(500)
end.
