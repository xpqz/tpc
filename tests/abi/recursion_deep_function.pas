program RecursionDeepFunction;
{ depth 1500 through a function: every frame must be popped on return }
function Sum(n: Integer): LongInt;
begin
  if n = 0 then Sum := 0
  else Sum := n + Sum(n - 1)
end;
var i: Integer;
begin
  WriteLn(Sum(1500));
  for i := 1 to 3 do WriteLn(Sum(i * 400))
end.
