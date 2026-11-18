program NegTest;
var b: Byte;
procedure Inc1(var x: Integer);
begin
  x := x + 1
end;
begin
  b := 1;
  Inc1(b)
end.
