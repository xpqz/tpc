program ArrayLocalInRecursion;
type Row = array[1..100] of Integer;
function Fill(depth, base: Integer): Integer;
var local: Row; i: Integer; below: Integer;
begin
  for i := 1 to 100 do local[i] := base + depth * 1000 + i;
  if depth > 0 then below := Fill(depth - 1, base + 1) else below := 0;
  Fill := local[1] + local[100] + below - base
end;
begin
  WriteLn(Fill(0, 5));
  WriteLn(Fill(3, 5));
  WriteLn(Fill(40, 1))
end.
