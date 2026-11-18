program AggregateLocals;
type Vec = array[1..4] of Integer;
procedure Fill(k: Integer);
var v: Vec; i: Integer;
begin
  for i := 1 to 4 do v[i] := k;
  WriteLn(v[1] + v[4])
end;
procedure Recur(n: Integer);
var v: Vec;
begin
  v[1] := n; v[4] := n * 2;
  if n > 0 then Recur(n - 1);
  WriteLn(n, ' ', v[1], ' ', v[4])
end;
begin
  Fill(3); Fill(40);
  Recur(3)
end.
