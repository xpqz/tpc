program NegTest;
procedure Inc1(var x: Integer);
begin
  x := x + 1
end;
function F: Integer;
begin
  Inc1(F);
  F := 1
end;
begin
end.
