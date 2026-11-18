program NegTest;
var n: Integer; b: Boolean;
function G(a: Integer): Integer;
begin
  G := a
end;
function F: Integer;
begin
  F := 1
end;
procedure Inc1(var x: Integer);
begin
  x := x + 1
end;
begin
  Inc1(-n)
end.
