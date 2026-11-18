program RecursionFibonacci;
var n: Integer;
function Fib(n: Integer): Integer;
begin
  if n < 2 then Fib := n
  else Fib := Fib(n - 1) + Fib(n - 2)
end;
begin
  for n := 0 to 20 do Write(Fib(n), ' ');
  WriteLn
end.
