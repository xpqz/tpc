program ValueParamPassedAsVarInside;
{ a value parameter whose address is taken inside the callee: the caller's variable is not
  changed }
procedure Bump(var x: Integer);
begin
  x := x + 1
end;
procedure Callee(a: Integer; b: Byte);
begin
  Bump(a);
  Bump(a);
  WriteLn(a, ' ', b)
end;
var x: Integer; y: Byte;
begin
  x := 40; y := 7;
  Callee(x, y);
  WriteLn(x, ' ', y);
  Callee(x + 1, 200)
end.
