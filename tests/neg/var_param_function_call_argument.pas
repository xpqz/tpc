program NegTest;
function Five: Integer;
begin
  Five := 5
end;
procedure Inc1(var x: Integer);
begin
  x := x + 1
end;
begin
  Inc1(Five)
end.
