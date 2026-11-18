program NegTest;
var n: Integer;
procedure Inc1(var x: Integer);
begin
  x := x + 1
end;
begin
  n := 1;
  Inc1(n + 1)
end.
