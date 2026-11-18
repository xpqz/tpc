program FunctionResultRecursionInExpression;
{ two recursive calls inside one expression: partial results must survive the second call }
function Fib(n: Integer): LongInt;
begin
  if n < 2 then Fib := n
  else Fib := Fib(n - 1) * 3 + Fib(n - 2) * 2
end;
function Bin(n, k: Integer): LongInt;
begin
  if (k = 0) or (k = n) then Bin := 1
  else Bin := Bin(n - 1, k - 1) + Bin(n - 1, k)
end;
begin
  WriteLn(Fib(10), ' ', Fib(15));
  WriteLn(Bin(10, 5), ' ', Bin(16, 8))
end.
