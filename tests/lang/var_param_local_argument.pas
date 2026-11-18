program VarParamLocalArgument;
{ a local variable and a value parameter of the caller passed by reference }
procedure Add10(var x: Integer);
begin
  x := x + 10
end;
function Calc(n: Integer): Integer;
var t: Integer;
begin
  t := n;
  Add10(t);
  Add10(n);
  Calc := t * 100 + n
end;
begin
  WriteLn(Calc(1));
  WriteLn(Calc(5))
end.
