program RecursionFactorial;
var n: Integer;
function Fact(n: Integer): LongInt;
begin
  if n <= 1 then Fact := 1
  else Fact := n * Fact(n - 1)
end;
begin
  for n := 0 to 12 do WriteLn(n:2, '! = ', Fact(n))
end.
