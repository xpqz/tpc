program NegTest;
var i: Integer;
procedure Inc1(var x: LongInt);
begin
  x := x + 1
end;
begin
  i := 1;
  Inc1(i)
end.
