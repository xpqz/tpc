program ProcVarRecursionThroughVariable;
{$F+}
{ a routine that calls itself through a procedural variable that holds it; the values of
  both calls are combined }
type TF = function(n: Integer): LongInt;
var fn: TF;
function Fib(n: Integer): LongInt;
begin
  if n < 2 then Fib := n else Fib := fn(n - 1) + fn(n - 2)
end;
var i: Integer;
begin
  fn := Fib;
  for i := 0 to 15 do Write(fn(i), ' ');
  WriteLn
end.
