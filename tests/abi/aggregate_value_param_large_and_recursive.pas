program AggregateRecursive;
type Big = array[1..100] of Integer;
function SumTo(b: Big; n: Integer): Integer;
begin
  if n = 0 then SumTo := 0
  else begin
    b[n] := b[n] * 2;
    SumTo := b[n] + SumTo(b, n - 1)
  end
end;
var b: Big; i: Integer;
begin
  for i := 1 to 100 do b[i] := i;
  WriteLn(SumTo(b, 40));
  WriteLn(b[1], ' ', b[40], ' ', b[100])
end.
