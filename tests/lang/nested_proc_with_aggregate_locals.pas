program NestedAggregates;
type V = array[1..3] of Integer;
procedure Outer(n: Integer);
var local: V; i: Integer;
  procedure Inner(k: Integer);
  begin
    local[k] := local[k] + n
  end;
begin
  for i := 1 to 3 do local[i] := i;
  Inner(2); Inner(2); Inner(3);
  WriteLn(local[1], local[2], local[3])
end;
begin
  Outer(10); Outer(1)
end.
