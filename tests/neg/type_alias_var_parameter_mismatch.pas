program NegTest;
type Num = Integer;
var b: Byte;
procedure Inc1(var x: Num);
begin
  x := x + 1
end;
begin
  Inc1(b)
end.
