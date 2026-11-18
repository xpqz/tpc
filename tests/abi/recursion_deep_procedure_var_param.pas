program RecursionDeepProcedureVarParam;
{ depth 1000: a var parameter threaded through every level, local per level }
procedure Walk(n: Integer; var acc: LongInt);
var mine: Integer;
begin
  mine := n * 2;
  if n > 0 then Walk(n - 1, acc);
  acc := acc + mine
end;
var total: LongInt;
begin
  total := 0;
  Walk(1000, total);
  WriteLn(total);
  Walk(10, total);
  WriteLn(total)
end.
