program NegTest;
var w: Word;
procedure Inc1(var x: Integer);
begin
  x := x + 1
end;
begin
  w := 1;
  Inc1(w)
end.
