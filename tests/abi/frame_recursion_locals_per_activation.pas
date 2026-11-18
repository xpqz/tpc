program FrameRecursionLocalsPerActivation;
{ each activation has its own frame: a local passed as a var argument keeps its value across
  the recursive calls that follow, and the results of both recursive calls are combined }
procedure Add(var acc: LongInt; v: LongInt); begin acc := acc + v end;
function Fib(n: Integer): LongInt;
var left, right: LongInt;
begin
  if n < 2 then begin Fib := n; Exit end;
  left := 0; right := 0;
  Add(left, Fib(n - 1));
  Add(right, Fib(n - 2));
  Add(left, right);
  Fib := left
end;
procedure Count(var c: Integer); begin c := c + 1 end;
function Depth(n: Integer; var seen: Integer): Integer;
var mine: Integer;
begin
  mine := n * 10;
  Count(seen);
  if n = 0 then Depth := 0 else Depth := Depth(n - 1, seen) + mine
end;
var s: Integer; i: Integer;
begin
  for i := 0 to 15 do Write(Fib(i), ' ');
  WriteLn;
  s := 0;
  WriteLn(Depth(20, s), ' ', s)
end.
