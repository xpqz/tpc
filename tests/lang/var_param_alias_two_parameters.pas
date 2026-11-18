program VarParamAliasTwoParameters;
{ two var parameters bound to one variable are aliases }
procedure P(var a, b: Integer);
begin
  a := 10;
  b := b + 1;
  WriteLn(a, ' ', b)
end;
var x: Integer;
begin
  x := 0;
  P(x, x);
  WriteLn(x)
end.
